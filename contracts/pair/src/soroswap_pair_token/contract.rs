//! This contract demonstrates a sample implementation of the Soroban token
//! interface.
use crate::soroswap_pair_token::admin::{has_administrator, read_administrator, write_administrator};
use crate::soroswap_pair_token::allowance::{read_allowance, spend_allowance, write_allowance};
use crate::soroswap_pair_token::balance::{read_balance, receive_balance, spend_balance};
use crate::soroswap_pair_token::metadata::{read_decimal, read_name, read_symbol, write_metadata};
use crate::soroswap_pair_token::total_supply::{read_total_supply, increase_total_supply, decrease_total_supply};
use crate::error::SoroswapPairError;

#[cfg(test)]
use crate::soroswap_pair_token::storage_types::{AllowanceDataKey, AllowanceValue, DataKey};
use crate::soroswap_pair_token::storage_types::{INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};
use soroban_sdk::token::{self, Interface as _};
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use soroban_token_sdk::metadata::TokenMetadata;
use soroban_token_sdk::TokenUtils;

fn check_nonnegative_amount(amount: i128) -> Result<(), SoroswapPairError> {
    if amount < 0 {
        //  panic!("negative amount is not allowed: {}", amount)
        //  TokenNegativeAmountNotAllowed = 123,
        return Err(SoroswapPairError::TokenNegativeAmountNotAllowed);
    }
    Ok(())
}

pub fn internal_burn(e: Env, from: Address, amount: i128) {
    check_nonnegative_amount(amount);
 
    e.storage()
    .instance()
    .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    
    spend_balance(&e, from.clone(), amount);
    decrease_total_supply(&e, amount);

    TokenUtils::new(&e).events().burn(from, amount);
} 

pub fn internal_mint(e: Env, to: Address, amount: i128) {
    check_nonnegative_amount(amount);

    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        
    receive_balance(&e, to.clone(), amount);
    increase_total_supply(&e, amount);

    TokenUtils::new(&e).events().mint(e.current_contract_address(), to, amount);
}


#[contract]
pub struct SoroswapPairToken;

#[contractimpl]
impl SoroswapPairToken {
    pub fn initialize(e: Env, admin: Address, decimal: u32, name: String, symbol: String) -> Result<(), SoroswapPairError> {
        if has_administrator(&e) {
            //  panic!("already initialized")
            //  TokenInitializeAlreadyInitialized = 124,
            return Err(SoroswapPairError::TokenInitializeAlreadyInitialized);
        }
        write_administrator(&e, &admin);
        if decimal > u8::MAX.into() {
            //  panic!("Decimal must fit in a u8");
            //  TokenDecimalNotAllowed = 125,
            return Err(SoroswapPairError::TokenDecimalNotAllowed);
        }

        write_metadata(
            &e,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        );

        Ok(())
    }

    pub fn mint(e: Env, to: Address, amount: i128) {
        let admin = read_administrator(&e);
        admin.require_auth();
        internal_mint(e, to, amount);
    }

    pub fn set_admin(e: Env, new_admin: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_administrator(&e, &new_admin);
        TokenUtils::new(&e).events().set_admin(admin, new_admin);
    }

    pub fn total_supply(e: Env) -> i128 {
        read_total_supply(&e)
    }

    #[cfg(test)]
    pub fn get_allowance(e: Env, from: Address, spender: Address) -> Option<AllowanceValue> {
        let key = DataKey::Allowance(AllowanceDataKey { from, spender });
        let allowance = e.storage().temporary().get::<_, AllowanceValue>(&key);
        allowance
    }
}

#[contractimpl]
impl token::Interface for SoroswapPairToken { 
    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_allowance(&e, from, spender).amount
    }

    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_allowance(&e, from.clone(), spender.clone(), amount, expiration_ledger);
        TokenUtils::new(&e)
            .events()
            .approve(from, spender, amount, expiration_ledger);
    }

    fn balance(e: Env, id: Address) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount);
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount)
    }

    fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth();
        internal_burn(e, from, amount);
    }

    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        decrease_total_supply(&e, amount);

        TokenUtils::new(&e).events().burn(from, amount)
    }

    fn decimals(e: Env) -> u32 {
        read_decimal(&e)
    }

    fn name(e: Env) -> String {
        read_name(&e)
    }

    fn symbol(e: Env) -> String {
        read_symbol(&e)
    }
}
