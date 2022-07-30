use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Client {
  id: String,
  alterId: u8,
  email: String,
}

impl Client {
  pub fn new() -> Self {
    Default::default()
  }
}
