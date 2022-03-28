use crate::common::types::Entry;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;

#[derive(CandidType, Deserialize)]
pub struct Ledger {
    pub token: Principal,
    pub entries: Vec<Entry>,
}

impl Ledger {
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> Vec<Entry> {
        self.entries.clone()
    }
}
