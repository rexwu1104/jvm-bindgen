use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse::Parse, Ident, Token};

#[derive(Debug)]
pub struct Import {
    prefix: Vec<Ident>,
    item: Ident
}

impl Parse for Import {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let import_keyword: Ident = input.parse()?;
        if import_keyword.to_string() == "import" {
            let mut prefix = vec![];
            loop {
                let token: Ident = match input.parse() {
                    Ok(ident) => ident,
                    Err(err) => break Err(err)
                };

                if input.peek(Token![;]) {
                    input.parse::<Token![;]>()?;
                    break Ok(Import { prefix, item: token });
                }
                
                if !input.peek(Token![.]) {
                    break Err(syn::Error::new(Span::call_site().into(), "parse import statement error."));
                }

                input.parse::<Token![.]>()?;
                prefix.push(token);
            }
        } else {
            return Err(syn::Error::new(import_keyword.span(), "not find import keyword at start."));
        }
    }
}

impl Import {
    pub fn resolve(&self) -> TokenStream {
        let path = self.prefix.clone();
        let name = self.item.clone();
        quote! {
            #[::jvm_bindgen::path(#(#path),*)]
            struct #name;
        }.into()
    }
}

#[derive(Debug)]
pub struct Imports {
    imports: Vec<Import>
}

impl Parse for Imports {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut imports = vec![];
        loop {
            match input.parse::<Import>() {
                Ok(import) => imports.push(import),
                Err(_) => break Ok(Imports { imports })
            }
        }
    }
}

impl Imports {
    pub fn resolve(&self) -> TokenStream {
        let imports = self.imports.iter().map(Import::resolve);
        let result = imports.into_iter().reduce(|mut p, c| {
            p.extend(c.into_iter());
            p
        }).unwrap();

        result
    }
}