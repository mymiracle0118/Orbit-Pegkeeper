#![no_std]
use soroban_sdk::{Address, contract, contractimpl, Env};
use crate::{
    dependencies::treasury, 
    errors::PegkeeperError,
    storage
};

#[contract]
pub struct PegkeeperContract;

#[contractimpl]
impl PegkeeperContract {
    pub fn loan(env: Env, token_address: Address, amount: u128) -> Result<(), PegkeeperError> {
        storage::extend_instance(&env);

        let admin = storage::get_admin(&env);
        if admin.is_none() {
            return Err(PegkeeperError::NotInitialized);
        }
        let admin = admin.unwrap();
        admin.require_auth();

        let treasury_address = storage::get_treasury(&env, token_address);
        if treasury_address.is_none() {
            return Err(PegkeeperError::NotProperTreasury);
        }

        let treasury_address = treasury_address.unwrap();
        
        /// invoke treasury loan func
        
        
        Ok(())
    }
    pub fn repay(env: Env, token_addres: Address, treasury_address: Address, amount: u128) -> Result<(), PegkeeperError> {
        storage::extend_instance(&env);
        
        /// check balance of token of contract
        
        /// repay the arbitrage delta + flash loan amount to treasury

        Ok(())
    }
}

