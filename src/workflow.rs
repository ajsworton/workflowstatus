use crate::elasticsearch;
use crate::query::*;
use chrono::{Duration, Utc};
use serde_json::Value;
use crate::elasticsearch::*;

pub fn live2vod_workflow() -> Workflow {

  Workflow(vec!(
    Stage::new("s3event", stage1_json()),
    Stage::new("copyrendition", stage2_json()),
    Stage::new("renditioncopycomplete", stage3_json())
    )
  )
}


pub fn stage1_json() -> Value {

  let matchers: Vec<Must> = vec!(
    get_match("appname", "live2vod-lambdas"),
    get_match("lambda_function", "cdt-live2vod-s3event-lambda-prd"),
    get_match("message", "0/0000/0000#001"),
    get_match_phrase("message", "Sent SQS Message"),
  );

  query_from(1, &matchers, &TimeRange::Since(Utc::now() - Duration::hours(1) - Duration::minutes(5)))
}

pub fn stage2_json() -> Value {

  let matchers: Vec<Must> = vec!(
    get_match("appname", "live2vod-lambdas"),
    get_match("lambda_function", "cdt-live2vod-copyrendition-lambda-prd"),
    get_match("message", "0/0000/0000#001"),
    get_match_phrase("message", "Sent SQS Message"),
  );

  query_from(1, &matchers, &TimeRange::Since(Utc::now() - Duration::hours(1) - Duration::minutes(5)))
}

pub fn stage3_json() -> Value {

  let matchers: Vec<Must> = vec!(
    get_match("appname", "live2vod-lambdas"),
    get_match("lambda_function", "cdt-live2vod-renditioncopycomplete-lambda-prd"),
    get_match("message", "0/0000/0000#001"),
    get_match_phrase("message", "Sent SQS Message"),
  );

  query_from(1, &matchers, &TimeRange::Since(Utc::now() - Duration::hours(1) - Duration::minutes(5)))
}




//#[cfg(test)]
//mod workflow_spec {
//
//  use super::*;
//
//  impl PartialEq for WorkFlow {
//    fn eq(&self, other: &WorkFlow) -> bool {
//      self.name == other.name
//    }
//  }
//
//  #[test]
//  fn return_an_empty_vec_when_no_files_exist() {
//    let response = list();
//
//    let expectation: Vec<WorkFlow> = vec!();
//
//    assert_eq!(response, expectation);
//
//  }
//
//}
//
