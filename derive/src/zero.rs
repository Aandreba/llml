use syn::{Type, Fields, Fields::Named};
use itertools::Itertools;

/// Derives the Zero trait by giving all fields inside a struct the value ```T::zero()```
#[proc_macro_derive(Zero)]
pub fn derive_zero (input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_zero(&ast)
}

fn impl_zero(ast: &syn::DeriveInput) -> TokenStream {
    let data = &ast.data;
    let gen = match data {
        syn::Data::Struct(data) => impl_zero_struct(ast, data),
        syn::Data::Enum(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    };

    gen.into()
}

fn impl_zero_struct (ast: &syn::DeriveInput, data: &DataStruct) -> TokenStream {
    let fields = &data.fields;
    match fields {
        Named(_) => impl_zero_named_struct(ast, fields),
        _ => todo!()
    }
}

fn impl_zero_named_struct (ast: &syn::DeriveInput, fields: &Fields) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    
    let field = fields.iter()
        .map(|x| x.ident.as_ref().unwrap());

    let ty = fields.iter()
        .map(|x| &x.ty)
        .unique();

    let ty2 = fields.iter()
        .map(|x| &x.ty);
    
    let name = if generics.params.len() > 0 { quote! { #name::#generics } } else { quote! { #name } };

    let gen = quote! {
        impl #generics llml::others::Zero for #name #generics where #( #ty : llml::others::Zero, )* {
            #[inline]
            fn zero () -> Self {
                #name {
                    #(
                        #field: <#ty2>::zero(),
                    )*
                }
            }
        }
    };

    gen.into()
}