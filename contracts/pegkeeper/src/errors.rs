use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PegkeeperError {
    /// not yet initialized
    NotInitialized = 101,
    
    /// not proper treasury for token
    NotProperTreasury = 102,

    /// uncorrect amount for loan
    UncorrectAmount = 103,

    /// unsufficient amount to repay
    UnsufficientAmount = 104
}