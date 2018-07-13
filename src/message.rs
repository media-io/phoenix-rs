use serde_json::Value;
use event::Event;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
  pub topic: String,
  pub event: Event,
  #[serde(rename = "ref")]
  pub reference: Option<String>,
  pub join_ref: Option<String>,
  pub payload: Value,
}
