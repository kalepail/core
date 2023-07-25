
use soroban_sdk::{symbol_short, contract, contractimpl, Env, Symbol, Address};

   
const DUMMY: Symbol = symbol_short!("DUMMY");


pub trait FactoryTrait {

    fn fee_to(e: Env) -> Address;
    fn fees_enabled(e: Env) -> bool;
}

#[contract]
pub struct Factory;

#[contractimpl]
impl FactoryTrait for Factory {

    fn fee_to(e: Env) -> Address {
        let address: Address = e.storage().instance().get(&DUMMY).unwrap();
        address
    }

    fn fees_enabled(_e: Env) -> bool {
        true
    }
}
