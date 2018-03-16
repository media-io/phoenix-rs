# Phoenix Channels client for Rust

This library is an attempt to create Phoenix client in Rust.
It is a work in progress so don't use it in production.

This crate is tested with https://github.com/laibulle/phoenix_channel_demo

__example__
```rust
extern crate phoenix;
#[macro_use]
extern crate serde_json;

use phoenix::{Phoenix};

fn main() {
    let url = "wss://staging.haum.io/socket";
    let mut phoenix = Phoenix::new(url);
    let mutex_chan = phoenix.channel("device:6a:00:02:5a:ca:10").clone();

    {
        let mut device_chan = mutex_chan.lock().unwrap();
        device_chan.join();
        device_chan.send("get_home_id", json!({}));
    }

    loop {
        let msg = phoenix.out.recv().unwrap();
        println!("{:?}", msg);
    }
}
```