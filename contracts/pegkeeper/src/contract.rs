#![no_std]
use soroban_sdk::{Address, contract, contractimpl, Env, token};
use crate::{
    dependencies::treasury, 
    errors::PegkeeperError,
    storage,
    balances
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
    pub fn repay(env: Env, token_address: Address, treasury_address: Address, amount: i128, treasury_fee: i128) -> Result<(), PegkeeperError> {
        storage::extend_instance(&env);
    
        // Check balance of token of contract
        let balance = balances::get_balance(&env, token_address.clone());
        if balance < 0 {
            return Err(PegkeeperError::NegativeBalance);
        }
    
        if balance < amount + treasury_fee {
            return Err(PegkeeperError::InsufficientBalance);
        }
    
        // Trades on any other protocols

        // Repay the flash loan amount + treasury fee to treasury
        balances::transfer_amount(&env, token_address, treasury_address, amount);

        Ok(())
    }
    
}

