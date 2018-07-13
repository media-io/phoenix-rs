use std::sync::mpsc::Sender;
use websocket::OwnedMessage;
use serde_json::Value;
use serde_json;

use message::Message;
use event::{Event, PhoenixEvent};

pub struct Channel {
  topic: String,
  tx: Sender<OwnedMessage>,
  reference: String,
}

impl Channel {
  pub fn new(topic: &str, tx: Sender<OwnedMessage>, reference: &str) -> Channel {
    Channel {
      topic: topic.to_owned(),
      tx,
      reference: reference.to_owned(),
    }
  }

  pub fn send(&mut self, event: Event, msg: Value) {
    let msg = Message {
      topic: self.topic.to_owned(),
      event,
      reference: Some(self.reference.to_owned()),
      join_ref: Some(self.reference.to_owned()),
      payload: msg.to_owned(),
    };

    self
      .tx
      .send(OwnedMessage::Text(serde_json::to_string(&msg).unwrap()))
      .unwrap();
  }

  pub fn join(&mut self) {
    let msg = Message {
      topic: self.topic.to_owned(),
      event: Event::Defined(PhoenixEvent::Join),
      reference: Some(self.reference.to_owned()),
      join_ref: Some(self.reference.to_owned()),
      payload: Value::Null,
    };
    self
      .tx
      .send(OwnedMessage::Text(serde_json::to_string(&msg).unwrap()))
      .unwrap();
  }

  pub fn join_with_message(&mut self, payload: Value) {
    let msg = Message {
      topic: self.topic.to_owned(),
      event: Event::Defined(PhoenixEvent::Join),
      reference: Some(self.reference.to_owned()),
      join_ref: Some(self.reference.to_owned()),
      payload: payload,
    };
    self
      .tx
      .send(OwnedMessage::Text(serde_json::to_string(&msg).unwrap()))
      .unwrap();
  }
}
