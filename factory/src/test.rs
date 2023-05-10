#![cfg(test)]
extern crate std;
use crate::{pair, SoroswapFactoryClient};

use soroban_sdk::{testutils::Address as _,
                Address, 
                BytesN, 
                Env,
                token::Client as TokenClient,
                Bytes}; // TODO; add when testing authorizations: IntoVal, Symbol};

fn create_token_contract(e: &Env, admin: &Address) -> TokenClient {
    TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()))
}

fn create_factory_contract(
    e: &Env,
    setter: &Address,
    pair_wasm_hash: &BytesN<32>
) -> SoroswapFactoryClient {
    let factory = SoroswapFactoryClient::new(e, &e.register_contract(None, crate::SoroswapFactory {}));
    factory.initialize(&setter, pair_wasm_hash);
    factory
}

fn pair_token_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm"
    );
    e.install_contract_wasm(WASM)
}

// fn guess_contract_address(
//     e: &Env,
//     pair_wasm_hash: &BytesN<32>,
//     token_a: &BytesN<32>,
//     token_b: &BytesN<32>,
// ) -> BytesN<32> {
//     // Create a new Bytes instance using the current environment
//     let token_0;
//     let token_1;

//     if token_a < token_b {
//         token_0 = token_a;
//         token_1 = token_b;
//     }
//     else {
//         token_0 = token_b;
//         token_1 = token_a;
//     }
//     let mut salt = Bytes::new(e);

//     // Append the bytes of token_a and token_b to the salt
//     salt.append(&token_0.clone().into());
//     salt.append(&token_1.clone().into());

//     // Hash the salt using SHA256 to generate a new BytesN<32> value
//     let salt = e.crypto().sha256(&salt);

//     // Return the hash without deploying the contract
//     salt
// }

/*
Function that will guess the contract address.
Currently is not working as expected.
TODO: Fix
*/
pub fn guess_contract_address(
    e: &Env,
    factory: &BytesN<32>,
    token_a: &BytesN<32>,
    token_b: &BytesN<32>,
) -> BytesN<32> {
    let token_0;
    let token_1;
    if token_a < token_b {
        token_0 = token_a;
        token_1 = token_b;
    }
    else {
        token_0 = token_b;
        token_1 = token_a;
    }
    let mut salt = Bytes::new(e);
    salt.append(&factory.clone().into());
    salt.append(&token_0.clone().into());
    salt.append(&token_1.clone().into());
    let salt_hash = e.crypto().sha256(&salt);
    // let contract_address = Address::try_from(&salt_hash.as_ref()[12..]);
    // contract_address.unwrap_or_else(|_| BytesN::zero())
    salt_hash
}


fn create_pair( e: &Env,
                factory: &SoroswapFactoryClient,
                token_0: &BytesN<32>,
                token_1: &BytesN<32>) {
    factory.create_pair(&token_0, &token_1);
    
    // TODO: Test the event emmited
}

#[test]
fn test() {
    let e: Env = Default::default();

    let mut admin = Address::random(&e);
    let mut fake_admin = Address::random(&e);
    
    let mut factory = create_factory_contract(&e, &admin, &pair_token_wasm(&e));

    

    /*
    expect(await factory.feeTo()).to.eq(AddressZero)
    expect(await factory.feeToSetter()).to.eq(wallet.address)
    expect(await factory.allPairsLength()).to.eq(0)
    */

    // fee_to_setter is equal to admin / but is not equal to fake_admin
    assert_eq!(factory.fee_to_setter(), admin);
    assert_ne!(factory.fee_to_setter(), fake_admin);
    assert_eq!(factory.all_pairs_length(), 0);

    // TODO: Implement kind-of zero address to test:
    // assert_eq!(factory.fee_to(), ZERO_ADDRESS);
    
    // Create two tokens in order to create a pair using the factory
    let mut token_0 = create_token_contract(&e, &admin);
    let mut token_1 = create_token_contract(&e, &admin);

    create_pair(&e, &factory, &token_0.contract_id, &token_1.contract_id);

    let pair_expected_address = guess_contract_address( &e,
                                                        &factory.contract_id, 
                                                        &token_1.contract_id, 
                                                        &token_0.contract_id);
    let pair_address = factory.get_pair(&token_0.contract_id, &token_1.contract_id);
    let pair_address_inverted = factory.get_pair(&token_1.contract_id, &token_0.contract_id);


    // expect(await factory.getPair(...tokens)).to.eq(create2Address)
    // expect(await factory.getPair(...tokens.slice().reverse())).to.eq(create2Address)
    assert_eq!(&pair_address, &pair_address_inverted);
    
    // TODO: fix the guess_contract_address function and uncomment the following line
    //  assert_eq!(&pair_expected_address, &pair_address);

    // expect(await factory.allPairs(0)).to.eq(create2Address)   
    // TODO: fix the guess_contract_address function and uncomment the following line
    // assert_eq!(&factory.all_pairs(&0), &pair_expected_address);
    assert_eq!(&factory.all_pairs(&0), &pair_address);

    // Test that all_pairs_length now is equal to 1
    // expect(await factory.allPairsLength()).to.eq(1)
    assert_eq!(factory.all_pairs_length(), 1);

    // TODO: Test that the pair:
    //      - has been correctly created
    //      - has the factory address correctly
    //      - token_0 is correct
    //      - token_1 is correct

    let pair_client = pair::Client::new(&e, &pair_address);
    assert_eq!(pair_client.factory(), Address::from_contract_id(&factory.contract_id));


    // const pair = new Contract(create2Address, JSON.stringify(UniswapV2Pair.abi), provider)
    // expect(await pair.factory()).to.eq(factory.address)
    // expect(await pair.token0()).to.eq(TEST_ADDRESSES[0])
    // expect(await pair.token1()).to.eq(TEST_ADDRESSES[1])

}

// Creating the same pair again should fail
// await expect(factory.createPair(...tokens)).to.be.reverted // UniswapV2: PAIR_EXISTS
#[test]
#[should_panic(expected = "SoroswapFactory: pair already exist between token_0 and token_1")]
fn test_double_same_pair_not_possible() {
    let e: Env = Default::default();
    let mut admin = Address::random(&e);    
    let mut factory = create_factory_contract(&e, &admin, &pair_token_wasm(&e));
    let mut token_0 = create_token_contract(&e, &admin);
    let mut token_1 = create_token_contract(&e, &admin);

    factory.create_pair(&token_0.contract_id, &token_1.contract_id);

    // Second creation of same pair should fail
    factory.create_pair(&token_0.contract_id, &token_1.contract_id);
}

// Creating the same pair again (but in inverse order) should also fail
// await expect(factory.createPair(...tokens.slice().reverse())).to.be.reverted // UniswapV2: PAIR_EXISTS

#[test]
#[should_panic(expected = "SoroswapFactory: pair already exist between token_0 and token_1")]
fn test_double_inverse_pair_not_possible() {
    let e: Env = Default::default();
    let mut admin = Address::random(&e);    
    let mut factory = create_factory_contract(&e, &admin, &pair_token_wasm(&e));
    let mut token_0 = create_token_contract(&e, &admin);
    let mut token_1 = create_token_contract(&e, &admin);

    factory.create_pair(&token_0.contract_id, &token_1.contract_id);

    // Second creation of same pair (but now in reverse order) should fail
    factory.create_pair(&token_1.contract_id, &token_0.contract_id);
}

// TODO: Test: Should panic when other account tries to change the fee_to
// TODO: Test: Should panic when other account tries to change the fee_to_setter