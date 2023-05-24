
use soroban_sdk::{contractimpl, Env, BytesN, Symbol};

   
const DUMMY: Symbol = Symbol::short("DUMMY");


pub trait FactoryTrait {

    fn fee_to(e: Env) -> BytesN<32>;
    fn fee_on(e: Env) -> bool;
}

pub struct Factory;

#[contractimpl]
impl FactoryTrait for Factory {

    fn fee_to(e: Env) -> BytesN<32> {
        e.storage().get(&DUMMY).unwrap().unwrap()
    }

    fn fee_on(e: Env) -> bool {
        true
    }
}
