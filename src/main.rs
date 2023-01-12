use rdev::{
  listen, 
  Event, 
  EventType::{KeyPress, KeyRelease, ButtonPress, ButtonRelease}
};

mod keys;
use keys::{scan_code, index};

use std::time::SystemTime;
const ZERO:SystemTime = SystemTime::UNIX_EPOCH;

fn main() {
  // This will block.
  if let Err(error) = listen(callback) {
    println!("Error: {:?}", error)
  }
}

fn callback(event: Event) {
  let (s, c, t);

  match event.event_type {
    KeyPress(_key) => {
        s = format!("{:?}", _key );
        c = scan_code(_key);
        t = "keydown";
      }
      
      KeyRelease(_key) => {
        s = format!("{:?}", _key );
        c = scan_code(_key);
        t = "keyup";
      }
  
      ButtonPress(_button) => {
        s = format!("{:?}", _button );
        c = index(_button);
        t = "mousedown";
      }
        
      ButtonRelease(_button) => {
        s = format!("{:?}", _button );
        c = index(_button);
        t = "mouseup";
      }
  
      _ => {
        s = "Error".to_string();
        c = "";
        t = "Error";
      }
  }

  if t != "Error" {
    let time = event.time.duration_since(ZERO).unwrap().as_millis();
    println!("{t},{c},{0},{1}", time.to_string(), (&s).to_string());
  }
}
