// T1531 - Account Access Removal
// Tactic::Impact
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
use xmlJSON::XmlDocument;
use std::str::FromStr;
use rustc_serialize::json::ToJson;
use colored::*;

#[derive(Deserialize, Default, Debug)]
struct AccountCreation {
    target_user: String,
    target_domain: String,
    subject_username: String,
}


pub async fn hunt_delete_accounts (){
    println!{"DELETE ACCOUNTS"};
    let hunt_config = true;
    let ten_millis = Duration::from_millis(10);

    let id = 4726 as u32;
    let event_filter = EventFilter::event_data("EventRecordID", "4726");

    let event_conditions = Condition::filter(EventFilter::event(id));

    println!("\nMonitoring Event Condition: {}", event_conditions);

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
                println!("\nCtrl+C to quit!");
                while let Some(_event) = events.next() {
                    // catch up to present
                }
                println!("Waiting for new events...");
                loop {
                    while let Some(event) = events.next() {
                        let string_event = event.to_string();
                        let mut res = string_event.trim_matches(char::from(0));
                        let data = XmlDocument::from_str(&res).unwrap();
                        let mut account_creation = AccountCreation::default();
                        for data_key in data.data.iter(){
                            for xmldata in &data_key.sub_elements{
                                if xmldata.name == "EventData"{
                                    for subelement in &xmldata.sub_elements{
                                        for (att_key, att_value) in &subelement.attributes{
                                            match att_value.as_str(){
                                                "TargetUserName" => {account_creation.target_user = subelement.data.as_ref().unwrap().to_string()}
                                                "TargetDomainName" => {account_creation.target_domain = subelement.data.as_ref().unwrap().to_string()}
                                                "SubjectUserName" => {account_creation.subject_username = subelement.data.as_ref().unwrap().to_string()}
                                                 _ => continue,
                                            }
                                        }
                                    }
                                }
                            }

                        }
                        println!("{}", "MITRE Technique: T1531 - Account Access Removal");
                        println!{"New User: {}, Domain: {}, Culprit: {}", account_creation.target_user, account_creation.target_domain, account_creation.subject_username };
                    }
                    sleep(Duration::from_millis(200));
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

