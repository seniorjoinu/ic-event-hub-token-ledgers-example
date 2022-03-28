use ic_cdk::export::candid::export_service;
use ic_cdk::export::Principal;
use ic_cdk::{caller, spawn, trap};
use ic_cdk_macros::{heartbeat, init, query, update};
use ic_cron::implement_cron;
use ic_cron::types::Iterations::Exact;
use ic_cron::types::SchedulingOptions;
use ic_event_hub::api::IEventHubClient;
use ic_event_hub::types::{CallbackInfo, Event, IEvent, IEventFilter, SubscribeRequest};

use crate::common::ledger::Ledger;
use crate::common::types::{
    BurnEvent, BurnEventFilter, Entry, MintEvent, MintEventFilter, TransferEvent,
    TransferEventFilter,
};

mod common;

// ----------------- MAIN LOGIC ------------------

#[init]
fn init(token_principal: Principal, track_principal: Principal) {
    let token = Ledger {
        token: token_principal,
        track: track_principal,
        entries: Vec::new(),
    };

    unsafe {
        STATE = Some(token);
    }

    cron_enqueue(
        (),
        SchedulingOptions {
            delay_nano: 0,
            interval_nano: 0,
            iterations: Exact(1),
        },
    )
    .expect("Enqueue failed");
}

#[query]
fn get_events() -> Vec<Entry> {
    get_state().get_entries()
}

implement_cron!();

#[heartbeat]
pub fn tick() {
    for _ in cron_ready_tasks() {
        spawn(async move {
            let state = get_state();

            state
                .token
                .subscribe(SubscribeRequest {
                    callbacks: vec![
                        CallbackInfo {
                            filter: MintEventFilter {
                                to: Some(state.track),
                            }
                            .to_event_filter(),
                            method_name: String::from("mint_callback"),
                        },
                        CallbackInfo {
                            filter: TransferEventFilter {
                                from: Some(state.track),
                                to: None,
                            }
                            .to_event_filter(),
                            method_name: String::from("transfer_callback"),
                        },
                        CallbackInfo {
                            filter: TransferEventFilter {
                                from: None,
                                to: Some(state.track),
                            }
                            .to_event_filter(),
                            method_name: String::from("transfer_callback"),
                        },
                        CallbackInfo {
                            filter: BurnEventFilter {
                                from: Some(state.track),
                            }
                            .to_event_filter(),
                            method_name: String::from("burn_callback"),
                        },
                    ],
                })
                .await
                .expect("Subscribe failed");
        });
    }
}

#[update(guard = "token_guard")]
pub fn mint_callback(events: Vec<Event>) {
    for event in events {
        match event.get_name().as_str() {
            "MintEvent" => {
                let mint_event = MintEvent::from_event(event);
                get_state().add_entry(Entry::Mint(mint_event));
            }
            _ => trap("Unknown event"),
        }
    }
}

#[update(guard = "token_guard")]
pub fn transfer_callback(events: Vec<Event>) {
    for event in events {
        match event.get_name().as_str() {
            "TransferEvent" => {
                let transfer_event = TransferEvent::from_event(event);
                get_state().add_entry(Entry::Transfer(transfer_event));
            }
            _ => trap("Unknown event"),
        }
    }
}

#[update(guard = "token_guard")]
pub fn burn_callback(events: Vec<Event>) {
    for event in events {
        match event.get_name().as_str() {
            "BurnEvent" => {
                let burn_event = BurnEvent::from_event(event);
                get_state().add_entry(Entry::Burn(burn_event));
            }
            _ => trap("Unknown event"),
        }
    }
}

fn token_guard() -> Result<(), String> {
    if caller() != get_state().token {
        Err(String::from("Can only be called by the token canister"))
    } else {
        Ok(())
    }
}

// ------------------ STATE ----------------------

export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

static mut STATE: Option<Ledger> = None;

pub fn get_state() -> &'static mut Ledger {
    unsafe { STATE.as_mut().unwrap() }
}
