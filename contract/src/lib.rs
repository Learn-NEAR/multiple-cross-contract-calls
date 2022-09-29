use near_sdk::{
    assert_one_yocto,
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, log, near_bindgen, AccountId, Gas, Promise,
};

const XCC_GAS: Gas = Gas(5 * 10u64.pow(13));

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    /// Donate an equal portion of the attached deposit to the passed array of account IDs.
    /// This method calls the `donate` function on each of the passed account IDs using a cross
    /// contract call (which is a `Promise`)
    #[payable]
    pub fn donate(&mut self, account_ids: Vec<AccountId>) -> Promise {
        assert_one_yocto();

        assert!(
            account_ids.len() < 60,
            r###"
            You can only donate to up to 60 accounts (you passed {}),
            there is not enough gas for more calls.
            "###,
            account_ids.len(),
        );

        // Make sure all of the account IDs passed are valid
        account_ids.iter().for_each(|account_id| {
            assert!(
                env::is_valid_account_id(account_id.to_string().as_bytes()),
                "Invalid account id {}",
                account_id
            );
        });

        // Calculate the individual donation amounts.
        let donation = env::attached_deposit() / account_ids.len() as u128;

        account_ids
            .into_iter()
            .map(|account_id| {
                // Create a promise for calling this contract to notify the completion of a
                // cross contract call.
                let then_promise =
                    Self::ext(env::current_account_id()).log_after_donation(&account_id, donation);

                // Create a promise to call the donate function on the provided account ID
                // with the appropriate donation amount.
                Promise::new(account_id)
                    .function_call("donate".to_owned(), vec![], donation, XCC_GAS)
                    .then(then_promise)
            })
            // Join all the promises into a single promise using the `and` method.
            .reduce(|accumulated_promise, current_promise| accumulated_promise.and(current_promise))
            .unwrap()
    }

    /// Method that can only be called by the current_account_id (same smart contract).
    /// Used to log the completion of a cross contract donation.
    #[private]
    pub fn log_after_donation(&self, account_id: &AccountId, donation: u128) {
        log!(format!("Donated {donation} to {account_id}"));
    }

    /// Method that _fake_ donates an equal proportion of the attached deposit to the passed array
    /// of account IDs.
    /// This method performs a similar function as the `donate` method above, but instead of
    /// calling another smart contract it calls a private method on the same smart contract.
    #[payable]
    pub fn log(&mut self, account_ids: Vec<AccountId>) {
        // Calculate the individual donation amounts.
        let donation = env::attached_deposit() / account_ids.len() as u128;

        account_ids
            .iter()
            // Call the `log_once` method to _fake_ donate.
            .for_each(|account_id| self.log_once(account_id, donation));
    }

    /// Private method to _fake_ donate to a specific account ID.
    fn log_once(&self, account_id: &AccountId, donation: u128) {
        log!(format!("Fake donating {donation} to {account_id}"));
    }
}
