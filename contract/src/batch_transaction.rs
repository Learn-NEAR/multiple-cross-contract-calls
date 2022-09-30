use near_sdk::{env, log, near_bindgen, Promise, PromiseResult};

use crate::*;

#[near_bindgen]
impl Contract {
    /// A helper method to create a cross contract function call.
    fn create_promise() -> Promise {
        // Create a promise to call the donate function on the provided account ID
        // with the appropriate donation amount.
        Promise::new(HELLO_CONTRACT.parse().unwrap()).function_call(
            "get_greeting".to_owned(),
            vec![],
            0,
            XCC_GAS,
        )
    }

    /// Call the `hello-nearverse` smart contract an arbitrary number of times.
    pub fn call_multiple_times(&mut self, number_of_promises: u8) -> Promise {
        // We check for the number of calls. The calls cannot exceed the gas limitations of our
        // environment.
        assert!(
            number_of_promises < 60,
            r###"
            You can only create up to 60 (you passed {}) cross contract calls.
            Otherwise there is not enough gas.
            "###,
            number_of_promises
        );

        // We create an empty vector that is going to store all of the promises we create.
        let mut promises = vec![];

        // We create the appropriate number of promises.
        for _ in 0..number_of_promises {
            promises.push(Self::create_promise());
        }

        // We return the promises and chain a then callback at the end.
        promises
            .into_iter()
            // Here we join/_and_ all of the promises so that they get executed together.
            .reduce(|accumulated_promise, current_promise| accumulated_promise.and(current_promise))
            .unwrap()
            .then(Self::ext(env::current_account_id()).callback_dynamic(number_of_promises))
    }

    /// Public method that can only be called by the current account ID.
    /// We dynamically check the number of promises returned and log messages based on the promise
    /// results.
    #[private]
    pub fn callback_dynamic(&self, number_of_promises: u8) {
        for index in 0..number_of_promises {
            let result = env::promise_result(index.into());

            match result {
                PromiseResult::Failed => {
                    log!(format!("Promise number {index} failed."))
                }
                PromiseResult::NotReady => {
                    log!(format!("Promise number {index} is not ready yet."))
                }
                PromiseResult::Successful(value) => {
                    if let Ok(message) = near_sdk::serde_json::from_slice::<String>(&value) {
                        log!(format!(
                            "This is the result of call number {index} is: {message}"
                        ));
                    }
                }
            }
        }
    }
}
