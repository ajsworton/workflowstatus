use hyper::{Body, Request, Response};
use hyper::StatusCode;


pub fn ping() -> Response<Body> {
  Response::new(Body::from("pong"))
}

pub fn status() -> Response<Body> {
  Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(Body::from(r#"{"version":"0.1"}"#))
    .unwrap()
}

pub fn echo(req: Request<Body>) -> Response<Body> {
  Response::new(req.into_body())
}

pub fn not_found() -> Response<Body> {
  Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()).unwrap()
}
