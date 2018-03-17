extern crate phoenix;
#[macro_use]
extern crate serde_json;

extern crate env_logger;

use std::{thread, time};
use phoenix::{Phoenix, Event};

fn main() {
    env_logger::init();
    
    let url = "ws://localhost:4000/socket";

    thread::spawn(move || {
        let mut phx = Phoenix::new(url);
        let mutex_chan = phx.channel("room:lobby").clone();

        {
            let mut device_chan = mutex_chan.lock().unwrap();
            device_chan.join();
            //device_chan.send(Event::User("get_home_id".to_string()), json!({}));
        }

        loop {
            match phx.out.recv() {
                Ok(msg) => println!("user1: {:?}", msg),
                Err(_err) => ()//println!("{:?}", err)
            }
        }
    });

    thread::sleep(time::Duration::from_millis(500));

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
