mod bindings;

use bindings::exports::component::cities::city_info::Guest;

struct Component;

impl Guest for Component {
    fn current_city() -> String {
        "Moscow".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
