extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::HashSet;
use lazy_static::lazy_static;
use syn::{Data, DeriveInput, Fields, FieldsNamed, parse_macro_input, PathArguments, Type};
use quote::{format_ident, quote, quote_spanned, TokenStreamExt, ToTokens};
use regex::Regex;
use syn::spanned::Spanned;

/// Implements `Readable<T>` and `Writable` for a type named `T`.
///
/// For a struct, it does so by looking at its fields and writing/reading them in the order they are
/// defined. All of these fields must also implement `Readable<S>` and `Writable` for a field with
/// type `S`.
///
/// Vectors are a special case. They do not implement Readable or Writable by themselves, but can
/// still be written of its content does. These vectors do require an attribute to specify what type
/// to use to write the length of the vector. This type needs to be convertable from and to usize.
///
/// The first such attribute is `#[size_type(L)]`, where `L` i the type to use for the vector
/// length. It should be put above the vector in question.
/// ```ignore
/// use zuri_net_derive::packet;
///
/// #[packet]
/// pub struct PacketWithVec {
///     #[size_type(u8)]
///     pub vec: Vec<String>
/// }
/// ```
/// This vector will use a u8 to write / read its length. Does not affect how String is read or
/// written.
///
/// The other attribute that can be used is `#[size_for(V)]`, which, unlike the previous attribute,
/// should be used on a field before the actual field with the vector (named `V`). It will make
/// previous field in a packet act like it is the size of that vector, allowing the size to be
/// written elsewhere than right before the vector's content. The type used will be the type of the
/// field. Note that in the macro expansion, this field will be removed. The field only exists to
/// specify how the packet's data is structured
/// ```ignore
/// use zuri_net_derive::packet;
///
/// #[packet]
/// pub struct PacketWithVec {
///     #[size_for(vec)]
///     __: u16,
///     pub some_field: f32,
///     pub vec: Vec<String>
/// }
/// ```
/// Note that the vector length is written with a u16 here. The field is named `__`, but it can have
/// any name (as it will be removed anyway). This also means that multiple size_for fields can be
/// named the same.
///
/// Enums work slightly differently. First, the discriminant of the variant is written and then
/// any data that might be present in that variant. Using this packet on an enum would look
/// something like this
/// ```ignore
/// use zuri_net_derive::packet;
///
/// #[packet(u8)]
/// #[repr(u8)]
/// pub enum EnumPacket {
///     Variant1,
///     Variant2(Data),
///     Variant3(Data, Data, f32) = 7,
/// }
///
/// #[packet]
/// pub struct Data;
/// ```
/// Here is can be seen that the macro has an extra parameters for enums: the size to use to write
/// and read the discriminant. Variants can also contain any amount of unnamed fields or have an
/// explicit discriminant.
///
/// Sometimes, enum discriminants are written with a different type for the same enum in the
/// minecraft protocol (for some reason). This is also supported. When using this macro on an enum
/// `T`, it automatically implements `EnumReadable<T, D>` and `EnumWritable<D>` for that enum, where
/// `D` refers to the new type used for the discriminant. `D` needs to be convertible from and to
/// the default type specified in the attribute, as well as be writable and readable. To write an
/// enum with a specific discriminant type, `#[enum_header(D)]` can be used.
/// ```ignore
/// use zuri_net_derive::packet;
///
/// #[packet]
/// pub struct PacketWithEnum {
///     #[enum_header(u16)]
///     pub my_enum: MyEnum,
/// }
///
/// #[packet(u8)]
/// pub enum MyEnum {
///     V1, V2
/// }
/// ```
/// In this example `MyEnum`, which is usually written with `u8` when no `enum_header` is specified,
/// will be written using a `u16`.
///
/// 'Fake' fields can also be added that are just copies of values of other fields. These fields
/// will be removed when the macro is expanded. Any expression can be used in the field, as long
/// as it produces a writable value. To ensure symmetry in reading and writing, the type of the
/// field should be the same as the one being written
/// ```ignore
/// use zuri_net_derive::packet;
///
/// #[packet]
/// pub struct PacketWithVec {
///     pub some_field: f32,
///     #[value(self.some_field)]
///     some_field_again: f32,
/// }
/// ```
/// Here we write the value of `some_field` again. If we instead wanted to write to a field of
/// `some_field` (if it had any), this can be done with the `.` operator like in normal rust code.
/// When reading, the value of `some_field` will not be overwritten when reading the duplicate. If
/// this is desired, use `#[overwrite(some_field)]` instead. Note that the `self` is not needed when
/// using overwrite.
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
                    // Helper function to parse attribute data of the form `(ident)`, and return the
                    // contained ident, or an error (to write to the error token stream) if parsing
                    // was unsuccessful.
                    fn parse_attribute_ident(tokens: proc_macro2::TokenStream) -> Result<proc_macro2::Ident, proc_macro2::TokenStream> {
                        let group = match syn::parse2::<proc_macro2::Group>(tokens) {
                            Ok(g) => g,
                            Err(err) => {
                                let err_msg = err.to_compile_error();
                                return Err(quote_spanned!(err.span()=> #err_msg));
                            }
                        };
                        let ident = match syn::parse2::<proc_macro2::Ident>(group.stream()) {
                            Ok(i) => i,
                            Err(err) => {
                                let err_msg = err.to_compile_error();
                                return Err(quote_spanned!(group.span()=> #err_msg));
                            }
                        };

                        Ok(ident)
                    }

                    let path = attr.path.to_token_stream().to_string();
                    if path == "size_for" {
                        // Get the vec name with delimiters `(` and `)`
                        match parse_attribute_ident(attr.tokens.clone()) {
                            Err(t) => error_stream.append_all(t),
                            Ok(vec_name) => {
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
                        };
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

                        match parse_attribute_ident(attr.tokens.clone()) {
                            Err(t) => error_stream.append_all(t),
                            Ok(type_name) => {
                                vec_type = Some((attr.span(), type_name));
                                attr_remove_queue.push(attr_i);
                            }
                        }
                    }
                    if path == "enum_header" {
                        match parse_attribute_ident(attr.tokens.clone()) {
                            Err(t) => error_stream.append_all(t),
                            Ok(type_name) => {
                                enum_type = Some((attr.span(), type_name));
                                attr_remove_queue.push(attr_i);
                            }
                        }
                    }
                    if path == "value" || path == "overwrite" {
                        if attr.tokens.is_empty() {
                            error_stream.append_all(quote_spanned!(attr.span()=> compile_error!("expression expected");));
                        }
                        removal_queue.push(field_i);

                        let tokens = match syn::parse2::<proc_macro2::Group>(attr.tokens.clone()) {
                            Ok(g) => g,
                            Err(err) => {
                                let err_msg = err.to_compile_error();
                                error_stream.append_all(quote_spanned!(err.span()=> #err_msg));
                                continue 'field_loop;
                            }
                        }.stream();
                        if path == "overwrite" {
                            write_stream.append_all(quote!(eq::<#field_type>(self.#tokens);));
                            write_stream.append_all(quote!(self.#tokens.write(writer);));
                            read_body_stream.append_all(quote!(#tokens = <#field_type>::read(reader);));
                        } else {
                            write_stream.append_all(quote!(eq::<#field_type>(#tokens);));
                            write_stream.append_all(quote!(#tokens.write(writer);));
                            read_body_stream.append_all(quote!(<#field_type>::read(reader);));
                        }
                        continue 'field_loop;
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
                    read_body_stream.append_all(quote!(let mut #field_ident = <#field_type as crate::proto::io::EnumReadable<#field_type, #et>>::read(reader);));
                } else {
                    write_stream.append_all(quote!(<#field_type as crate::proto::io::Writable>::write(&self.#field_ident, writer);));
                    read_body_stream.append_all(quote!(let mut #field_ident = <#field_type as crate::proto::io::Readable<#field_type>>::read(reader);));
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
            // Get the type name provided in `#[packet(TYPE_NAME)]`.
            let type_name = match syn::parse2::<proc_macro2::Ident>(_attr.clone().into()) {
                Ok(t) => t,
                Err(err) => {
                    let err_msg = if _attr.is_empty() {
                        format!("expected default variant type for enum `{}`", ident.to_string())
                    } else {
                        format!("unexpected token in default variant type for enum `{}`", ident.to_string())
                    };
                    error_stream.append_all(quote_spanned!(err.span()=> compile_error!(#err_msg);));
                    format_ident!("_")
                }
            };

            // Regex used to detect if the default variant type of an enum is a primitive integer
            // type. This is used to determine how the read and write function should be implemented
            // for this enum, as the implementation is different when using a builtin type compared
            // to a user-defined integer type.
            lazy_static! {
                static ref PRIM_RE: Regex = Regex::new("^(u|i)(8|(16)|(32)|(64)|(128))$").unwrap();
            }
            ;
            let is_primitive = PRIM_RE.is_match(type_name.to_string().as_str());

            // Token streams for all the match cases in the read/write implementation.
            let mut write_match_stream = proc_macro2::TokenStream::new();
            let mut read_match_stream = proc_macro2::TokenStream::new();
            // Token streams for the fallback (_) case.
            let mut write_fallback_stream = proc_macro2::TokenStream::new();
            let mut read_fallback_stream = proc_macro2::TokenStream::new();

            // Use an i128 to store the variant number to ensure both i64 and u64 discriminants
            // work.
            let mut variant_number = 0i128;
            'variant_loop: for variant in &mut e.variants {
                let variant_name = &variant.ident;

                let mut attr_remove_queue = vec![];
                for (attr_i, attr) in variant.attrs.iter().enumerate() {
                    if attr.path.to_token_stream().to_string() != "fallback" {
                        continue;
                    }
                    attr_remove_queue.push(attr_i);

                    if !read_fallback_stream.is_empty() || !write_fallback_stream.is_empty() {
                        error_stream.append_all(quote_spanned!(attr.span()=> compile_error!("cannot have more than one fallback variant");));
                    }
                    let err_msg = format!("trying to write fallback variant for enum {}", ident);
                    write_fallback_stream.append_all(quote!(_ => panic!(#err_msg)));
                    read_fallback_stream.append_all(quote!(_ => #ident::#variant_name));
                }
                attr_remove_queue.sort();
                for attr_i in attr_remove_queue.iter().rev() {
                    variant.attrs.remove(*attr_i);
                }
                if !attr_remove_queue.is_empty() {
                    continue 'variant_loop;
                }

                // Check if the variant has an explicit integer discriminant such as the `1` in
                // `Variant = 1`.
                if let Some(discriminant) = variant.discriminant.as_ref() {
                    match discriminant.1.to_token_stream().to_string().chars().filter(|c| !c.is_ascii_whitespace()).collect::<String>().parse::<i128>() {
                        Err(err) => {
                            let err_msg = format!("could not parse enum variant into integer: {}", err);
                            error_stream.append_all(quote_spanned!(discriminant.1.span()=> compile_error!(#err_msg);));
                        }
                        Ok(val) => variant_number = val,
                    }
                }

                let variant_number_token = proc_macro2::Literal::i128_unsuffixed(variant_number);
                let mut enum_content = proc_macro2::TokenStream::new();
                let mut enum_content_read = proc_macro2::TokenStream::new();
                let mut enum_content_write = proc_macro2::TokenStream::new();

                // Go over all the 'fields' of the variant. For a tuple variant, this would be the
                // Type1, Type2, etc in `MyVariant(Type1, Type2)`. First checks if the fields are
                // named (in the form of {}), as this is not supported.
                if let Fields::Named(_) = &variant.fields {
                    error_stream.append_all(quote_spanned!(variant.fields.span()=> compile_error!("enum variant cannot have named fields");));
                }
                for (field_num, field) in variant.fields.iter().enumerate() {
                    let field_name = format_ident!("e_{}", field_num);
                    let field_type = &field.ty;
                    enum_content.append_all(quote!(#field_name,));
                    enum_content_write.append_all(quote!(<#field_type as crate::proto::io::Writable>::write(#field_name, writer);));
                    enum_content_read.append_all(quote!(<#field_type as crate::proto::io::Readable<#field_type>>::read(reader),));
                }
                if !enum_content.is_empty() {
                    enum_content = quote!((#enum_content));
                }
                if !enum_content_read.is_empty() {
                    enum_content_read = quote!((#enum_content_read));
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

            // Write the default Writable and Readable function bodies.
            write_stream.append_all(quote! {
                <#ident as crate::proto::io::EnumWritable<#type_name>>::write(self, writer)
            });
            read_stream.append_all(quote! {
                <#ident as crate::proto::io::EnumReadable<#ident, #type_name>>::read(reader)
            });

            if read_fallback_stream.is_empty() {
                read_fallback_stream = quote!(_ => panic!("Unknown enum variant"));
            }

            extra_stream.append_all(quote! {
                impl<D: crate::proto::io::Writable + TryFrom<#type_name>> crate::proto::io::EnumWritable<D> for #ident
                where <D as TryFrom<#type_name>>::Error: std::fmt::Debug {
                    fn write(&self, writer: &mut crate::proto::io::Writer) {
                        match self {
                            #write_match_stream
                            #write_fallback_stream
                        };
                    }
                }

                impl<D: crate::proto::io::Readable<D> + TryInto<#type_name>> crate::proto::io::EnumReadable<#ident, D> for #ident
                where <D as TryInto<#type_name>>::Error: std::fmt::Debug {
                    fn read(reader: &mut crate::proto::io::Reader) -> #ident {
                        match D::read(reader).try_into().unwrap() {
                            #read_match_stream
                            #read_fallback_stream
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
                // hack to be able to see if an expression is of a certain type on compile time
                #[inline]
                fn eq<T>(_: T) {}
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
