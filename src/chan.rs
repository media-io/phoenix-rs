use serde_json;
use serde_json::Value;
use websocket::OwnedMessage;

use websocket::futures::sync::mpsc::Sender;
use websocket::futures::Sink;

use event::{Event, PhoenixEvent};
use message::Message;

pub struct Channel {
  topic: String,
  reference: String,
  sender: Sender<OwnedMessage>,
}

impl Channel {
  pub fn new(topic: &str, sender: Sender<OwnedMessage>, reference: &str) -> Channel {
    Channel {
      topic: topic.to_owned(),
      reference: reference.to_owned(),
      sender,
    }
  }

  pub fn send_message(&mut self, message: OwnedMessage) {
    self.sender.clone().wait().send(message).unwrap();
  }

  fn build_message(&mut self, event: Event, payload: Value) -> OwnedMessage {
    let message = Message {
      topic: self.topic.to_owned(),
      event,
      reference: Some(self.reference.to_owned()),
      join_ref: Some(self.reference.to_owned()),
      payload,
    };
    OwnedMessage::Text(serde_json::to_string(&message).unwrap())
  }

  pub fn send(&mut self, event: Event, msg: &Value) {
    let message = self.build_message(event, msg.to_owned());
    self.send_message(message);
  }

  pub fn join(&mut self) {
    let message = self.build_message(Event::Defined(PhoenixEvent::Join), Value::Null);
    self.send_message(message);
  }

  pub fn join_with_message(&mut self, payload: Value) {
    let message = self.build_message(Event::Defined(PhoenixEvent::Join), payload);
    self.send_message(message);
  }
}
