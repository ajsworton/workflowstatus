extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate futures;

mod http;

fn main() {

    let addr = [0,0,0,0];
    let port = 5000;

    http::server::serve(addr, port);


}
