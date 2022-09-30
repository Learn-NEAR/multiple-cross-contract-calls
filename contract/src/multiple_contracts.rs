use near_sdk::{env, log, near_bindgen, Promise, PromiseResult};

use crate::*;

#[near_bindgen]
impl Contract{
  /// A method which calls different contracts via cross contract function calls.
  pub fn call_multiple_contracts(&mut self) -> Promise {
    // We create a promise that calls the `get_greeting` function on the HELLO_CONTRACT
    let hello_promise = Promise::new(self.hello_account)
    .function_call(
      "get_greeting".to_owned(),
      vec![],
      0,
      XCC_GAS,
    );

    // We create a promise that calls the `get_num` function on the COUNTER_CONTRACT
    let counter_promise = Promise::new(self.counter_account)
    .function_call(
      "get_num".to_owned(),
      vec![],
      0,
      XCC_GAS,
    );

    // We create a promise that calls the `` function on the GUESTBOOK_CONTRACT
    let guestbook_promise = Promise::new(self.guestbook_account)
    .function_call(
      "".to_owned(),
      vec![],
      0,
      XCC_GAS,
    );

    // We join all promises and chain a callback to collect their results.
    hello_promise
    .and(counter_promise)
    .and(guestbook_promise)
    .then(
      Self::ext(env::current_account_id()).callback()
    )
  }

  #[private] // can only be called by `current_account_id` (this contract).
  pub fn callback(&self,
                  #[callback_result] hello_result: Result<String, PromiseError>,
                  #[callback_result] counter_result: Result<i8, PromiseError>,
                  #[callback_result] guestbook_result: Result<String, PromiseError>
                ) {
    
    if let Ok(result) = hello_result {
      log!(format("HelloNear says {result}"));
    }else{
      log!("The call to HelloNear failed");
    }

    if let Ok(result) = hello_result {
      log!(format("HelloNear says {result}"));
    }else{
      log!("The call to HelloNear failed");
    }

    if let Ok(result) = hello_result {
      log!(format("HelloNear says {result}"));
    }else{
      log!("The call to HelloNear failed");
    }
    

  }
}