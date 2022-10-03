use near_sdk::{env, log, near_bindgen, serde_json::json, Promise, PromiseError};

use crate::*;

#[near_bindgen]
impl Contract {
  pub fn batch_actions(&mut self) -> Promise {
    let hi = json!({ "greeting": "hi".to_string() });
    let bye = json!({ "greeting": "bye".to_string() });

    // You can create one transaction calling multiple methods
    // on a same contract
    Promise::new(self.hello_account.clone())
      .function_call(
        "set_greeting".to_owned(),
        hi.to_string().into_bytes().to_vec(),
        NO_DEPOSIT,
        XCC_GAS,
      )
      .function_call("get_greeting".to_owned(), NO_ARGS, NO_DEPOSIT, XCC_GAS)
      .function_call(
        "set_greeting".to_owned(),
        bye.to_string().into_bytes().to_vec(),
        NO_DEPOSIT,
        XCC_GAS,
      )
      .function_call("get_greeting".to_owned(), NO_ARGS, NO_DEPOSIT, XCC_GAS)
      .then(Self::ext(env::current_account_id()).batch_actions_callback())
  }

  #[private]
  pub fn batch_actions_callback(
    &self,
    #[callback_result] last_result: Result<String, PromiseError>,
  ) -> String {
    // The callback only has access to the last action's result
    let mut greeting = "".to_string();

    if let Ok(result) = last_result {
      greeting = result;
      log!(format!("The last result is {greeting}"));
    } else {
      log!("The batch call failed and all calls got reverted");
    };

    greeting
  }
}