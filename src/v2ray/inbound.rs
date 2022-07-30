use serde::{Deserialize, Serialize};
pub(crate) mod client;
use client::Client;

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Settings {
  clients: Vec<Client>,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Inbound {
  port: u16,
  listen: String,
  protocol: String,
  pub settings: Settings,
}

impl Inbound {
  pub fn new() -> Self {
    Default::default()
  }
}
