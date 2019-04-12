use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_json::json;
use crate::query::*;


#[derive(Serialize, Deserialize)]
pub enum Must {
  #[serde(rename = "match")]
  Match { key: String, value: String },
  #[serde(rename = "match_phrase")]
  MatchPhrase { key: String, value: String },
}


pub fn query_from(records: i8, matchers: &Vec<Must>, time_range: &TimeRange) -> Value {

  let range = match time_range {
    TimeRange::Within(start, end) => json!({
                    "gte": start,
                    "lte": end
                }),
    TimeRange::Since(start) => json!({
                    "gte": start
                }),
    TimeRange::Until(end) => json!({
                    "lte": end
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

#[cfg(test)]
mod elasticsearch_tests {
  use chrono::prelude::*;
  use super::*;

  #[test]
  fn query_test() {
    let matchers = vec!(
      Must::Match { key: String::from("key1"), value: String::from("value1") },
      Must::MatchPhrase { key: String::from("key2"), value: String::from("value2") },
    );

    let start = Utc.ymd(2019, 3, 30).and_hms(9, 5, 0);
    let end = Utc.ymd(2019, 3, 30).and_hms(12, 5, 0);

    let date_range = TimeRange::Within(start, end);

    let response = query_from(1, &matchers, &date_range);

    let expected = r#"{"query":{"bool":{"filter":{"range":{"@timestamp":{"gte":"2019-03-30T09:05:00Z","lte":"2019-03-30T12:05:00Z"}}},"must":[{"match":{"key":"key1","value":"value1"}},{"match_phrase":{"key":"key2","value":"value2"}}]}},"size":1}"#;

    let expectedJson: Value = serde_json::from_str(expected).unwrap();

    assert_eq!(response, expectedJson);
  }
}
