use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, log, near_bindgen,
    serde::Deserialize,
    AccountId, Gas, Promise, PromiseResult,
};

mod dynamic_callbacks;

const XCC_GAS: Gas = Gas(5 * 10u64.pow(13));
const HELLO_CONTRACT: &str = "hello-nearverse.testnet";
const COUNTER_CONTRACT: &str = "counter-nearverse.testnet";
const GUESTBOOK_CONTRACT: &str = "guestbook-nearverse.testnet";

#[derive(Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract;

// Implement the contract structure
#[near_bindgen]
impl Contract {
    /// A method which calls different contracts via cross contract function calls.
    pub fn call_multiple_contracts(&mut self) -> Promise {
        // We create a promise that calls the `get_greeting` function on the HELLO_CONTRACT
        let hello_promise = Promise::new(HELLO_CONTRACT.parse().unwrap()).function_call(
            "get_greeting".to_owned(),
            vec![],
            0,
            XCC_GAS,
        );

        // We create a promise that calls the `get_num` function on the COUNTER_CONTRACT
        let counter_promise = Promise::new(COUNTER_CONTRACT.parse().unwrap()).function_call(
            "get_num".to_owned(),
            vec![],
            0,
            XCC_GAS,
        );

        // We create a promise that calls the `` function on the GUESTBOOK_CONTRACT
        let guestbook_promise = Promise::new(GUESTBOOK_CONTRACT.parse().unwrap()).function_call(
            "get_messages".to_owned(),
            vec![],
            0,
            XCC_GAS,
        );

        // Here we join/_and_ all of the different promises and chain a callback that will collect
        // all of the results.
        // Keep in mind that we can join/_and_ arbitrary promises together.
        hello_promise
            .and(counter_promise)
            .and(guestbook_promise)
            .then(Self::ext(env::current_account_id()).callback())
    }

    /// Method that can only be called by the current_account_id (same smart contract).
    /// Used to log the completion of the cross contract calls.
    #[private]
    pub fn callback(&self) {
        // We read the result of the first promise.
        let hello_result = env::promise_result(0);

        // Handle the result of the promise.
        match hello_result {
            PromiseResult::Failed => {
                log!("hello-nearverse call failed");
            }
            PromiseResult::NotReady => {
                log!("hello-nearverse promise not ready yet");
            }
            PromiseResult::Successful(hello_value) => {
                if let Ok(message) = near_sdk::serde_json::from_slice::<String>(&hello_value) {
                    log!(format!(
                        "This is the result of the get_greeting call: {message}"
                    ));
                } else {
                    log!("There was an error deserializeing the value from the get_greeting call.");
                }
            }
        }

        // We read the result of the second promise.
        let counter_result = env::promise_result(1);

        // Handle the result of the promise.
        match counter_result {
            PromiseResult::Failed => {
                log!("counter-nearverse call failed");
            }
            PromiseResult::NotReady => {
                log!("counter-nearverse promise not ready yet");
            }
            PromiseResult::Successful(counter_value) => {
                if let Ok(num) = near_sdk::serde_json::from_slice::<i64>(&counter_value) {
                    log!(format!("This is the result of the get_num call: {num}"));
                } else {
                    log!("There was an error deserializeing the value from the get_num call.");
                }
            }
        }

        // We read the result of the third promise.
        let guestbook_result = env::promise_result(2);

        // Handle the result of the promise.
        match guestbook_result {
            PromiseResult::Failed => {
                log!("guestbook-nearverse call failed");
            }
            PromiseResult::NotReady => {
                log!("guestbook-nearverse promise not ready yet");
            }
            PromiseResult::Successful(guestbook_value) => {
                if let Ok(messages) =
                    near_sdk::serde_json::from_slice::<Vec<PostedMessage>>(&guestbook_value)
                {
                    log!(format!(
                        "This is the result of the get_messages call: {messages:?}"
                    ));
                } else {
                    log!("There was an error deserializeing the value from the get_messages call.");
                }
            }
        }
    }
}
