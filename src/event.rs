use std::fmt;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Event {
    Defined(PhoenixEvent),
    Custom(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PhoenixEvent {
    #[serde(rename="phx_join")]
    Join,
    #[serde(rename="phx_close")]
    Close
}

#[derive(Serialize, Deserialize)]
pub struct Test {
    event: Event
}


#[test]
fn test_event_serialization() {
    let t = Test{event: Event::Custom("blablabla".to_string())};
    let val = serde_json::to_string(&t).unwrap();
    println!("{}", val);
    assert_eq!(val, "{\"event\":\"blablabla\"}");
}