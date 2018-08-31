use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{thread, time};

use serde_json;
use websocket::client::ClientBuilder;
use websocket::OwnedMessage;

use chan::Channel;
use event::{Event, PhoenixEvent};
use message::Message as PhoenixMessage;
use tokio_core::reactor::Core;
use websocket::futures::sync::mpsc::{Receiver, Sender};
use websocket::futures::{Future, Sink, Stream};
use websocket::result::WebSocketError;

pub struct Phoenix {
  count: u8,
  channels: Arc<Mutex<Vec<Arc<Mutex<Channel>>>>>,
  sender: Sender<OwnedMessage>,
}

impl Phoenix {
  pub fn new(
    sender: &Sender<OwnedMessage>,
    receiver: Receiver<OwnedMessage>,
    callback: &Sender<PhoenixMessage>,
    url: &str,
  ) -> Phoenix {
    Phoenix::new_with_parameters(sender, receiver, callback, url, &HashMap::new())
  }

  pub fn new_with_parameters(
    sender: &Sender<OwnedMessage>,
    receiver: Receiver<OwnedMessage>,
    callback: &Sender<PhoenixMessage>,
    url: &str,
    params: &HashMap<&str, &str>,
  ) -> Phoenix {
    let full_url = if params.is_empty() {
      format!("{}/websocket", url)
    } else {
      let mut joined_params = "".to_owned();
      for (index, (key, value)) in params.iter().enumerate() {
        joined_params += if index == 0 { "?" } else { "&" };
        joined_params += key;
        joined_params += "=";
        joined_params += value;
      }
      format!("{}/websocket{}", url, joined_params)
    };

    debug!("connect socket to URL: {}", full_url);

    let copy_callback = callback.clone();

    thread::spawn(move || {
      let mut core = Core::new().unwrap();

      let runner = ClientBuilder::new(&full_url)
        .unwrap()
        .async_connect(None, &core.handle())
        .and_then(|(duplex, _)| {
          let (sink, stream) = duplex.split();
          stream
            .filter_map(|message| match message {
              OwnedMessage::Close(e) => {
                let message = PhoenixMessage {
                    topic: "phoenix".to_string(),
                    event: Event::Defined(PhoenixEvent::Close),
                    reference: None,
                    join_ref: None,
                    payload: serde_json::Value::Null,
                  };
                let _ = copy_callback.clone().wait().send(message);
                Some(OwnedMessage::Close(e))
              },
              OwnedMessage::Ping(d) => Some(OwnedMessage::Pong(d)),
              OwnedMessage::Text(content) => {
                let message: PhoenixMessage = serde_json::from_str(&content).unwrap();
                let _ = copy_callback.clone().wait().send(message);
                None
              }
              _ => None,
            })
            .select(receiver.map_err(|_| WebSocketError::NoDataAvailable))
            .forward(sink)
        });

      if let Err(msg) = core.run(runner) {
        let message = PhoenixMessage {
            topic: "phoenix".to_string(),
            event: Event::Defined(PhoenixEvent::Close),
            reference: None,
            join_ref: None,
            payload: serde_json::Value::Null,
          };
        let _ = copy_callback.clone().wait().send(message);
        error!("{:?}", msg);
      }
    });

    let tx = sender.clone();
    thread::spawn(move || loop {
      let mut stdin_sink = tx.clone().wait();
      let msg = PhoenixMessage {
        topic: "phoenix".to_owned(),
        event: Event::Defined(PhoenixEvent::Heartbeat),
        reference: None,
        join_ref: None,
        payload: serde_json::from_str("{}").unwrap(),
      };

      let message = OwnedMessage::Text(serde_json::to_string(&msg).unwrap());
      if stdin_sink.send(message).is_err() {
        error!("unable to send Heartbeat");
        break;
      }

      thread::sleep(time::Duration::from_secs(30));
    });

    let channels: Arc<Mutex<Vec<Arc<Mutex<Channel>>>>> = Arc::new(Mutex::new(vec![]));

    Phoenix {
      count: 0,
      channels: channels.clone(),
      sender: sender.clone(),
    }
  }

  pub fn channel(&mut self, topic: &str) -> Arc<Mutex<Channel>> {
    self.count += 1;
    let chan = Arc::new(Mutex::new(Channel::new(
      topic,
      self.sender.clone(),
      &format!("{}", self.count),
    )));
    let mut channels = self.channels.lock().unwrap();
    channels.push(chan.clone());
    chan
  }
}
