#[macro_use]
extern crate nickel;

use nickel::Nickel;

fn say_hello() -> &'static str {
    "Hello my world!"
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            say_hello()
        }
    });

    let _ = server.listen("127.0.0.1:6767");
}
