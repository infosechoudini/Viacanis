//LOCAL IMPORTS
use crate::hunts;
use crate::messages::messages::*;
use crate::user::core;
use crate::user::comms;


//DEPENDENCIES
use log::*;
use async_std::task;
use std::time::Duration;
use lazy_static::lazy_static;
use std::sync::Mutex;
use tokio::*;
use futures::future::*;
lazy_static! {
    pub static ref INTERFACE: Mutex<String> = Mutex::new(String::new());
}

pub async fn monitor_run(server_ip: String){
    *INTERFACE.lock().unwrap() = server_ip.clone();

    info!("** Monitoring the System **");

    let agent = core::new_agent();
    let agent_msg = core::initial_checkin(agent);
    comms::send_message(agent_msg);


    let handles = vec![
        tokio::spawn(async { hunts::logonscripts::monitor_logon_scripts().await;}),
        tokio::spawn(async { hunts::createaccount::monitor_create_accounts().await;}),
        tokio::spawn(async { hunts::deleteaccount::monitor_delete_accounts().await;}),
        tokio::spawn(async { hunts::scheduletasks::monitor_scheduled_tasks().await;}),
    ];


    join_all(handles).await;

    let dur = Duration::from_millis(1000);
    tokio::time::sleep(dur).await;
}



pub async fn hunt_run(){
    info!("** Hunting the System **");
    tokio::spawn( async {
        hunts::logonscripts::hunt_logon_scripts().await;   
        hunts::createaccount::hunt_create_accounts().await;
        hunts::deleteaccount::hunt_delete_accounts().await;
        hunts::scheduletasks::hunt_scheduled_tasks().await;
        }).await;
}


