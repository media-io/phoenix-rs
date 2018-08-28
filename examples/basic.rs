extern crate env_logger;
extern crate phoenix;
extern crate serde_json;
extern crate tokio_core;
extern crate websocket;

use phoenix::{Event, Phoenix};
use std::{thread, time};
use tokio_core::reactor::Core;
use websocket::futures::sync::mpsc::channel;
use websocket::futures::Stream;

fn main() {
  env_logger::init();

  let url = "ws://localhost:4000/socket";

  let (sender, emitter) = channel(0);
  let (callback, messages) = channel(0);

  // Simulate a user
  thread::spawn(move || {
    let mut phx = Phoenix::new(&sender, emitter, &callback, url);
    let mutex_chan = phx.channel("room:lobby").clone();

    {
      let mut device_chan = mutex_chan.lock().unwrap();
      device_chan.join();
    }

    let runner = messages.for_each(|message| {
      println!("user1: {:?}", message);
      Ok(())
    });

    let mut core = Core::new().unwrap();
    core.run(runner).unwrap();
  });

  thread::sleep(time::Duration::from_millis(500));

  // Simulate an other user

  let (sender, emitter) = channel(0);
  let (callback, messages) = channel(0);
  let mut phx = Phoenix::new(&sender, emitter, &callback, url);
  let mutex_chan = phx.channel("room:lobby").clone();

  {
    let mut device_chan = mutex_chan.lock().unwrap();
    device_chan.join();
    let body = serde_json::from_str(r#"{"body": "Hello"}"#).unwrap();
    device_chan.send(Event::Custom("new_msg".to_string()), &body);
  }

  let runner = messages.for_each(|message| {
    println!("user2: {:?}", message);
    Ok(())
  });

  let mut core = Core::new().unwrap();
  core.run(runner).unwrap();
}
