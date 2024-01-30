use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(MyDebug)]
pub fn debug(item: TokenStream) -> TokenStream {
    let item: DeriveInput = syn::parse(item).unwrap();
    let name = &item.ident;

    let mut content: Vec<Ident> = Vec::new();
    let mut format_code = proc_macro2::TokenStream::new();

    if let syn::Data::Struct(s) = item.data {
        s.fields.iter().for_each(|f| {
            let n = f.ident.clone().unwrap();
            content.push(n.clone());
            format_code.extend(quote! { #n:{},});
        })
    }

    let prefix = quote!(
        #name format: #format_code
    )
    .to_string();

    let d = quote!(
        impl std::fmt::Debug for #name {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(fmt, #prefix, #(self.#content),*)
            }
        }
    );

    d.into()
}
