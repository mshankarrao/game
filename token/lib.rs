#![cfg_attr(not(feature = "std"), no_std)]

pub use self::token::{Token, TokenRef};

#[ink::contract]
pub mod token {

    use ink::storage::Mapping;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Token {
        balances: Mapping<AccountId,Balance>,     
        total_supply: Balance
    }

    impl Token {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            Self { balances, total_supply}
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Balance {
            self.balances.get(&account).unwrap_or_default()
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance){
            let from = self.env().caller();
            let from_balance = self.balance_of(from);
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let token = Token::new(100000);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut token = Token::new(100000);
        }
    }
}
