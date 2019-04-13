use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_json::json;
use crate::query::*;
use hyper::client::HttpConnector;
use hyper::{header, Body, Uri, Client, Method, Request};
use std::collections::HashMap;
use futures::stream;
use futures::prelude::*;
use hyper::rt::{self, Future, Stream};

mod http;


#[derive(Serialize, Deserialize)]
pub enum Must {
  #[serde(rename = "match")]
  Match(HashMap<String, String>),
  #[serde(rename = "match_phrase")]
  MatchPhrase(HashMap<String, String>),
}

pub fn get_match(key: &str, value: &str) -> Must {
  let mut hm: HashMap<String, String> = HashMap::new();
  hm.insert(key.to_owned(), value.to_owned());
  Must::Match(hm)
}

pub fn get_match_phrase(key: &str, value: &str) -> Must {
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
  http_client: Client<HttpConnector, Body>,
}

//Uri.uri("http://logs.infraprd.cd.itv.com:9200/logstash-*/_search")

fn to_json(query: &Query) -> Value {
  unimplemented!()
}

impl ElasticQueryExecutor {
  pub fn new() -> Self {
   let http_client = Client::new();

    ElasticQueryExecutor {
      http_client
    }
  }
}

impl QueryExecutor for ElasticQueryExecutor {

  fn run(&self, stage_idx: u8, es_query: &Value) -> EventResult {
    let root_url = Uri::from_static("http://logsv6.infraprd.cd.itv.com:9200/logstash-*/_search");

    let body = Body::wrap_stream(stream::once(serde_json::to_string(&es_query)));

    let request: Request<Body> = Request::builder()
      .method(Method::POST)
      .header(header::CONTENT_TYPE, "application/json")
      .uri(root_url.to_string())
      .body(body)
      .expect("couldn't build a request!");

    let fut = http::expect::<EsHits>(&self.http_client, request);

    rt::spawn(fut);
    fut.wait().map(|es_hits| {
      if es_hits.total > 0 {
        Some(Event { stage_idx })
      } else {
        None
      }
    })
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
