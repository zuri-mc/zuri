extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::HashSet;
use lazy_static::lazy_static;
use syn::{Data, DeriveInput, Expr, Fields, FieldsNamed, Lit, parse_macro_input, PathArguments, Type};
use quote::{format_ident, quote, quote_spanned, TokenStreamExt, ToTokens};
use regex::Regex;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn packet(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(_item as DeriveInput);
    let ident = input.ident.clone();

    // Write all errors to a separate token stream. If we have fully executed the macro, and notice
    // something went wrong, we use this as output to show all the errors that occurred while
    // compiling.
    let mut error_stream = proc_macro2::TokenStream::new();

    let mut write_stream = proc_macro2::TokenStream::new();
    let mut read_stream = proc_macro2::TokenStream::new();

    let mut extra_stream = proc_macro2::TokenStream::new();

    match &mut input.data {
        Data::Struct(struct_data) => {
            let mut read_body_stream = proc_macro2::TokenStream::new();
            let mut read_inner_stream = proc_macro2::TokenStream::new();

            let named_fields: &mut FieldsNamed = match &mut struct_data.fields {
                // In the 'normal' case, a struct will have named fields inside curly brackets `{}`.
                // This is the main path of execution for this function.
                Fields::Named(f) => f,
                Fields::Unnamed(f) => return quote_spanned!(f.span()=> compile_error!("Tuple structs are not supported");).into(),
                // Unit structs do not have fields, so the read and write methods do not have to do
                // anything.
                Fields::Unit => return quote! {
                    #input

                    impl crate::proto::io::Writable for #ident {
                        #[inline]
                        fn write(&self, writer: &mut crate::proto::io::Writer) {}
                    }

                    impl crate::proto::io::Readable<#ident> for #ident {
                        #[inline]
                        fn read(reader: &mut crate::proto::io::Reader) -> #ident { Self }
                    }
                }.into(),
            };

            // Keep track of which vectors already had their size written previously (due to a
            // size_for attribute).
            let mut vector_size_map = HashSet::new();

            // Keep track of which fields need to be removed from the output struct. Currently, this
            // is every field that has a size_for attribute.
            let mut removal_queue = Vec::<usize>::new();

            'field_loop: for (field_i, field) in named_fields.named.iter_mut().enumerate() {
                let field_ident = field.ident.as_ref().unwrap();
                let field_type = &field.ty;

                // If this is Some, this indicates that the current field has a `size_type`
                // attribute. The second value inside the option will then contain the argument
                // provided in the attribute. The first value is a span of the entire attribute,
                // which is used to show an error at a certain location.
                let mut vec_type = None;
                let mut enum_type = None;
                let mut attr_remove_queue = Vec::new();
                for (attr_i, attr) in field.attrs.iter().enumerate() {
                    let path = attr.path.to_token_stream().to_string();
                    if path == "size_for" {
                        // The vec name with delimiters `(` and `)`
                        // todo: error handling
                        let group = syn::parse2::<proc_macro2::Group>(attr.tokens.clone()).unwrap();
                        let vec_name = syn::parse2::<proc_macro2::Ident>(group.stream()).unwrap();

                        let len_var_name = format_ident!("_{}_len", vec_name);
                        if vector_size_map.contains(vec_name.to_string().as_str()) {
                            let err = format!("duplicate `size_for` for vector `{}`", vec_name);
                            error_stream.append_all(quote_spanned!(vec_name.span()=> compile_error!(#err);));
                        }

                        write_stream.append_all(quote!(<#field_type>::try_from(self.#vec_name.len()).unwrap().write(writer);));
                        read_body_stream.append_all(quote!(let #len_var_name = usize::try_from(<#field_type>::read(reader)).unwrap();));
                        vector_size_map.insert(vec_name.to_string());

                        removal_queue.push(field_i);
                        continue 'field_loop;
                    }
                    if path == "size_type" {
                        if vector_size_map.contains(field_ident.to_string().as_str()) {
                            let err = format!("Cannot combine `size_type` specifier with `size_for` for the same vector `{}`", field_ident.to_string());
                            error_stream.append_all(quote_spanned!(attr.span()=> compile_error!(#err);));
                            continue 'field_loop;
                        }
                        if !attr_remove_queue.is_empty() {
                            let err = format!("Found more than one `size_type` specifier for vector `{}`", field_ident.to_string());
                            error_stream.append_all(quote_spanned!(attr.span()=> compile_error!(#err);));
                        }
                        // todo: error handling
                        let group = syn::parse2::<proc_macro2::Group>(attr.tokens.clone()).unwrap();
                        let type_name = syn::parse2::<proc_macro2::Ident>(group.stream()).unwrap();

                        vec_type = Some((attr.span(), type_name));
                        attr_remove_queue.push(attr_i);
                    }
                    if path == "enum_header" {
                        // todo: error handling
                        let group = syn::parse2::<proc_macro2::Group>(attr.tokens.clone()).unwrap();
                        let type_name = syn::parse2::<proc_macro2::Ident>(group.stream()).unwrap();

                        enum_type = Some((attr.span(), type_name));
                        attr_remove_queue.push(attr_i);
                    }
                }
                // Remove all the attributes that should be removed. First we make sure that all
                // the indices are sorted, so we can remove them all in reverse without changing the
                // indices of the next elements that we want to remove.
                attr_remove_queue.sort();
                for to_remove in attr_remove_queue.iter().rev() {
                    field.attrs.remove(*to_remove);
                }

                if let Type::Path(path) = field_type {
                    let last = path.path.segments.last().unwrap();
                    if last.ident.to_string() == "Vec" {
                        let len_var_name = format_ident!("_{}_len", field_ident);

                        // In this case, the vector's length has not yet been written, so we should
                        // do it here.
                        if !vector_size_map.contains(field_ident.to_string().as_str()) {
                            vector_size_map.insert(field_ident.to_string());

                            if vec_type.is_none() {
                                let err = format!("Missing `size_for` or `size_type` for vector `{}`", field_ident.to_string());
                                error_stream.append_all(quote_spanned!(field_ident.span()=> compile_error!(#err);));

                                continue 'field_loop;
                            }
                            let t = vec_type.unwrap().1;
                            write_stream.append_all(quote!((#t::try_from(self.#field_ident.len()).expect("vector exceeds maximum allowed size")).write(writer);));
                            // Unwrapping when converting from our int type to usize is ok here: if
                            // the conversion fails, the vector cannot be represented in memory
                            // anyway.
                            read_body_stream.append_all(quote!(let #len_var_name = usize::try_from(#t::read(reader)).unwrap();));
                        }

                        // This part adds the actual writing/reading of the content of the vector.
                        // Should always happen.
                        if let PathArguments::AngleBracketed(generic_type) = &last.arguments {
                            write_stream.append_all(quote! {
                                for elem in &self.#field_ident {
                                    elem.write(writer);
                                }
                            });

                            let inner_type = generic_type.args.first().unwrap();
                            read_body_stream.append_all(quote! {
                                let #field_ident = (0..#len_var_name).map(|_| #inner_type::read(reader)).collect();
                            });
                            read_inner_stream.append_all(quote!(#field_ident,));
                        } else {
                            unreachable!();
                        }
                        continue 'field_loop;
                    }
                }
                // If we reach this part of the code, we know the field's type must not be a vector.
                // If vec_type is not None, then a `size_type` attribute has been specified on this
                // non-vector field, which is not allowed.
                if vec_type.is_some() {
                    error_stream.append_all(quote_spanned!(vec_type.unwrap().0=> compile_error!("the `size_type` attribute is only allowed on vectors");));
                }

                if enum_type.is_some() {
                    let et = enum_type.unwrap().1;
                    write_stream.append_all(quote!(<#field_type as crate::proto::io::EnumWritable<#et>>::write(&self.#field_ident, writer);));
                    read_body_stream.append_all(quote!(let #field_ident = <#field_type as crate::proto::io::EnumReadable<#field_type, #et>>::read(reader);));
                } else {
                    write_stream.append_all(quote!(<#field_type as crate::proto::io::Writable>::write(&self.#field_ident, writer);));
                    read_body_stream.append_all(quote!(let #field_ident = <#field_type as crate::proto::io::Readable<#field_type>>::read(reader);));
                }
                read_inner_stream.append_all(quote!(#field_ident,));
            }
            // We can only remove the fields that need to be removed after iterating over them, so
            // we remove them here.
            let mut xi = 0usize;
            named_fields.named = named_fields.named.clone().into_pairs().filter(|_| {
                let remove = !removal_queue.contains(&xi);
                xi += 1;
                remove
            }).collect();

            read_stream.append_all(quote! {
               #read_body_stream
                Self {
                    #read_inner_stream
                }
            });
        }
        Data::Enum(e) => {
            let type_name = syn::parse2::<proc_macro2::Ident>(_attr.clone().into()).unwrap();

            lazy_static! {
                static ref PRIM_RE: Regex = Regex::new("^(u|i)(8|(16)|(32)|(64)|(128))$").unwrap();
            };
            let is_primitive = PRIM_RE.is_match(type_name.to_string().as_str());

            let mut write_match_stream = proc_macro2::TokenStream::new();
            let mut read_match_stream = proc_macro2::TokenStream::new();

            let mut variant_number = 0i128;
            for variant in &mut e.variants {
                let variant_name = &variant.ident;
                if let Some(discriminant) = variant.discriminant.as_ref() {
                    if let Expr::Lit(lit) = &discriminant.1 {
                        if let Lit::Int(int_lit) = &lit.lit {
                            let val = int_lit.base10_digits().parse().unwrap();
                            variant_number = val;
                        }
                    }
                }

                let variant_number_token = proc_macro2::Literal::i128_unsuffixed(variant_number);
                let mut enum_content = proc_macro2::TokenStream::new();
                let mut enum_content_read = proc_macro2::TokenStream::new();
                let mut enum_content_write = proc_macro2::TokenStream::new();
                for field in &variant.fields {
                    if !enum_content.is_empty() {
                        error_stream.append_all(quote_spanned!(variant.fields.span()=> compile_error!("enum variant exceeds the maximum allowed field count of 1");));
                        break;
                    }
                    let field_name = format_ident!("e");
                    let field_type = &field.ty;
                    enum_content.append_all(quote!((#field_name)));
                    enum_content_write.append_all(quote!(e.write(writer);));
                    enum_content_read.append_all(quote!((#field_type::read(reader))));
                }
                match is_primitive {
                    false => {
                        write_match_stream.append_all(quote! {
                            #ident::#variant_name #enum_content => {
                                D::try_from(#type_name(#variant_number_token)).unwrap().write(writer);
                                #enum_content_write
                            },
                        });
                        read_match_stream.append_all(quote! {
                            #type_name(#variant_number_token) => #ident::#variant_name #enum_content_read,
                        });
                    }
                    true => {
                        write_match_stream.append_all(quote! {
                            #ident::#variant_name #enum_content => {
                                D::try_from(#variant_number_token as #type_name).unwrap().write(writer);
                                #enum_content_write
                            },
                        });
                        read_match_stream.append_all(quote! {
                            #variant_number_token => #ident::#variant_name #enum_content_read,
                        });
                    }
                };
                variant_number += 1;
            }

            write_stream.append_all(quote! {
                <#ident as crate::proto::io::EnumWritable<#type_name>>::write(self, writer)
            });
            read_stream.append_all(quote! {
                <#ident as crate::proto::io::EnumReadable<#ident, #type_name>>::read(reader)
            });

            extra_stream.append_all(quote! {
                impl<D: crate::proto::io::Writable + TryFrom<#type_name>> crate::proto::io::EnumWritable<D> for #ident
                where <D as TryFrom<#type_name>>::Error: std::fmt::Debug {
                    fn write(&self, writer: &mut crate::proto::io::Writer) {
                        match self {
                            #write_match_stream
                        };
                    }
                }

                impl<D: crate::proto::io::Readable<D> + TryInto<#type_name>> crate::proto::io::EnumReadable<#ident, D> for #ident
                where <D as TryInto<#type_name>>::Error: std::fmt::Debug {
                    fn read(reader: &mut crate::proto::io::Reader) -> #ident {
                        match D::read(reader).try_into().unwrap() {
                            #read_match_stream
                            _ => panic!("Unknown enum variant"),
                        }
                    }
                }
            });
        }
        Data::Union(u) => {
            error_stream.append_all(quote_spanned!(u.union_token.span()=> compile_error!("Unions are not supported");))
        }
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

        impl crate::proto::io::Readable<#ident> for #ident {
            fn read(reader: &mut crate::proto::io::Reader) -> #ident {
                #read_stream
            }
        }

        #extra_stream
    };
    tok.into()
}
