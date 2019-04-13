use hyper::{Body, Request, Response};

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

//pub fn test(_req: Request<Body>) -> Response<Body> {
//
//  let matchers = vec!(
//    Must::Match{ key: String::from("appname"), value: String::from("live2vod-lambdas") },
//    Must::Match{ key: String::from("lambda_function"), value: String::from("cdt-live2vod-s3event-lambda-prd") },
//    Must::MatchPhrase{ key: String::from("message"), value: String::from("Sent SQS Message"), },
//  );
//
//  let json: Value = query_from(1, &matchers, &TimeRange::default());
//
//  Response::builder()
//    .status(200)
//    .header("Content-Type", "application/json")
//    .body(Body::from(json.to_string()))
//    .unwrap()
//}