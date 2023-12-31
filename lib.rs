#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod game {

    use token::TokenRef;

    #[ink(event)]
    pub struct Won{
           won: bool,
           account: AccountId
        
    }

    #[ink(storage)]
    pub struct Game {
        /// Stores a single `bool` value on the storage.
        secret: u32,
        token: TokenRef,
    }

    impl Game {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(secret: u32, token_hash: Hash) -> Self {
            let token = TokenRef::new(500)
             .code_hash(token_hash)
             .endowment(0)
             .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
            .instantiate();

        Self { secret, token}

        }

      pub fn mint(&self){
        TokenRef::new(500);
      }

        #[ink(message)]
        pub fn play(&mut self, number: u32) {
            if number == self.secret {   
                self.token.transfer(self.env().caller(), 100);      
                self.env().emit_event(
                Won{
                    won: true,
                    account: self.env().caller(),
                }
            )
        }
    }

    }


}
