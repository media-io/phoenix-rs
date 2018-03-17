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

use phoenix::{Phoenix, Event};

fn main() {
    let url = "ws://localhost:4000/socket";
    let mut phoenix = Phoenix::new(url);
    let mutex_chan = phoenix.channel("room").clone();

    {
        let mut device_chan = mutex_chan.lock().unwrap();
        device_chan.join();
        device_chan.send(Event::User("get_home_id".to_string()), json!({}));
    }

    loop {
        let msg = phoenix.out.recv().unwrap();
        println!("{:?}", msg);
    }
}
```

```
RUST_LOG=debug cargo run
```