mod elasticsearch;
mod workflow;
mod server;
mod config;

fn main() {

  let cfg = config::Config("com", "ajsworton", "workflowstatus");

  let addr = [0, 0, 0, 0];
  let port = 5000;

  server::serve(addr, port);
}
