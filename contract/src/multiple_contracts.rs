use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::{env, log, near_bindgen, AccountId, Promise, PromiseError};

use crate::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PostedMessage {
  pub premium: bool,
  pub sender: AccountId,
  pub text: String,
}

#[near_bindgen]
impl Contract {
  /// A method which calls different contracts via cross contract function calls.
  pub fn multiple_contracts(&mut self) -> Promise {
    // We create a promise that calls the `get_greeting` function on the HELLO_CONTRACT
    let hello_promise = Promise::new(self.hello_account.clone()).function_call(
      "get_greeting".to_owned(),
      vec![],
      0,
      XCC_GAS,
    );

    // We create a promise that calls the `get_num` function on the COUNTER_CONTRACT
    let counter_promise = Promise::new(self.counter_account.clone()).function_call(
      "get_num".to_owned(),
      vec![],
      0,
      XCC_GAS,
    );

    // We create a promise that calls the `` function on the GUESTBOOK_CONTRACT
    let args = json!({ "from_index": "0".to_string(), "limit":2 });

    let guestbook_promise = Promise::new(self.guestbook_account.clone()).function_call(
      "get_messages".to_owned(),
      args.to_string().into_bytes().to_vec(),
      0,
      XCC_GAS,
    );

    // We join all promises and chain a callback to collect their results.
    hello_promise
      .and(counter_promise)
      .and(guestbook_promise)
      .then(
        Self::ext(env::current_account_id())
          .with_static_gas(XCC_GAS)
          .multiple_contracts_callback(),
      )
  }

  #[private]
  pub fn multiple_contracts_callback(
    &self,
    #[callback_result] hello_result: Result<String, PromiseError>,
    #[callback_result] counter_result: Result<i8, PromiseError>,
    #[callback_result] guestbook_result: Result<Vec<PostedMessage>, PromiseError>,
  ) -> (String, i8, Vec<PostedMessage>) {
    let mut greeting: String = "".to_string();
    let mut counter: i8 = 0;
    let mut messages: Vec<PostedMessage> = vec![];

    if let Ok(result) = hello_result {
      greeting = result;
      log!(format!("HelloNear says {greeting}"));
    } else {
      log!("The call to HelloNear failed");
    };

    if let Ok(result) = counter_result {
      counter = result;
      log!(format!("Counter is {counter}"));
    } else {
      log!("The call to Counter failed");
    };

    if let Ok(result) = guestbook_result {
      messages = result;
      log!(format!("The messages are {messages:?}"));
    } else {
      log!("The call to GuestBook failed");
    };

    (greeting, counter, messages)
  }
}
