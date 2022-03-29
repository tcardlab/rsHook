#![deny(clippy::all)]

mod keys;
use keys::{scan_code, index};

#[macro_use]
extern crate napi_derive;

use napi::{JsFunction, Result, JsString};
use napi::{threadsafe_function::{
  ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode, ThreadSafeCallContext
}};

use rdev::{listen, Event, 
  EventType::{KeyPress, KeyRelease, ButtonPress, ButtonRelease}
};

use std::time::SystemTime;
const ZERO:SystemTime = SystemTime::UNIX_EPOCH;


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
