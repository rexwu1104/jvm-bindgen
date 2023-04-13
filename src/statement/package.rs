use syn::{parse::Parse, Token, Ident};

pub struct Package {
    pub (crate) path: String
}

impl Parse for Package {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut paths = vec![];
        loop {
            if let Ok(ident) = input.parse::<Ident>() {
                paths.push(ident.to_string());
            }

            if !input.peek(Token![,]) {
                break Ok(Package { path: paths.join("\\") });
            }

            input.parse::<Token![,]>()?;
        }
    }
}