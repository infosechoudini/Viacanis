use crate::colors::*;
use crate::hunts;

use smol;
use tokio;

pub async fn monitor_run(){
    colors::note_logger("** Monitoring the System **".to_string());

    tokio::spawn(async {hunts::logonscripts::hunt_logon_scripts().await});
    tokio::spawn(async {hunts::createaccount::hunt_create_accounts().await});
    tokio::spawn(async {hunts::deleteaccount::hunt_delete_accounts().await}).await;


}