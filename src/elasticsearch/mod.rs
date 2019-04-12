use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_json::json;
use crate::query::*;
use hyper::client::HttpConnector;
use hyper::{header, Body, Chunk, Uri, Client, Method, Request};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use chrono::Duration;
use futures::stream;

mod http;


#[derive(Serialize, Deserialize)]
pub enum Must {
  #[serde(rename = "match")]
  Match(HashMap<String, String>),
  #[serde(rename = "match_phrase")]
  MatchPhrase(HashMap<String, String>),
}

pub fn test_json() {
  stage1_json();
  stage2_json();
  stage3_json();
}

pub fn stage1_json() {

  let matchers = vec!(
    get_match("appname", "live2vod-lambdas"),
    get_match("lambda_function", "cdt-live2vod-s3event-lambda-prd"),
    get_match("message", "0/0000/0000#001"),
    get_match_phrase("message", "Sent SQS Message"),
  );

  let q = query_from(1, &matchers, &TimeRange::Since(Utc::now() - Duration::hours(1) - Duration::minutes(5)));
  println!("Query: {}", q)
}

pub fn stage2_json() {

  let matchers = vec!(
    get_match("appname", "live2vod-lambdas"),
    get_match("lambda_function", "cdt-live2vod-copyrendition-lambda-prd"),
    get_match("message", "0/0000/0000#001"),
    get_match_phrase("message", "Sent SQS Message"),
  );

  let q = query_from(1, &matchers, &TimeRange::Since(Utc::now() - Duration::hours(1) - Duration::minutes(5)));
  println!("Query: {}", q)
}

pub fn stage3_json() {

  let matchers = vec!(
    get_match("appname", "live2vod-lambdas"),
    get_match("lambda_function", "cdt-live2vod-renditioncopycomplete-lambda-prd"),
    get_match("message", "0/0000/0000#001"),
    get_match_phrase("message", "Sent SQS Message"),
  );

  let q = query_from(1, &matchers, &TimeRange::Since(Utc::now() - Duration::hours(1) - Duration::minutes(5)));
  println!("Query: {}", q)
}

fn get_match(key: &str, value: &str) -> Must {
  let mut hm: HashMap<String, String> = HashMap::new();
  hm.insert(key.to_owned(), value.to_owned());
  Must::Match(hm)
}

fn get_match_phrase(key: &str, value: &str) -> Must {
  let mut hm: HashMap<String, String> = HashMap::new();
  hm.insert(key.to_owned(), value.to_owned());
  Must::MatchPhrase(hm)
}

pub fn query_from(records: i8, matchers: &Vec<Must>, time_range: &TimeRange) -> Value {

  let fmt = "%Y-%m-%d %H:%M:%S";
  let es_fmt = "yyyy-MM-dd HH:mm:ss";

  let range = match time_range {
    TimeRange::Within(start, end) => json!({
                    "gte": start.format(fmt).to_string(),
                    "lte": end.format(fmt).to_string(),
                    "format": es_fmt
                }),
    TimeRange::Since(start) => json!({
                    "gte": start.format(fmt).to_string(),
                    "format": es_fmt
                }),
    TimeRange::Until(end) => json!({
                    "lte": end.format(fmt).to_string(),
                    "format": es_fmt
                }),
  };

  json!({
      "size": records,
      "query": {
        "bool": {
           "must": matchers,
           "filter": {
             "range": {
               "@timestamp": range
             }
           }
        }
      }
     })
}

#[derive(Serialize, Deserialize)]
pub struct EsHits {
  pub total: usize,
}

pub struct EsConfig {
  base_uri: Uri,
}

pub struct ElasticQueryExecutor {
  config: EsConfig,
  http_client: Client<HttpConnector, Body>,
}

//Uri.uri("http://logs.infraprd.cd.itv.com:9200/logstash-*/_search")

fn to_json(query: &Query) -> Value {
  unimplemented!()
}

impl QueryExecutor for ElasticQueryExecutor {
  fn run(&self, stage_idx: u8, query: &Query) -> EventResult {

    let es_query = to_json(query);

    let root_url = Uri::from_static("http://logs.infraprd.cd.itv.com:9200/logstash-*/_search");

    let body = Body::wrap_stream(stream::once(serde_json::to_string(&es_query)));

    let request: Request<Body> = Request::builder()
      .method(Method::POST)
      .header(header::CONTENT_TYPE, "application/json")
      .uri(root_url.to_string())
      .body(body)
      .expect("couldn't build a request!");

    let result = http::expect::<EsHits>(&self.http_client, request);
    unimplemented!()
  }
}

#[cfg(test)]
mod elasticsearch_tests {
  use chrono::prelude::*;
  use super::*;

  #[test]
  fn query_test() {
//    let matchers = vec!(
//      Must::Match { key: String::from("key1"), value: String::from("value1") },
//      Must::MatchPhrase { key: String::from("key2"), value: String::from("value2") },
//    );
//
//    let start = Utc.ymd(2019, 3, 30).and_hms(9, 5, 0);
//    let end = Utc.ymd(2019, 3, 30).and_hms(12, 5, 0);
//
//    let date_range = TimeRange::Within(start, end);
//
//    let response = query_from(1, &matchers, &date_range);
//
//    let expected = r#"{"query":{"bool":{"filter":{"range":{"@timestamp":{"gte":"2019-03-30T09:05:00Z","lte":"2019-03-30T12:05:00Z"}}},"must":[{"match":{"key":"key1","value":"value1"}},{"match_phrase":{"key":"key2","value":"value2"}}]}},"size":1}"#;
//
//    let expectedJson: Value = serde_json::from_str(expected).unwrap();
//
//    assert_eq!(response, expectedJson);
  }
}
