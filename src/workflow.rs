#[derive(Debug)]
pub struct WorkFlow {
  name: String,
}

pub fn list() -> Vec<WorkFlow> {
  vec!()
}




#[cfg(test)]
mod workflow_spec {

  use super::*;

  impl PartialEq for WorkFlow {
    fn eq(&self, other: &WorkFlow) -> bool {
      self.name == other.name
    }
  }

  #[test]
  fn return_an_empty_vec_when_no_files_exist() {
    let response = list();

    let expectation: Vec<WorkFlow> = vec!();

    assert_eq!(response, expectation);

  }

}

