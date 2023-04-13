#![feature(const_io_structs)]
#![feature(const_result_drop)]
#![feature(const_option)]
#![feature(const_mut_refs)]

mod statement;
mod structs;
mod build_init;

use build_init::init;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use statement::{Imports, Package};

type TokenStream2 = quote::__private::TokenStream;

const _: () = {
    init();
};

#[proc_macro]
pub fn jimport(statements: TokenStream) -> TokenStream {
    syn::parse_macro_input!(statements as Imports).resolve()
}

#[proc_macro_attribute]
pub fn path(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let package = syn::parse_macro_input!(attribute as Package);
    let tokens = TokenStream2::from(item.clone());
    let item_struct: syn::ItemStruct = syn::parse_macro_input!(item);

    let package_path = [
        env!("process_dir").into(),
        package.path.clone(),
        [item_struct.ident.to_string(), String::from(".rs")].concat()
    ].join("\\");

    quote! {
        include!(#package_path);
        #tokens
    }.into()
}

#[proc_macro]
pub fn parse_java(tokentree: TokenStream) -> TokenStream {
    // println!("{}", tokentree.to_string());
    TokenStream::new()
}