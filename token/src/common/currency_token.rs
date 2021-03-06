use std::collections::HashMap;

use ic_cdk::export::candid::{CandidType, Deserialize, Principal};

use crate::common::types::{Controllers, Error, TokenInfo};

#[derive(CandidType, Deserialize)]
pub struct CurrencyToken {
    pub balances: HashMap<Principal, u64>,
    pub total_supply: u64,
    pub info: TokenInfo,
    pub controllers: Controllers,
}

impl CurrencyToken {
    pub fn mint(&mut self, to: Principal, qty: u64) -> Result<(), Error> {
        if qty == 0 {
            return Err(Error::ZeroQuantity);
        }

        let prev_balance = self.balance_of(&to);
        let new_balance = prev_balance + qty;

        self.total_supply += qty;
        self.balances.insert(to, new_balance);

        Ok(())
    }

    pub fn transfer(&mut self, from: Principal, to: Principal, qty: u64) -> Result<(), Error> {
        if qty == 0 {
            return Err(Error::ZeroQuantity);
        }

        let prev_from_balance = self.balance_of(&from);
        let prev_to_balance = self.balance_of(&to);

        if prev_from_balance < qty {
            return Err(Error::InsufficientBalance);
        }

        let new_from_balance = prev_from_balance - qty;
        let new_to_balance = prev_to_balance + qty;

        if new_from_balance == 0 {
            self.balances.remove(&from);
        } else {
            self.balances.insert(from, new_from_balance);
        }

        self.balances.insert(to, new_to_balance);

        Ok(())
    }

    pub fn burn(&mut self, from: Principal, qty: u64) -> Result<(), Error> {
        if qty == 0 {
            return Err(Error::ZeroQuantity);
        }

        let prev_balance = self.balance_of(&from);

        if prev_balance < qty {
            return Err(Error::InsufficientBalance);
        }

        let new_balance = prev_balance - qty;

        if new_balance == 0 {
            self.balances.remove(&from);
        } else {
            self.balances.insert(from, new_balance);
        }

        self.total_supply -= qty;

        Ok(())
    }

    pub fn balance_of(&self, account_owner: &Principal) -> u64 {
        match self.balances.get(account_owner) {
            None => 0,
            Some(b) => *b,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::time::{SystemTime, UNIX_EPOCH};

    use ic_cdk::export::candid::Principal;

    use crate::common::currency_token::CurrencyToken;
    use crate::common::types::TokenInfo;

    pub fn random_principal_test() -> Principal {
        Principal::from_slice(
            &SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .to_be_bytes(),
        )
    }

    fn create_currency_token() -> (CurrencyToken, Principal) {
        let controller = random_principal_test();
        let token = CurrencyToken {
            balances: HashMap::new(),
            total_supply: 0,
            info: TokenInfo {
                name: String::from("test"),
                symbol: String::from("TST"),
                decimals: 8,
            },
            controllers: vec![controller],
        };

        (token, controller)
    }

    #[test]
    fn creation_works_fine() {
        let (token, controller) = create_currency_token();

        assert!(token.balances.is_empty());
        assert_eq!(token.total_supply, 0);
        assert_eq!(token.controllers.get(0).cloned().unwrap(), controller);
        assert_eq!(token.info.name, String::from("test"));
        assert_eq!(token.info.symbol, String::from("TST"));
        assert_eq!(token.info.decimals, 8);
    }

    #[test]
    fn minting_works_right() {
        let (mut token, controller) = create_currency_token();
        let user_1 = random_principal_test();

        token.mint(user_1, 100).ok().unwrap();

        assert_eq!(token.total_supply, 100);
        assert_eq!(token.balances.len(), 1);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 100);

        token.mint(controller, 200).ok().unwrap();

        assert_eq!(token.total_supply, 300);
        assert_eq!(token.balances.len(), 2);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 100);
        assert_eq!(token.balances.get(&controller).unwrap().clone(), 200);
    }

    #[test]
    fn burning_works_fine() {
        let (mut token, _) = create_currency_token();
        let user_1 = random_principal_test();

        token.mint(user_1, 100).ok().unwrap();

        token.burn(user_1, 90).ok().unwrap();

        assert_eq!(token.balances.len(), 1);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 10);
        assert_eq!(token.total_supply, 10);

        token.burn(user_1, 20).err().unwrap();

        token.burn(user_1, 10).ok().unwrap();

        assert!(token.balances.is_empty());
        assert!(token.balances.get(&user_1).is_none());
        assert_eq!(token.total_supply, 0);

        token.burn(user_1, 20).err().unwrap();
    }

    #[test]
    fn transfer_works_fine() {
        let (mut token, controller) = create_currency_token();
        let user_1 = random_principal_test();
        let user_2 = random_principal_test();

        token.mint(user_1, 1000).ok().unwrap();

        token.transfer(user_1, user_2, 100).ok().unwrap();

        assert_eq!(token.balances.len(), 2);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 900);
        assert_eq!(token.balances.get(&user_2).unwrap().clone(), 100);
        assert_eq!(token.total_supply, 1000);

        token.transfer(user_1, user_2, 1000).err().unwrap();

        token.transfer(controller, user_2, 100).err().unwrap();

        token.transfer(user_2, user_1, 100).ok().unwrap();

        assert_eq!(token.balances.len(), 1);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 1000);
        assert!(token.balances.get(&user_2).is_none());
        assert_eq!(token.total_supply, 1000);

        token.transfer(user_2, user_1, 1).err().unwrap();

        token.transfer(user_2, user_1, 0).err().unwrap();
    }
}
