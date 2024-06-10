use soroban_sdk::{Address, Env, Symbol, symbol_short, unwrap::UnwrapOptimized, contracttype};

use crate::dependencies::treasury;

const ONE_DAY_LEDGERS: u32 = 17280; // assumes 5s a ledger

const LEDGER_THRESHOLD_INSTANCE: u32 = ONE_DAY_LEDGERS * 30; // ~ 30 days
const LEDGER_BUMP_INSTANCE: u32 = LEDGER_THRESHOLD_INSTANCE + ONE_DAY_LEDGERS; // ~ 31 days

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    ADMIN,
    TREASURY(Address), // mapping token address to treasury addres
    BLEND,
    SOROSWAP
}
/// Bump the instance rent for the contract
pub fn extend_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD_INSTANCE, LEDGER_BUMP_INSTANCE);
}

/// Check if the contract has been initialized
pub fn is_init(e: &Env) -> bool { e.storage().instance().has(&DataKey::ADMIN) }

/// Fetch the current admin Address
///
/// ### Panics
/// If the admin does not exist
pub fn get_admin(e: &Env) -> Option<Address> {
    e.storage()
        .instance()
        .get(&DataKey::ADMIN)
        .unwrap_or_default()
}

/// Set a new admin
///
/// ### Arguments
/// * `new_admin` - The Address for the admin
pub fn set_admin(e: &Env, new_admin: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::ADMIN, new_admin);
}

/// Fetch the current treasury Address depending on token address
///
/// ### Panics
/// If the treasury does not exist
pub fn get_treasury(e: &Env, token_address: Address) -> Option<Address> {
    e.storage()
        .instance()
        .get(&DataKey::TREASURY(token_address))
        .unwrap_or_default()
}

/// Set the treasury Address depending on token address
///
/// ### Arguments
/// * `token_address` - token address of treasury, `treasury_address` - The Address for the treasury
pub fn set_treasury(e: &Env, token_address: Address, treasury_address: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::TREASURY(token_address), treasury_address);
}

/// Fetch the current blend Address
///
/// ### Panics
/// If the blend does not exist
pub fn get_blend(e: &Env) -> Option<Address> {
    e.storage()
        .instance()
        .get(&DataKey::BLEND)
        .unwrap_or_default()
}

/// Set the blend Address
///
/// ### Arguments
/// * `blend` - The Address for the blend pool
pub fn set_blend(e: &Env, blend: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::BLEND, blend);
}

/// Fetch the current token Address
///
/// ### Panics
/// If the token does not exist
pub fn get_token(e: &Env) -> Option<Address> {
    e.storage()
        .instance()
        .get(&DataKey::SOROSWAP)
        .unwrap_or_default()
}

/// Set the token Address
///
/// ### Arguments
/// * `token` - The Address for the token
pub fn set_token(e: &Env, token: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::SOROSWAP, token);
}