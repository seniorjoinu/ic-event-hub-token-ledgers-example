use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_event_hub_macros::Event;

#[derive(Clone, CandidType, Deserialize)]
pub enum Entry {
    Mint(MintEvent),
    Transfer(TransferEvent),
    Burn(BurnEvent),
}

#[derive(Clone, Event, CandidType, Deserialize)]
pub struct TransferEvent {
    #[topic]
    pub from: Principal,
    #[topic]
    pub to: Principal,
    pub amount: u64,
    pub timestamp: u64,
}

#[derive(Clone, Event, CandidType, Deserialize)]
pub struct MintEvent {
    #[topic]
    pub to: Principal,
    pub amount: u64,
    pub timestamp: u64,
}

#[derive(Clone, Event, CandidType, Deserialize)]
pub struct BurnEvent {
    #[topic]
    pub from: Principal,
    pub amount: u64,
    pub timestamp: u64,
}