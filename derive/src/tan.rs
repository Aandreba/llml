/// Derives the Hypot trait by calculating ```sqrt(self * self + rhs * rhs)```
#[proc_macro_derive(Tan)]
pub fn derive_tan (input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_tan(&ast)
}

fn impl_tan(ast: &syn::DeriveInput) -> TokenStream {
    let data = &ast.data;
    let gen = match data {
        syn::Data::Struct(_) => impl_tan_struct(ast),
        syn::Data::Enum(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    };

    gen.into()
}

fn impl_tan_struct (ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    let gen = quote! {
        impl llml::others::Tan for #name #generics where Self: SinCos + Div<Output = Self> {
            #[inline]
            fn tan (self) -> Self {
                let sin_cos = self.sin_cos();
                sin_cos.0 / sin_cos.1
            }
        }
    };

    gen.into()
}