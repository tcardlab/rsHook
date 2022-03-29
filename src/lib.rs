use node_bindgen::derive::node_bindgen;

mod keys;
use keys::{scan_code, index};

use rdev::{listen, Event, 
  EventType::{KeyPress, KeyRelease, ButtonPress, ButtonRelease}
};

use std::time::SystemTime;
const ZERO:SystemTime = SystemTime::UNIX_EPOCH;


/// Handles callback event for user input events
#[node_bindgen(name = "rsHook")]
async fn rs_hook<F: Fn(Vec<String>) + 'static>(func: F) {
  
  fn pre_handle_event(event: Event) -> Vec<String> {
    let (s, c, t);

    // println!("{:?}", event);
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
  
    let time = event.time.duration_since(ZERO).unwrap().as_millis();
    vec![ 
      (&t).to_string(),
      (&c).to_string(),
      time.to_string(),
      (&s).to_string() 
    ]
  }

  let curry_event = move | event: Event | {
    let action = pre_handle_event(event);
    if action[0] != "Error" {
      func(action)
    }
  };

  if let Err(error) = listen(curry_event) {
    println!("Error: {:?}", error)
  }
}
