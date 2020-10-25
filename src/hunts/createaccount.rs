// T1136 - Create Account
// Tactic::Persistence
use crate::colors::*;
use crate::eventlog::subscriber;

use std::thread::sleep;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use evtx::EvtxParser;
use xml_to_json;
use std::fs::File;
use win_event_log::prelude::*;
use std::io::prelude::*;
use std::sync::Arc;
use ben;

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "PascalCase")]
struct Execution {
    pub ProcessID: Option<i16>,
    pub ThreadID: Option<i16>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "PascalCase")]
struct TimeCreated {
    pub SystemTime: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "PascalCase")]
struct Provider {
    pub name: Option<String>,
    pub guid: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "PascalCase")]
struct System {
    pub provider: Option<Provider>,
    pub EventID: Option<i16>,
    pub Computer: Option<String>,
    pub TimeCreated: Option<TimeCreated>,
    pub Execution: Option<Execution>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "PascalCase")]
struct EventData {
    pub Data: Option<Vec<u8>>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "PascalCase")]
struct AccountCreation {
    pub system: Option<System>,
    pub EventData: Option<EventData>
}


pub fn hunt_create_accounts (){
    let hunt_config = true;
    let ten_millis = Duration::from_millis(10);

    let id = 4720 as u32;
    let event_filter = EventFilter::event_data("EventRecordID", "4720");

    let event_conditions = Condition::filter(EventFilter::event(id));

    println!("{}", event_conditions);

    let conditions = vec![
        Condition::filter(EventFilter::level(4, Comparison::GreaterThanOrEqual)),
        Condition::filter(EventFilter::level(0, Comparison::GreaterThanOrEqual)),
    ];
    let query = QueryList::new()
        .with_query(
            Query::new()
                .item(
                    QueryItem::selector("Security".to_owned())
                        .system_conditions(event_conditions)
                        .build(),
                )
                .query(),
        )
        .build();

    while hunt_config {
        let query = query.clone();
        match subscriber::WinEventsSubscriber::get(query) {
            Ok(mut events) => {
                println!("Ctrl+C to quit!");
                while let Some(_event) = events.next() {
                    // catch up to present
                }
                println!("Waiting for new events...");
                loop {
                    while let Some(event) = events.next() {

                        let string_event = event.into();
                        //let res = string_event.trim_matches(char::from(0)).as_bytes().to_vec();
                        
                    }
                    sleep(Duration::from_millis(200));
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

