mod elasticsearch;
mod workflow;
mod server;
mod query;

use elasticsearch::ElasticQueryExecutor;
use crate::query::WorkflowExecutor;
use crate::workflow::live2vod_workflow;
//mod config;

#[macro_use] extern crate failure;

fn main() {

  let query_executor = ElasticQueryExecutor::new();
  let workflow_executor = WorkflowExecutor::new(query_executor);
  let res = workflow_executor.run(&live2vod_workflow());

  match res {
    Ok(Some(idx)) => println!("reached stage idx {}", idx.stage_idx + 1),
    r => println!("Couldn't detect much.. sorry: {:?}", r),
  }

}
