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


pub fn query(records: i8, matchers: Vec<Must>) -> Value {
  let now: DateTime<Utc> = Utc::now();

  let json = json!({
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
     });
  json
}

pub fn create_matcher(matcher: &str, key: &str, value: &str) -> Value {
  json!({ matcher: { key: value } })
}