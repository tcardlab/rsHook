#![deny(clippy::all)]

mod keys;

#[macro_use]
extern crate napi_derive;

use napi::{JsFunction, Result, JsString};
use napi::{threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode, ThreadSafeCallContext}};

use rdev::{listen, Event, Button};

use std::time::SystemTime;

fn index(button: Button) -> &'static str {
  match button {
    Button::Left => "0",
    Button::Right => "1",
    Button::Middle => "2",
    _=> ""
  }
}

fn pre_handle_event(event: Event) -> Vec<String> {
  let s;
  let c;
  let t;
  // println!("{:?}", event);
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
      c = index(_button);
      t = "mousedown";
    }
      
    rdev::EventType::ButtonRelease(_button) => {
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

  let time = event.time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
  vec![ (&t).to_string(), (&c).to_string(), time.to_string(), (&s).to_string() ]
}


#[napi(js_name = "rsHook")]
fn rs_hook(callback: JsFunction, is_thread_safe: Option<bool>) -> Result<()> {

  let tsfn: ThreadsafeFunction<Vec<String>, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx:ThreadSafeCallContext< Vec<String> >| {
      ctx.value.iter().map(|v| ctx.env.create_string(&*v)).collect::<Result<Vec<JsString>>>()
    })?;

  let curry_event = move | event: Event | {
    let action = pre_handle_event(event);
    if action[0] != "Error" {
      // println!("Rust Log: {:?}", action);
      tsfn.call( Ok(action), ThreadsafeFunctionCallMode::NonBlocking);
    }
  };

  if is_thread_safe.unwrap_or(true) {
    use std::thread;
    thread::spawn(move || {
      if let Err(error) = listen(curry_event) {
        println!("Error: {:?}", error)
      }
    });
  } else {
    if let Err(error) = listen(curry_event) {
      println!("Error: {:?}", error)
    }
  }
  
  Ok(())
}
