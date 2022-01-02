/// Derives the Hypot trait by calculating ```sqrt(self * self + rhs * rhs)```
#[proc_macro_derive(Hypot)]
pub fn derive_hypot (input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hypot(&ast)
}

fn impl_hypot(ast: &syn::DeriveInput) -> TokenStream {
    let data = &ast.data;
    let gen = match data {
        syn::Data::Struct(data) => impl_hypot_struct(ast, data),
        syn::Data::Enum(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    };

    gen.into()
}

fn impl_hypot_struct (ast: &syn::DeriveInput, data: &DataStruct) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    let gen = quote! {
        impl llml::others::Hypot for #name #generics where Self: Clone + Mul<Self, Output = Self> + Sqrt {
            #[inline]
            fn sin_cos (self, rhs: Self) -> Self {
                (self.clone() * self + rhs.clone() * rhs).sqrt()
            }
        }
    };

    gen.into()
}