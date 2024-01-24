#![cfg(test)]
extern crate std;

use soroban_sdk::{Env, Address, vec, Vec, IntoVal, String};

use crate::test::{SoroswapRouterTest};
use crate::test::add_liquidity::add_liquidity;
use crate::error::CombinedRouterError;

// Malicious Token Contract
mod token_malicious_contract {
    soroban_sdk::contractimport!(file = "../token-malicious/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm");
    pub type MaliciousTokenClient<'a> = Client<'a>;
}

use token_malicious_contract::MaliciousTokenClient;

fn create_token_malicious_contract<'a>(e: &Env, admin: & Address) -> MaliciousTokenClient<'a> {
    let token_malicious_address = &e.register_contract_wasm(None, token_malicious_contract::WASM);
    let token_malicious = MaliciousTokenClient::new(e, token_malicious_address); 
    token_malicious.initialize(
        &admin,
        &7,
        &String::from_str(&e, "name"),
        &String::from_str(&e, "name"), 
    );
    token_malicious
}



#[test]
fn phishing_attack() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;  
    let initial_user_balance: i128 = 10_000_000_000_000_000_000;

    // Malicious token setup
    let token_malicious = create_token_malicious_contract(&test.env, &test.admin);
    // This is being executed by the admin.
    token_malicious.mint(&test.user, &initial_user_balance);

    token_malicious.set_target_token_contract(&test.token_1.address.clone());
    token_malicious.set_target_user(&test.user.clone());

    let amount_0: i128 = 4_000_000_000;
    let amount_1: i128 = 1_000_000_000;

    // Initial balance
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance);
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance);
    assert_eq!(test.token_0.balance(&test.admin), 0);
    assert_eq!(test.token_1.balance(&test.admin), 0);
    assert_eq!(token_malicious.balance(&test.user), initial_user_balance);
    assert_eq!(token_malicious.balance(&test.admin), 0);

    test.contract.add_liquidity(
        &test.token_0.address, //     token_a: Address,
        &token_malicious.address, //     token_b: Address,
        &amount_0, //     amount_a_desired: i128,
        &amount_1, //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &deadline//     deadline: u64,
    );

    // added tokens, initial balance - amount added
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance - amount_0);
    assert_eq!(token_malicious.balance(&test.user), initial_user_balance - amount_1);

    assert_eq!(test.token_1.balance(&test.user), 0);
    assert_eq!(test.token_1.balance(&test.admin), initial_user_balance);

    // to test manually
    // cargo test --package soroban-token-contract phishing_attack
}

