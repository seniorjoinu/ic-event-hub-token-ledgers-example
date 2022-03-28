use ic_cdk::export::candid::export_service;
use ic_cdk::export::Principal;
use ic_cdk::{caller, spawn, trap};
use ic_cdk_macros::{heartbeat, init, query, update};
use ic_cron::implement_cron;
use ic_cron::types::Iterations::Exact;
use ic_cron::types::SchedulingOptions;
use ic_event_hub::api::IEventHubClient;
use ic_event_hub::types::{CallbackInfo, Event, EventFilter, IEvent, SubscribeRequest};

use crate::common::ledger::Ledger;
use crate::common::types::{BurnEvent, Entry, MintEvent, TransferEvent};

mod common;

// ----------------- MAIN LOGIC ------------------

#[query]
fn get_events() -> Vec<Entry> {
    get_state().get_entries()
}

#[update(guard = "token_guard")]
pub fn events_callback(events: Vec<Event>) {
    for event in events {
        match event.get_name().as_str() {
            "MintEvent" => {
                let mint_event = MintEvent::from_event(event);
                get_state().add_entry(Entry::Mint(mint_event));
            }
            "TransferEvent" => {
                let transfer_event = TransferEvent::from_event(event);
                get_state().add_entry(Entry::Transfer(transfer_event));
            }
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

static mut STATE: Option<Ledger> = None;

pub fn get_state() -> &'static mut Ledger {
    unsafe { STATE.as_mut().unwrap() }
}

implement_cron!();

#[init]
fn init(token_principal: Principal) {
    let token = Ledger {
        token: token_principal,
        entries: Vec::new(),
    };

    unsafe {
        STATE = Some(token);
    }

    cron_enqueue(
        token_principal,
        SchedulingOptions {
            delay_nano: 0,
            interval_nano: 0,
            iterations: Exact(1),
        },
    )
    .expect("Enqueue failed");
}

#[heartbeat]
pub fn tick() {
    for task in cron_ready_tasks() {
        let token = task
            .get_payload::<Principal>()
            .expect("Payload deserialization failed");

        spawn(async move {
            token
                .subscribe(SubscribeRequest {
                    callbacks: vec![CallbackInfo {
                        filter: EventFilter::empty(),
                        method_name: String::from("events_callback"),
                    }],
                })
                .await
                .expect("Subscribe failed");
        });
    }
}
