use std::fmt;
#[derive(Debug)]
pub enum Event {
    Join,
    Custom(String),
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        match self {
            &Event::Join => write!(f, "{}", "phx_join"),
            &Event::Custom(ref x) => write!(f, "{}", x)
        }
    }
}

/*impl Decodable for Event {
    fn decode<D: Decoder>(d: &mut D) -> Result<Event, D::Error> {
        let event: Option<String> = try!(d.read_struct_field("event", 0, |d| Decodable::decode(d)));
        match event {
            Some(event) => {
                match event.as_ref() {
                    "heartbeat" => Ok(Event::Heartbeat),
                    "phx_close" => Ok(Event::Close),
                    "phx_error" => Ok(Event::Error),
                    "phx_join" => Ok(Event::Join),
                    "phx_leave" => Ok(Event::Leave),
                    "presence_diff" => Ok(Event::PresenceDiff),
                    "presence_state" => Ok(Event::PresenceState),
                    "phx_reply" => Ok(Event::Reply),
                    other => Ok(Event::Custom(other.to_string()))
                }
            }
            None => Ok(Event::Custom("unknown".to_string())),
        }
    }
}*/

#[test]
fn it_works() {
    //let val = serde_json::to_string(&Event::Custom("blablabla".to_string())).unwrap();
    let val = Event::Custom("blablabla".to_string()).to_string();
    println!("{}", val);
    assert_eq!(val, "blablabla");
}