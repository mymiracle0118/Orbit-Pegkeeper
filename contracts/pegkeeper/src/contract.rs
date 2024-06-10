#![no_std]
use soroban_sdk::{Address, contract, contractimpl, Env};
use crate::errors::PegkeeperError;

#[contract]
pub struct PegkeeperContract;

#[contractimpl]
impl PegkeeperContract {
    pub fn loan(env: Env, token_address: Address, amount: u128) -> Result<bool, PegkeeperError> {
        
        Ok(true)
    }
    pub fn repay(env: Env, amount: u128) -> bool {
        true
    }
}

