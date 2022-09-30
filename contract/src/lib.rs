use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, Gas};

mod dynamic_calls;
mod fixed_calls;

const XCC_GAS: Gas = Gas(5 * 10u64.pow(13));
const HELLO_CONTRACT: &str = "hello-nearverse.testnet";
const COUNTER_CONTRACT: &str = "counter-nearverse.testnet";
const GUESTBOOK_CONTRACT: &str = "guestbook-nearverse.testnet";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub hello_account: AccountId,
    pub counter_account: AccountId,
    pub guestbook_account: AccountId,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            hello_account: HELLO_CONTRACT.parse().unwrap(),
            counter_account: COUNTER_CONTRACT.parse().unwrap(),
            guestbook_account: GUESTBOOK_CONTRACT.parse().unwrap(),
        }
    }
}

#[near_bindgen]
impl Contract {
  #[init]
  #[private] // Public - but only callable by env::current_account_id()
  pub fn init(
      hello_account: AccountId,
      counter_account: AccountId,
      guestbook_account: AccountId,
  ) -> Self {
      Self {
          hello_account,
          counter_account,
          guestbook_account,
      }
  }
}
