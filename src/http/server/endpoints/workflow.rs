use hyper::{Body, Request, Response};
use serde_json::{json, Value};
use crate::http::server::elasticsearch::lib::Must;

pub fn status(req: Request<Body>) -> Response<Body> {
  let _workflow = req.uri().query();

  let body = Body::from(r#"{
  "stages": [
    {
      "order": 1,
      "name": "first",
      "reached": true
    },
    {
      "order": 2,
      "name": "second",
      "reached": true
    },
    {
      "order": 3,
      "name": "third",
      "reached": true
    },
    {
      "order": 4,
      "name": "fourth",
      "reached": false
    }
  ]

  }"#);

  Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(body)
    .unwrap()
}

pub fn test(req: Request<Body>) -> Response<Body> {

  let matchers = vec!(
    Must::Match{ key: String::from("appname"), value: String::from("live2vod-lambdas") },
    Must::Match{ key: String::from("lambda_function"), value: String::from("cdt-live2vod-s3event-lambda-prd") },
    Must::MatchPhrase{ key: String::from("message"), value: String::from("Sent SQS Message"), },
  );

  let json: Value = json!({
      "size": 1,
      "query": {
        "bool": {
           "must": matchers,
           "filter": {
             "range": {
               "@timestamp": {
                 "gte": "${from.fold[LocalDateTime](LocalDateTime.now.minusDays(1))(identity).format(DateTimeFormatter.ISO_INSTANT)}",
                 "lte": "${to.fold[LocalDateTime](LocalDateTime.now)(identity).format(DateTimeFormatter.ISO_INSTANT)}"
               }
             }
           }
        }
      }
     });

  Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(Body::from(json.to_string()))
    .unwrap()
}