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
