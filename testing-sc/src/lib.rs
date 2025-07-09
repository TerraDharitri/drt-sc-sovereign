#![no_std]

#[allow(unused_imports)]
use dharitri_sc::imports::*;

#[dharitri_sc::contract]
pub trait TestingSc {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint]
    fn hello(&self, value: BigUint) {
        require!(value > BigUint::zero(), "Value should be greater than 0")
    }
}
