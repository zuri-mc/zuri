use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Implements the Component trait. Currently, this generates an empty trait implementation.
#[proc_macro_derive(Component)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();

    let name = ast.ident;
    let out = quote! {
        impl zuri_world::block::component::Component for #name {}
    };
    out.into()
}
