# Phoenix Channels client for Rust

This library is an attempt to create Phoenix client in Rust.
It is a work in progress so don't use it in production.

This crate is tested with https://github.com/laibulle/phoenix_channel_demo

__example__

```
git clone git@github.com:laibulle/phoenix_channel_demo.git
cd phoenix_channel_demo
mix deps.get
mix phx.server
```


```rust
extern crate phoenix;
#[macro_use]
extern crate serde_json;

extern crate env_logger;

use std::{thread, time};
use phoenix::{Phoenix, Event};

fn main() {
    env_logger::init();
    
    let url = "ws://localhost:4000/socket";

    // Simulate a user
    thread::spawn(move || {
        let mut phx = Phoenix::new(url);
        let mutex_chan = phx.channel("room:lobby").clone();

        {
            let mut device_chan = mutex_chan.lock().unwrap();
            device_chan.join();
        }

        loop {
            match phx.out.recv() {
                Ok(msg) => println!("user1: {:?}", msg),
                Err(_err) => ()//println!("{:?}", err)
            }
        }
    });

    thread::sleep(time::Duration::from_millis(500));

    // Simulate an other user
    let mut phx = Phoenix::new(url);
    let mutex_chan = phx.channel("room:lobby").clone();

    {
        let mut device_chan = mutex_chan.lock().unwrap();
        device_chan.join();
        device_chan.send(Event::Custom("new_msg".to_string()), serde_json::from_str(r#"{"body": "Hello"}"#).unwrap());
    }

    loop {
        match phx.out.recv() {
            Ok(msg) => println!("user2: {:?}", msg),
            Err(_err) => ()//println!("{:?}", err)
        }
    }
}

```

```
RUST_LOG=debug cargo run
```