
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use websocket::{Message, OwnedMessage};
use websocket::client::ClientBuilder;
use serde_json;

use chan::Channel;
use message::{Message as PhoenixMessage};
use event::{PhoenixEvent, Event};

pub struct Phoenix {
	tx: Sender<OwnedMessage>,
	count: u8,
	channels: Arc<Mutex<Vec<Arc<Mutex<Channel>>>>>,
	pub out: Receiver<PhoenixMessage>,
}

impl Phoenix {
	pub fn new(url: &str) -> Phoenix {
		Phoenix::new_with_parameters(url, &HashMap::new())
	}

	pub fn new_with_parameters(url: &str, params: &HashMap<&str, &str>) -> Phoenix {
		let full_url =
			if params.is_empty() {
				format!("{}/websocket", url)
			} else {
				let mut joined_params = "".to_owned();
				for (index, (key, value)) in params.iter().enumerate() {
					joined_params +=
						if index == 0 {
							"?"
						} else {
							"&"
						};
					joined_params += key;
					joined_params += "=";
					joined_params += value;
				}
				format!("{}/websocket{}", url, joined_params)
			};

		debug!("connect socket to URL: {}", full_url);

		let client = ClientBuilder::new(&full_url)
			.unwrap()
			.connect_insecure()
			.unwrap();

		let (mut receiver, mut sender) = client.split().unwrap();

		let (tx, rx) = channel();

		let tx_1 = tx.clone();

		thread::spawn(move || {
			loop {
				// Send loop
				let message = match rx.recv() {
					Ok(m) => {
						debug!("Send Loop: {:?}", m);
						m
					},
					Err(e) => {
						error!("Send Loop: {:?}", e);
						return;
					}
				};
				match message {
					OwnedMessage::Close(_) => {
						debug!("Received a close message");
						let _ = sender.send_message(&message);
						// If it's a close message, just send it and then return.
						return;
					}
					_ => (),
				}
				// Send the message
				match sender.send_message(&message) {
					Ok(()) => (),
					Err(e) => {
						error!("Send Loop: {:?}", e);
						let _ = sender.send_message(&Message::close());
						return;
					}
				}
			}
		});

		let channels: Arc<Mutex<Vec<Arc<Mutex<Channel>>>>> = Arc::new(Mutex::new(vec![]));
		let (send, recv) = channel();

		thread::spawn(move || {
			// Receive loop
			for message in receiver.incoming_messages() {
				let message = match message {
					Ok(m) => m,
					Err(e) => {
						error!("Receive Loop: {:?}", e);
						let _ = tx_1.send(OwnedMessage::Close(None));
						return;
					}
				};

				match message {
					OwnedMessage::Close(x) => {
						debug!("Received close {:?}", x);
						// Got a close message, so send a close message and return
						let _ = tx_1.send(OwnedMessage::Close(None));
						return;
					}
					
					OwnedMessage::Ping(data) => {
						match tx_1.send(OwnedMessage::Pong(data)) {
							// Send a pong in response
							Ok(()) => debug!("Received ping"),
							Err(e) => {
								error!("Ping: {:?}", e);
								return;
							}
						}
					}

					// Say what we received
					OwnedMessage::Text(data) => {
						let v: PhoenixMessage = serde_json::from_str(&data).unwrap();
						send.send(v);
					},
					
					message => debug!("Receive Loop: {:?}", message)
				}
			}
		});

        let tx_h = tx.clone();
        thread::spawn(move || {
            loop {

                let msg = PhoenixMessage {
                    topic: "phoenix".to_owned(),
                    event: Event::Defined(PhoenixEvent::Heartbeat),
                    reference: None,
                    join_ref: None,
                    payload: serde_json::from_str("{}").unwrap(),
                };


                tx_h
                    .send(OwnedMessage::Text(serde_json::to_string(&msg).unwrap()))
                    .unwrap();

                thread::sleep(time::Duration::from_secs(30));
            }
        });


		Phoenix{
			tx: tx.clone(),
			count: 0,
			channels: channels.clone(),
			out: recv,
		}
	}

	pub fn channel(&mut self, topic: &str) -> Arc<Mutex<Channel>> {
		self.count = self.count+1;
		let chan = Arc::new(Mutex::new(Channel::new(topic, self.tx.clone(), &format!("{}", self.count))));
		let mut channels = self.channels.lock().unwrap();
		channels.push(chan.clone());
		chan
	}
}