// T1053 - Scheduled Tasks
// Tactic::Persistence / Tactic::Execution / Tactic::PrivilegeEscalation
use crate::eventlog::subscriber;

use async_std::task;
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
use log::*;
use std::io;
use smol::Timer;
use futures::Future;
use futures::task::Poll;
use futures::task::Context;
use std::pin::Pin;
use futures::future;

#[derive(Deserialize, Default, Debug)]
struct ScheduledTask {
    taskname: String,
    taskcontent: String,
    subject_username: String,
}


pub async fn monitor_scheduled_tasks(){
    info!{"SCHEDULE TASKS"};
    let ten_millis = Duration::from_millis(10);

    let id = 4698 as u32;
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
                        let mut sched_task = ScheduledTask::default();
                        for data_key in data.data.iter(){
                            for xmldata in &data_key.sub_elements{
                                if xmldata.name == "EventData"{
                                    for subelement in &xmldata.sub_elements{
                                        for (att_key, att_value) in &subelement.attributes{
                                            match att_value.as_str(){
                                                "TaskName" => {sched_task.taskname = subelement.data.as_ref().unwrap().to_string()}
                                                "TaskContent" => {sched_task.taskcontent = subelement.data.as_ref().unwrap().to_string()}
                                                "SubjectUserName" => {sched_task.subject_username = subelement.data.as_ref().unwrap().to_string()}
                                                 _ => continue,
                                            }
                                        }
                                    }
                                }
                            }

                        }
                        info!("{}", "MITRE Technique: T1053 - Scheduled Tasks");
                        info!{"Task Name: {}, TaskContent: {}, Username: {}", sched_task.taskname, sched_task.taskcontent, sched_task.subject_username };
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

pub async fn hunt_scheduled_tasks(){

    info!{"SCHEDULE TASKS"};

    let id = 4698 as u32;
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
                let mut sched_task = ScheduledTask::default();
                for data_key in data.data.iter(){
                    for xmldata in &data_key.sub_elements{
                        if xmldata.name == "EventData"{
                            for subelement in &xmldata.sub_elements{
                                for (att_key, att_value) in &subelement.attributes{
                                    match att_value.as_str(){
                                        "TaskName" => {sched_task.taskname = subelement.data.as_ref().unwrap().to_string()}
                                        "TaskContent" => {sched_task.taskcontent = subelement.data.as_ref().unwrap().to_string()}
                                        "SubjectUserName" => {sched_task.subject_username = subelement.data.as_ref().unwrap().to_string()}
                                         _ => continue,
                                    }
                                }
                            }
                        }
                    }

                }
                info!("{}", "MITRE Technique: T1053 - Scheduled Tasks");
                info!{"Task Name: {}, TaskContent: {}, Username: {}", sched_task.taskname, sched_task.taskcontent, sched_task.subject_username };
            }         
        }
        Err(e) => info!("Error: {}", e),
    }
}