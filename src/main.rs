mod elasticsearch;
mod workflow;
mod server;
mod query;
//mod config;

#[macro_use] extern crate failure;

fn main() {

//  let cfg = config::Config("com", "ajsworton", "workflowstatus");

//  let  stage1_matchers = vec!()

  elasticsearch::test_json();

//  let addr = [0, 0, 0, 0];
//  let port = 5000;

//  server::serve(addr, port);
}
