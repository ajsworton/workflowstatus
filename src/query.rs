use chrono::{Utc, DateTime, Duration};
use std::default::Default;
use failure::Error;
use serde_json::Value;

#[derive(Debug)]
pub enum TimeRange {
  Within(DateTime<Utc>, DateTime<Utc>),
  Since(DateTime<Utc>),
  Until(DateTime<Utc>),
}

impl Default for TimeRange {
  fn default() -> Self {
    TimeRange::Since(Utc::now() - Duration::hours(2))
  }
}

pub enum Query {
  ExactMatch(String),
  MatchAll(Vec<String>)
}

pub struct Stage {
  label: String,
  query: Value,
}

impl Stage {
  pub fn new(label: &str, query: Value) -> Stage {
    Stage { label: label.to_owned(), query }
  }
}

#[derive(Debug)]
pub struct Event {
  pub stage_idx: u8,
}

pub struct Workflow(pub Vec<Stage>);

pub type EventResult = Result<Option<Event>, Error>;


pub trait QueryExecutor {
  fn run(&self, stage_idx: u8, query: &Value) -> EventResult;
}

pub struct WorkflowExecutor<Q: QueryExecutor> {
  query_executor: Q,
}

impl <Q: QueryExecutor> WorkflowExecutor<Q> {
  pub fn new(query_executor: Q) -> Self {
    WorkflowExecutor {
      query_executor
    }
  }

  pub fn run(&self, workflow: &Workflow) -> EventResult {

    for (i, stage) in workflow.0.iter().rev().enumerate() {
      let maybe_event = self.query_executor.run(i as u8, &stage.query)?;
      if let Some(event) = maybe_event {
        return Ok(Some(event));
      }
    }

    Ok(None)
  }
}

