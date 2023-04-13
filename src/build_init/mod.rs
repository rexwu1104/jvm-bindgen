pub const fn init() {
    if !cfg!(bytecode_build) {
        panic!("Only can use to library.");
    }
}