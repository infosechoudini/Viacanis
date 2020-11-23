use crate::hunts;

use log::*;
use async_std::task;

pub async fn monitor_run(){
    info!("** Monitoring the System **");
    task::spawn( async {hunts::logonscripts::monitor_logon_scripts().await});   
    task::spawn( async {hunts::createaccount::monitor_create_accounts().await});
    task::spawn( async {hunts::deleteaccount::monitor_delete_accounts().await});
    task::spawn( async {hunts::scheduletasks::monitor_scheduled_tasks().await});

}



pub async fn hunt_run(){
    info!("** Hunting the System **");
    task::spawn( async {hunts::logonscripts::hunt_logon_scripts().await});   
    task::spawn( async {hunts::createaccount::hunt_create_accounts().await});
    task::spawn( async {hunts::deleteaccount::hunt_delete_accounts().await});
    task::spawn( async {hunts::scheduletasks::hunt_scheduled_tasks().await});
}
