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




#[test]
fn query_test() {

  let matchers = vec!(
    Must::Match{ key: String::from("appname"), value: String::from("live2vod-lambdas") },
    Must::Match{ key: String::from("lambda_function"), value: String::from("cdt-live2vod-s3event-lambda-prd") },
    Must::MatchPhrase{ key: String::from("message"), value: String::from("Sent SQS Message"), },
  );

  let response = query(1, matchers);

  assert_eq!(response.to_string(), r#"{"query":{"bool":{"filter":{"range":{"@timestamp":{"gte":"2019-03-29T12:06:06.832089342Z","lte":"2019-03-30T12:06:06.832089342Z"}}},"must":[{"match":{"key":"appname","value":"live2vod-lambdas"}},{"match":{"key":"lambda_function","value":"cdt-live2vod-s3event-lambda-prd"}},{"match_phrase":{"key":"message","value":"Sent SQS Message"}}]}},"size":1}"#);
}