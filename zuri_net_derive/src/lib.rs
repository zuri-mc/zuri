extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::{HashMap, HashSet};
use syn::{Data, DeriveInput, Fields, FieldsNamed, parse_macro_input, PathArguments, Type};
use quote::{format_ident, quote, TokenStreamExt, ToTokens};

#[proc_macro_attribute]
pub fn packet(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(_item as DeriveInput);
    let ident = input.ident.clone();

    let mut error_stream = proc_macro2::TokenStream::new();

    let mut write_stream = proc_macro2::TokenStream::new();
    let mut read_stream = proc_macro2::TokenStream::new();
    let mut read_inner_stream = proc_macro2::TokenStream::new();

    let mut vector_size_map = HashSet::new();

    if let Data::Struct(struct_data) = &mut input.data {
        let named_fields: &mut FieldsNamed = match &mut struct_data.fields {
            Fields::Named(f) => f,
            _ => unreachable!(),
        };

        let mut removal_queue = Vec::<usize>::new();
        'field_loop: for (i, field) in named_fields.named.iter_mut().enumerate() {
            let field_ident = field.ident.as_ref().unwrap();
            let field_type = &field.ty;

            let mut vec_type = None;
            let mut type_index = None;
            for (attr_i, attr) in field.attrs.iter().enumerate() {
                let path = attr.path.to_token_stream().to_string();
                if path == "size_for" {
                    // The vec name with delimiters
                    let group = syn::parse2::<proc_macro2::Group>(attr.tokens.clone()).unwrap();
                    let vec_name = syn::parse2::<proc_macro2::Ident>(group.stream()).unwrap();

                    let len_var_name = format_ident!("_{}_len", vec_name);
                    write_stream.append_all(quote!(<#field_type>::try_from(self.#vec_name.len()).unwrap().write(writer);));
                    read_stream.append_all(quote!(let #len_var_name = usize::try_from(<#field_type>::read(reader)).unwrap();));
                    vector_size_map.insert(vec_name.to_string());

                    removal_queue.push(i);
                    continue 'field_loop;
                }
                if path == "size_type" {
                    if vector_size_map.contains(field_ident.to_string().as_str()) {
                        let err = format!("Cannot combine `size_type` specifier with `size_for` for the same vector `{}`", field_ident.to_string());
                        error_stream.append_all(quote!(compile_error!(#err);));
                        continue 'field_loop;
                    }
                    if !type_index.is_none() {
                        let err = format!("Found more than one `size_type` specifier for vector `{}`", field_ident.to_string());
                        error_stream.append_all(quote!(compile_error!(#err);));
                    }
                    let group = syn::parse2::<proc_macro2::Group>(attr.tokens.clone()).unwrap();
                    let type_name = syn::parse2::<proc_macro2::Ident>(group.stream()).unwrap();

                    vec_type = Some(type_name);
                    type_index = Some(attr_i);
                }
            }
            if let Some(index) = type_index {
                field.attrs.remove(index);
            }

            if let Type::Path(path) = field_type {
                let last = path.path.segments.last().unwrap();
                if last.ident.to_string() == "Vec" {
                    let len_var_name = format_ident!("_{}_len", field_ident);

                    if !vector_size_map.contains(field_ident.to_string().as_str()) {
                        if vec_type.is_none() {
                            let err = format!("Missing size type for vector `{}`", field_ident.to_string());
                            error_stream.append_all(quote!(compile_error!(#err);));

                            continue 'field_loop;
                        }
                        let t = vec_type.unwrap();
                        write_stream.append_all(quote!((#t::try_from(self.#field_ident.len()).unwrap()).write(writer);));
                        read_stream.append_all(quote!(let #len_var_name = usize::try_from(#t::read(reader)).unwrap();));
                    }

                    if let PathArguments::AngleBracketed(generic_type) = &last.arguments {
                        write_stream.append_all(quote! {
                            for elem in &self.#field_ident {
                                elem.write(writer);
                            }
                        });
                        let inner_type = generic_type.args.first().unwrap();
                        read_stream.append_all(quote! {
                            let #field_ident = (0..#len_var_name).into_iter().map(|_| #inner_type::read(reader)).collect();
                        });
                        read_inner_stream.append_all(quote!(#field_ident,));
                    }
                    continue 'field_loop;
                }
            }

            write_stream.append_all(quote!(self.#field_ident.write(writer);));
            read_stream.append_all(quote!(let #field_ident = <#field_type>::read(reader);));
            read_inner_stream.append_all(quote!(#field_ident,));
        }
        let mut xi = 0usize;
        named_fields.named = named_fields.named.clone().into_pairs().filter(|p| {
            let remove = !removal_queue.contains(&xi);
            xi += 1;
            remove
        }).collect();
    }

    if !error_stream.is_empty() {
        return error_stream.into();
    }

    let tok = quote! {
        #input

        impl crate::proto::io::Writable for #ident {
            fn write(&self, writer: &mut crate::proto::io::Writer) {
                #write_stream
            }
        }

        // hi future andreas you can do reading by first just putting the fields as variables
        impl crate::proto::io::Readable<#ident> for #ident {
            fn read(reader: &mut crate::proto::io::Reader) -> #ident {
                #read_stream
                Self {#read_inner_stream}
            }
        }
    };
    tok.into()
}
