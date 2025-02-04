#[no_mangle]
pub extern "C" fn uppercase(str: String) -> String {
    str.to_uppercase()
}
