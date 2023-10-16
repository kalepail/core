#![no_std]

// /extern crate soroswap_library;

use soroban_sdk::{
    contract, contractimpl};

// use SoroswapLibraryTrait;
//use fixed_point_math;
use dummy_contract;

pub trait SoroswapRouterTrait{

    // **** LIBRARY FUNCTIONS ****
    
    // given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
    fn quote(amount_a: i128, reserve_a: i128, reserve_b: i128) -> i128;
}

#[contract]
struct SoroswapRouter;

#[contractimpl]
impl SoroswapRouterTrait for SoroswapRouter {
  
   
    // given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
    fn quote(amount_a: i128, reserve_a: i128, reserve_b: i128)  -> i128 {
        // function quote(uint amountA, uint reserveA, uint reserveB) public pure virtual override returns (uint amountB) {
        //     return UniswapV2Library.quote(amountA, reserveA, reserveB);
        // }
        //quote(amount_a, reserve_a, reserve_b)
        0
    }

}
