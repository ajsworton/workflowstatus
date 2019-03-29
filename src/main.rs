extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate futures;

mod http;

use http::server::elasticsearch::lib::*;



fn main() {

    let addr = [0,0,0,0];
    let port = 5000;

//    http::server::serve(addr, port);

    let matchers: Vec<Must> = vec!(
        Must::new("match", "appname", "live2vod-lambdas"),
        Must::new("match", "lambda_function", "cdt-live2vod-s3event-lambda-prd"),
        Must::new("match_phrase", "message", "Sent SQS Message"),
    );

    let j = http::server::elasticsearch::lib::query(1, matchers);

    println!("{}", j.to_string())
}
