use chrono::{Utc, DateTime, Duration};
use serde::{Deserialize, Serialize, Deserializer, Serializer};
use serde::ser;
use serde::ser::SerializeStruct;
use serde_json::Value;
use serde_json::json;
use std::collections::hash_map::HashMap;

#[derive(Serialize)]
pub struct Pair {
  key: String,
  value: String,
}


pub struct Must {
  match_name: String,
  matcher: Pair,
}


//TODO: Fix broken serialiser
impl Serialize for Must {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: ser::Serializer,
  {
    let mut must = serializer.serialize_struct("Must", 1)?;
    must.serialize_field(&self.match_name, &self.matcher)?;
    must.end()
  }
}


impl Must {
  pub fn new(matcher: &str, key: &str, value: &str) -> Must {
    let t: Value = json!({});

    Must {
      match_name: matcher.to_owned(),
      matcher: Pair {
        key: key.to_owned(),
        value: value.to_owned(),
      },
    }
  }
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