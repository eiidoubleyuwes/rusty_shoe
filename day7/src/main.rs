//Learning about nickel the web framework
//It will expose port 8080
#[macro_use] extern crate nickel;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "You need to stop with the drugs"
        }
        get "**" => |_req, _res| {
            "Young man"
        }
    });

    server.listen("127.0.0.1:8080").unwrap();
}