// T1136 - Create Account
// Tactic::Persistence
use crate::eventlog::subscriber;
use crate::eventlog::api;
use crate::messages::messages::*;
use crate::user::comms;


use async_std::task;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use win_event_log::prelude::{QueryItem, QueryList, Query, Condition, EventFilter};
use xmlJSON::XmlDocument;
use std::str::FromStr;
use log::*;
use tokio::*;


#[derive(Serialize, Deserialize, Default, Debug)]
struct AccountCreation {
    time_created: String,
    target_user: String,
    target_domain: String,
    subject_username: String,
}


pub async fn monitor_create_accounts(){
    info!{"ATTACHED FUTURE: CREATE ACCOUNTS"};

    let id = 4720 as u32;
    let event_conditions = Condition::filter(EventFilter::event(id));

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

                                if xmldata.name == "System"{
                                    for subelement in &xmldata.sub_elements{
                                        for (att_key, att_value) in &subelement.attributes{
                                            match att_key.as_str(){
                                                "SystemTime" => {account_creation.time_created = att_value.to_string()}
                                                 _ => continue,
                                            }
                                        }
                                    }
                                }
                            }

                        }
                        info!("{} - Created At: {}", "MITRE Technique: T1136 - New Account", account_creation.time_created);
                        info!{"Time: {} New User: {}, Domain: {}, Culprit: {}",account_creation.time_created, account_creation.target_user, account_creation.target_domain, account_creation.subject_username };
                    
                        let mut base_msg = BaseMessage::default();
                        let mut alert_msg = Alert::default();
                        alert_msg.engine = "Hunts".to_string();
                        alert_msg.title = "MITRE Technique: T1136 - New Account".to_string();
                        alert_msg.description = "New Account Created".to_string();
                        alert_msg.details =  format!{"Time: {}, New User: {}, Domain: {}, Culprit: {}", account_creation.time_created, account_creation.target_user, account_creation.target_domain, account_creation.subject_username }.to_string();

                        base_msg.payload_type = "Alert".to_string();
                        base_msg.alert = alert_msg;

                        comms::send_alert(base_msg.clone()).await;
                    }

                    let dur = Duration::from_millis(1000);
                    task::sleep(dur).await;
                }
            }
            Err(e) => {
                error!("Error: {}", e);
            },
        }
    }

}

pub async fn hunt_create_accounts(){

    let id = 4720 as u32;
    let event_conditions = Condition::filter(EventFilter::event(id));

    info!("\nHunting Event Condition: {}", event_conditions);

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

    match api::WinEvents::get(query) {
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

                                    let mut base_msg = BaseMessage::default();
                                    let mut alert_msg = Alert::default();
                                    alert_msg.engine = "Hunts".to_string();
                                    alert_msg.title = "MITRE Technique: T1136 - New Account".to_string();
                                    alert_msg.description = "New Account Created".to_string();
                                    alert_msg.details =  format!{"New User: {}, Domain: {}, Culprit: {}", account_creation.target_user, account_creation.target_domain, account_creation.subject_username }.to_string();
            
                                    base_msg.payload_type = "Alert".to_string();
                                    base_msg.alert = alert_msg;
            
                                    comms::send_alert(base_msg.clone()).await;

                                }
                            }
                        }
                    }

                }
                info!("{}", "MITRE Technique: T1136 - New Account");
                info!{"New User: {}, Domain: {}, Culprit: {}", account_creation.target_user, account_creation.target_domain, account_creation.subject_username };
            }          
        }
        Err(e) => info!("Error: {}", e),
    }
}