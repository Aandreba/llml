/// Derives the SinCos trait by returning both the sine and cosine simultaneously
#[proc_macro_derive(SinCos)]
pub fn derive_sin_cos (input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_sin_cos(&ast)
}

fn impl_sin_cos(ast: &syn::DeriveInput) -> TokenStream {
    let data = &ast.data;
    let gen = match data {
        syn::Data::Struct(data) => impl_sin_cos_struct(ast, data),
        syn::Data::Enum(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    };

    gen.into()
}

fn impl_sin_cos_struct (ast: &syn::DeriveInput, data: &DataStruct) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    let gen = quote! {
        impl llml::others::SinCos for #name #generics {
            #[inline]
            fn sin_cos (self) -> (Self, Self) {
                (self.clone().sin(), self.cos())
            }
        }
    };

    gen.into()
}