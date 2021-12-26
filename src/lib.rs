use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn make_optional(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as syn::parse::Nothing);
    impl_make_optional(&mut input).into()
}

fn impl_make_optional(obj: &mut ItemStruct) -> proc_macro2::TokenStream {
    match obj.fields {
        syn::Fields::Named(ref mut fields) => fields.named.iter_mut().for_each(|field| {
            let orig_ty = &field.ty;
            field.ty = syn::Type::Verbatim(quote!(Option<#orig_ty>))
        }),
        syn::Fields::Unnamed(ref mut fields) => fields.unnamed.iter_mut().for_each(|field| {
            let orig_ty = &field.ty;
            field.ty = syn::Type::Verbatim(quote!(Option<#orig_ty>))
        }),
        _ => {}
    }
    quote! {
        #obj
    }
}
