#![feature(const_io_structs)]
#![feature(const_result_drop)]
#![feature(const_option)]

mod statement;
mod resolve;
mod wrapper;

use proc_macro::TokenStream;
use statement::Imports;
use wrapper::ZipWrapper;

#[proc_macro]
pub fn jimport(statements: TokenStream) -> TokenStream {
    syn::parse_macro_input!(statements as Imports).resolve()
}

#[cfg(windows)]
const JAVA_LIB: &str = concat!(env!("JAVA_HOME"), "lib\\");

const fn read_zip<'a>() -> ZipWrapper<'a> {
    ZipWrapper::new()
}

static mut ZIP_FILE_CONTENT: ZipWrapper = read_zip();

#[proc_macro_attribute]
pub fn path(attribute: TokenStream, item: TokenStream) -> TokenStream {
    println!("{JAVA_LIB} {attribute}");
    unsafe {
        if ZIP_FILE_CONTENT.is_not_load() {
            ZIP_FILE_CONTENT.load().unwrap();
        }
        
        println!("{}", String::from_utf8_lossy(&ZIP_FILE_CONTENT.by_name("System.java".to_string()).compressed_data));
        item
    }
}