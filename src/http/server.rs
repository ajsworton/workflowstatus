extern crate hyper;
extern crate futures;

use hyper::Server;
use hyper::rt::Future;
use hyper::service::service_fn;

use futures::future;
use hyper::Method;
use hyper::{Body, Request, Response};

mod endpoints;
pub mod elasticsearch;

pub fn serve(addr: [u8; 4], port: u16) {
  let socket_addr = (addr, port).into();

  let server = Server::bind(&socket_addr)
    .serve(||service_fn(endpoints))
    .map_err(|e| eprintln!("server error: {}", e));

  let wf_status = r#"

 __          ________ _____ _        _
 \ \        / /  ____/ ____| |      | |
  \ \  /\  / /| |__ | (___ | |_ __ _| |_ _   _ ___
   \ \/  \/ / |  __| \___ \| __/ _` | __| | | / __|
    \  /\  /  | |    ____) | || (_| | |_| |_| \__ \
     \/  \/   |_|   |_____/ \__\__,_|\__|\__,_|___/


  "#;

  println!("{}", wf_status);
  println!("Server starting on address {:?} using port {}", addr, port);
  hyper::rt::run(server);
}

type FResponseBody = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn endpoints(req: Request<Body>) -> FResponseBody {
  let response: Response<Body> = match (req.method(), req.uri().path()) {

    (&Method::POST, "/_meta/echo") => endpoints::meta::echo(req),
    (_, "/_meta/ping") => endpoints::meta::ping(),
    (_, "/_meta/status") => endpoints::meta::status(),
    (_, "/workflow/status") => endpoints::workflow::status(req),
    (_, "/workflow") => endpoints::workflow::test(req),
    _ => endpoints::meta::not_found(),
  };

  Box::new(future::ok(response))
}


