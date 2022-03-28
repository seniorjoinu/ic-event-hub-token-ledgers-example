use std::collections::HashMap;

use ic_cdk::api::time;
use ic_cdk::caller;
use ic_cdk::export::Principal;
use ic_cdk_macros::{heartbeat, init, query, update};
use ic_event_hub::{implement_event_emitter, implement_subscribe, implement_unsubscribe};

use crate::common::currency_token::CurrencyToken;
use crate::common::guards::controller_guard;
use crate::common::types::{BurnEvent, MintEvent, TokenInfo, TransferEvent};

mod common;

// ----------------- MAIN LOGIC ------------------

#[update(guard = "controller_guard")]
fn mint(to: Principal, qty: u64) {
    get_state().mint(to, qty).expect("Minting failed");

    emit(MintEvent {
        to,
        amount: qty,
        timestamp: time(),
    });
}

#[update]
fn transfer(to: Principal, qty: u64) {
    let from = caller();

    get_state()
        .transfer(from, to, qty)
        .expect("Transfer failed");

    emit(TransferEvent {
        from,
        to,
        amount: qty,
        timestamp: time(),
    });
}

#[update]
fn burn(qty: u64) {
    let from = caller();

    get_state().burn(from, qty).expect("Burning failed");

    emit(BurnEvent {
        from,
        amount: qty,
        timestamp: time(),
    });
}

#[query]
fn get_balance_of(account_owner: Principal) -> u64 {
    get_state().balance_of(&account_owner)
}

#[query]
fn get_total_supply() -> u64 {
    get_state().total_supply
}

#[query]
fn get_info() -> TokenInfo {
    get_state().info.clone()
}

// ------------------- EVENT HUB ------------------

implement_event_emitter!(10 * 1_000_000_000, 100 * 1024);
implement_subscribe!();
implement_unsubscribe!();

#[heartbeat]
pub fn tick() {
    send_events();
}

// ------------------ STATE ----------------------

static mut STATE: Option<CurrencyToken> = None;

pub fn get_state() -> &'static mut CurrencyToken {
    unsafe { STATE.as_mut().unwrap() }
}

#[init]
fn init(controller: Principal, info: TokenInfo) {
    let token = CurrencyToken {
        balances: HashMap::new(),
        total_supply: 0,
        info,
        controllers: vec![controller],
    };

    unsafe {
        STATE = Some(token);
    }
}
