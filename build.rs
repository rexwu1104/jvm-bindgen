use std::env;

fn main() {
    let path = env::var("JAVA_HOME").unwrap();
    println!("cargo:rustc-link-search={path}lib");
    println!("cargo:rustc-link-lib=jvm");
}