mod bindings;

use bindings::component::cities::city_info::current_city;

fn main() {
    let current_city = current_city();

    println!("Hello {current_city}");
}
