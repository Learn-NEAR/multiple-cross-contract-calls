use near_sdk::serde_json::json;
use near_sdk::{env, log, near_bindgen, Promise, PromiseResult};

use crate::*;

#[near_bindgen]
impl Contract {
  fn promise_set_get(&self, message: String) -> Promise {
    let args = json!({ "greeting": message });

    Promise::new(self.hello_account.clone())
      .function_call(
        "set_greeting".to_owned(),
        args.to_string().into_bytes().to_vec(),
        0,
        XCC_GAS,
      )
      .function_call("get_greeting".to_owned(), vec![], 0, XCC_GAS)
  }

  pub fn similar_contracts(&mut self) -> Promise {
    // Create promises to call 3 contracts that return the same type
    let hello_one = self.promise_set_get("hi".to_owned());
    let hello_two = self.promise_set_get("howdy".to_owned());
    let hello_three = self.promise_set_get("bye".to_owned());

    // Join all promises and chain a callback to collect their results.
    hello_one
      .and(hello_two)
      .and(hello_three)
      .then(
        Self::ext(env::current_account_id())
          .with_static_gas(XCC_GAS)
          .similar_contracts_callback(),
    )
  }

  #[private]
  pub fn similar_contracts_callback(&self) -> Vec<String> {
    let mut results: Vec<String> = vec![];

    for index in 0..3 {
      // env::promise_result(i) has the result of the i-th call
      let result = env::promise_result(index);

      match result {
        PromiseResult::Failed => {
          log!(format!("Promise number {index} failed."));
        }
        PromiseResult::NotReady => {
          log!(format!("Promise number {index} is not ready yet."));
        }
        PromiseResult::Successful(value) => {
          if let Ok(message) = near_sdk::serde_json::from_slice::<String>(&value) {
            results.push(message.clone());
            log!(format!("Call {index} returned: {message}"));
          } else {
            log!(format!("Error deserializing call {index} result."));
          }
        }
      }
    }
    results
  }
}
