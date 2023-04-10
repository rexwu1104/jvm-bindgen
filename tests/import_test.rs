#[test]
fn import() {
    jvm_bindgen::jimport!(
        import java.lang.System;
        // import java.utils.ArrayList;
    );
}