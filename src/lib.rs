use node_bindgen::derive::node_bindgen;

mod keys;

use rdev::{listen, Event};

/// Handles callback event for user input events
#[node_bindgen(name = "rsHook")]
async fn rs_hook<F: Fn(Vec<String>) + 'static>(func: F) {
  
  pub fn pre_handle_event(event: Event) -> Vec<String> {
    let s;
    let c: String;
    let t;
  
    match event.event_type {
      rdev::EventType::KeyPress(_key) => {
        s = format!("{:?}", _key );
        c = keys::keys::scan_code(_key);
        t = "keydown";
      }
      
      rdev::EventType::KeyRelease(_key) => {
        s = format!("{:?}", _key );
        c = keys::keys::scan_code(_key);
        t = "keyup";
      }
  
      rdev::EventType::ButtonPress(_button) => {
        s = format!("{:?}", _button );
        c = "0".to_string();
        t = "mousedown";
      }
        
      rdev::EventType::ButtonRelease(_button) => {
        s = format!("{:?}", _button );
        c = "0".to_string();
        t = "mouseup";
      }
  
      _ => {
        s = "Error".to_string();
        c = "".to_string();
        t = "Error";
      }
    }
  
    vec![
      (&t).to_string(),
      (&c).to_string(),
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
