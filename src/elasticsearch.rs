use chrono::{Utc, DateTime, Duration};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub enum Must {
  #[serde(rename = "match")]
  Match { key: String, value: String },
  #[serde(rename = "match_phrase")]
  MatchPhrase { key: String, value: String },
}

pub fn query_from(records: i8, matchers: &Vec<Must>, now: DateTime<Utc>) -> Value {
  json!({
      "size": records,
      "query": {
        "bool": {
           "must": matchers,
           "filter": {
             "range": {
               "@timestamp": {
                 "gte": now - Duration::days(1),
                 "lte": now
               }
             }
           }
        }
      }
     })
}

#[cfg(test)]
mod elasticsearch_tests {
  use chrono::prelude::*;

  #[test]
  fn query_test() {
    let matchers = vec!(
      super::Must::Match { key: String::from("key1"), value: String::from("value1") },
      super::Must::MatchPhrase { key: String::from("key2"), value: String::from("value2") },
    );

    let date_time = Utc.ymd(2019, 3, 30).and_hms(9, 5, 0);

    let response = super::query_from(1, &matchers, date_time);

    let expected = r#"{"query":{"bool":{"filter":{"range":{"@timestamp":{"gte":"2019-03-29T09:05:00Z","lte":"2019-03-30T09:05:00Z"}}},"must":[{"match":{"key":"key1","value":"value1"}},{"match_phrase":{"key":"key2","value":"value2"}}]}},"size":1}"#;

    assert_eq!(response.to_string(), expected);
  }
}
