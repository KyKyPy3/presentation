wit_bindgen::generate!();

pub struct MyGuest;

impl Guest for MyGuest {
    fn transform(input: String) -> String {
        input.to_uppercase()
    }
}

export!(MyGuest);
