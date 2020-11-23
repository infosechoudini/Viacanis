// T1531 - Account Access Removal
// Tactic::Impact
use crate::eventlog::subscriber;

use async_std::task;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use win_event_log::prelude::*;
use xmlJSON::XmlDocument;
use std::str::FromStr;
use log::*;



#[derive(Serialize, Deserialize, Default, Debug)]
struct AccountCreation {
    target_user: String,
    target_domain: String,
    subject_username: String,
}

pub async fn monitor_delete_accounts(){
    info!{"DELETE ACCOUNTS"};

    let id = 4726 as u32;

    let event_conditions = Condition::filter(EventFilter::event(id));

    info!("\nMonitoring Event Condition: {}", event_conditions);
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

    loop  {
        let query = query.clone();
        match subscriber::WinEventsSubscriber::get(query) {
            Ok(mut events) => {
                info!("\nCtrl+C to quit!");
                info!("Waiting for new events...");
                loop {
                    while let Some(event) = events.next() {
                        let string_event = event.to_string();
                        let res = string_event.trim_matches(char::from(0));
                        let data = XmlDocument::from_str(&res).unwrap();
                        let mut account_creation = AccountCreation::default();
                        for data_key in data.data.iter(){
                            for xmldata in &data_key.sub_elements{
                                if xmldata.name == "EventData"{
                                    for subelement in &xmldata.sub_elements{
                                        for (_att_key, att_value) in &subelement.attributes{
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
                        info!("{}", "MITRE Technique: T1531 - Account Access Removal");
                        info!{"New User: {}, Domain: {}, Culprit: {}", account_creation.target_user, account_creation.target_domain, account_creation.subject_username };
                    }
                    let dur = Duration::from_millis(200);
                    task::sleep(dur).await;
                }
            }
            Err(e) => {
                error!("Error: {}", e);
            },
        }
    }
}

pub async fn hunt_delete_accounts(){
    info!{"DELETE ACCOUNTS"};

    let id = 4726 as u32;

    let event_conditions = Condition::filter(EventFilter::event(id));

    info!("\nMonitoring Event Condition: {}", event_conditions);
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
    match WinEvents::get(query) {
        Ok(events) => {
            if let Some(event) = events.into_iter().next() {
                let string_event = event.to_string();
                let res = string_event.trim_matches(char::from(0));
                let data = XmlDocument::from_str(&res).unwrap();
                let mut account_creation = AccountCreation::default();
                for data_key in data.data.iter(){
                    for xmldata in &data_key.sub_elements{
                        if xmldata.name == "EventData"{
                            for subelement in &xmldata.sub_elements{
                                for (_att_key, att_value) in &subelement.attributes{
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
                info!("{}", "MITRE Technique: T1531 - Account Access Removal");
                info!{"New User: {}, Domain: {}, Culprit: {}", account_creation.target_user, account_creation.target_domain, account_creation.subject_username };
            }          
        }
        Err(e) => info!("Error: {}", e),
    }
}
