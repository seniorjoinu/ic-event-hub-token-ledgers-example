use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_event_hub_macros::Event;

#[derive(Debug)]
pub enum Error {
    InsufficientBalance,
    ZeroQuantity,
    AccessDenied,
    ForbiddenOperation,
}

pub type Controllers = Vec<Principal>;

#[derive(Clone, CandidType, Deserialize)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Event)]
pub struct TransferEvent {
    #[topic]
    pub from: Principal,
    #[topic]
    pub to: Principal,
    pub amount: u64,
    pub timestamp: u64,
}

#[derive(Event)]
pub struct MintEvent {
    #[topic]
    pub to: Principal,
    pub amount: u64,
    pub timestamp: u64,
}

#[derive(Event)]
pub struct BurnEvent {
    #[topic]
    pub from: Principal,
    pub amount: u64,
    pub timestamp: u64,
}