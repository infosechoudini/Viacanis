// T1053 - Scheduled Tasks
// Tactic::Persistence / Tactic::Execution / Tactic::PrivilegeEscalation

//LOCAL IMPORTS
use crate::eventlog::subscriber;
use crate::eventlog::api;
use crate::messages::messages::*;
use crate::user::comms;

//DEPENDENCIES
use async_std::task;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use win_event_log::prelude::{QueryItem, QueryList, Query, Condition, EventFilter};
use xmlJSON::XmlDocument;
use std::str::FromStr;
use log::*;
use tokio::*;



#[derive(Serialize, Deserialize, Default, Debug)]
struct ScheduledTask {
    taskname: String,
    taskcontent: String,
    subject_username: String,
}


pub async fn monitor_scheduled_tasks(){
    info!{"ATTACHED FUTURE: SCHEDULE TASKS"};
    let id = 4698 as u32;
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
                        let mut sched_task = ScheduledTask::default();
                        for data_key in data.data.iter(){
                            for xmldata in &data_key.sub_elements{
                                if xmldata.name == "EventData"{
                                    for subelement in &xmldata.sub_elements{
                                        for (_att_key, att_value) in &subelement.attributes{
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
                        
                        let mut base_msg = BaseMessage::default();
                        let mut alert_msg = Alert::default();
                        alert_msg.engine = "Hunts".to_string();
                        alert_msg.title = "MITRE Technique: T1053 - Scheduled Tasks".to_string();
                        alert_msg.description = "Scheduled Tasks".to_string();
                        alert_msg.details =  format!{"Task Name: {}, TaskContent: {}, Username: {}", sched_task.taskname, sched_task.taskcontent, sched_task.subject_username}.to_string();

                        base_msg.payload_type = "Alert".to_string();
                        base_msg.alert = alert_msg;

                        comms::send_alert(base_msg.clone()).await;
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

    info!{"ATTACHED FUTURE: SCHEDULE TASKS"};

    let id = 4698 as u32;
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

    match api::WinEvents::get(query) {
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
                                for (_att_key, att_value) in &subelement.attributes{
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
            
                let mut base_msg = BaseMessage::default();
                let mut alert_msg = Alert::default();
                alert_msg.engine = "Hunts".to_string();
                alert_msg.title = "MITRE Technique: T1053 - Scheduled Tasks".to_string();
                alert_msg.description = "Scheduled Tasks".to_string();
                alert_msg.details =  format!{"Task Name: {}, TaskContent: {}, Username: {}", sched_task.taskname, sched_task.taskcontent, sched_task.subject_username}.to_string();

                base_msg.payload_type = "Alert".to_string();
                base_msg.alert = alert_msg;

                comms::send_alert(base_msg.clone()).await;
            }         
        }
        Err(e) => info!("Error: {}", e),
    }
}