use serde::{Deserialize, Serialize};
use serde_json::Value;
mod inbound;
use inbound::client::Client;
use inbound::Inbound;

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct V2ray {
  log: Value,
  stats: Value,
  api: Value,
  policy: Value,
  allocate: Value,
  inbounds: Vec<Inbound>,
  outbounds: Vec<Value>,
  routing: Value,
}

impl V2ray {
  pub fn from_buf(slice: &[u8]) -> V2ray {
    match serde_json::from_slice(slice) {
      Ok(v) => v,
      Err(e) => panic!("Deserialize From Buf Fail: {}", e),
    }
  }

  pub fn add_client(v2ray: V2ray, client: Client) -> V2ray {
    let def_inbound = &Inbound::new();
    let inbound = match v2ray.inbounds.get(0) {
      Some(c) => c,
      None => def_inbound,
    };
    // TODO fix follow error
    //inbound.settings;
    v2ray
  }
  pub fn remove_client(v2ray: V2ray, client: Client) -> V2ray {
    v2ray
  }
}
