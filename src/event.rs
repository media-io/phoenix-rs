use serde_json;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Event {
  Defined(PhoenixEvent),
  Custom(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PhoenixEvent {
  #[serde(rename = "phx_join")]
  Join,
  #[serde(rename = "phx_close")]
  Close,
  #[serde(rename = "phx_reply")]
  Reply,
  #[serde(rename = "heartbeat")]
  Heartbeat,
}

#[derive(Serialize, Deserialize)]
pub struct Test {
  event: Event,
}

#[test]
fn test_event_serialization() {
  let t = Test { event: Event::Custom("blablabla".to_string()) };
  let val = serde_json::to_string(&t).unwrap();
  println!("{}", val);
  assert_eq!(val, "{\"event\":\"blablabla\"}");
}
