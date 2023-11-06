use tari_template_lib::prelude::*;

#[template]
mod superbru_template {
    use super::*;
    use tari_template_abi::rust::collections::BTreeMap;

    pub struct SuperBruPool {
        treasury: Vault,
        // allowed_accounts: Vec<ComponentAddress>,
        team_1: String,
        team_2: String,
        prediction_difference: BTreeMap<RistrettoPublicKeyBytes, u64>,
    }

    impl SuperBruPool {
        pub fn create_pool(
            token_symbol: String,
            team_1: String,
            team_2: String,
            num_players: u64,
        ) -> Self {
            let members = ResourceBuilder::fungible(&token_symbol)
                .initial_supply(Amount(num_players as i64))
                .build_bucket();

            // let resource_address = coins.resource_address();
            Self {
                // allowed_accounts: vec![],
                treasury: Vault::from_bucket(members),
                team_1,
                team_2,
                prediction_difference: HashMap::new(),
            }
        }

        pub fn make_prediction(&mut self, difference: u64, mut membership: Bucket) {
            // if membership.resource_address() != self.treasury.resource_address() {
            // error("Not the right resource");
            // panic!("error1");
            // }
            // if membership.amount() != Amount(1) {
            // error("Not enough to play");
            // panic!("error2");
            // }
            // burn the token
            // membership.burn();
            self.prediction_difference
                .insert(CallerContext::caller(), difference);
        }

        // pub fn add_account(&mut self, account: ComponentAddress) {
        //     // if self.disallowed_accounts.contains(account) {
        //     // panic!("Not allowed to add");
        //     // }
        //     if !self.allowed_accounts.contains(&account) {
        //         self.allowed_accounts.push(account);
        //     }
        // }

        // pub fn remove_account(&mut self, account: ComponentAddress) {
        //     self.allowed_accounts.retain(|a| a != &account);
        // }

        // pub fn transfer(&self, from: ComponentAddress, to: ComponentAddress, amount: u64) {
        //     let from_account = ComponentManager::get(from);
        //     if !self.allowed_accounts.contains(&from) {
        //         panic!("Not allowed to send");
        //     }

        //     // let to_account = ComponentManager::get(to);

        //     // let bucket = from_account.call(
        //     //     "withdraw".to_string(),
        //     //     vec![self.treasury.resource_address(), amount.to_le_bytes()],
        //     // );
        //     // to_account.call("deposit".to_string(), bucket);
        // }

        pub fn take_free_coins(&mut self) -> Bucket {
            debug("Withdrawing 1 coins from faucet");
            self.treasury.withdraw(Amount::new(1))
        }

        // pub fn burn_coins(&mut self, amount: Amount) {
        //     let mut bucket = self.treasury.withdraw(amount);
        //     bucket.burn();
        // }

        // pub fn total_supply(&self) -> Amount {
        //     ResourceManager::get(self.treasury.resource_address()).total_supply()
        // }
    }
}
