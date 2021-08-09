use crate::messages::*;
use crate::colors::*;
use crate::standalone_server::*;

use lazy_static::lazy_static;
use linked_hash_map::LinkedHashMap;
use log::info;
use uuid::Uuid;
use std::time;
use std::sync::RwLock;
use crossbeam_channel::{bounded, Sender, Receiver};
use rand::{thread_rng, Rng};
use std::sync::Mutex;
use std::io::Write;


#[derive(Default, Clone, Debug)]
pub struct Job {
    pub job_id:     String,
    pub job_type:   String,
    pub status:     String,
    pub args:       Vec<String>,
    pub created:    Option<time::SystemTime>,
}

#[derive(Default, Clone, Debug)]
pub struct Agent  {
	pub id:            Uuid,       // ID is a Universally Unique Identifier per agent
	pub platform:      String,          // Platform is the operating system platform the agent is running on (i.e. windows)
	pub architecture:  String,          // Architecture is the operating system architecture the agent is running on (i.e. amd64)
	pub username:      String,          // UserName is the username that the agent is running as
	pub hostname:      String,          // HostName is the computer's host name
    pub os_type:       String,
    pub ips:           Vec<String>,        // Ips is a slice of all the IP addresses assigned to the host's interfaces
	pub pid:           u32,             // Pid is the Process ID that the agent is running under
	pub icheckin:      Option<time::SystemTime>,       // iCheckIn is a timestamp of the agent's initial check in time
	pub scheckin:      Option<time::SystemTime>,       // sCheckIn is a timestamp of the agent's last status check in time
	pub version:       String,          // Version is the version number of the Merlin Agent program
	pub build:         String,          // Build is the build number of the Merlin Agent program
	pub waittime:      String,   // WaitTime is how much time the agent waits in-between checking in
	pub maxretry:      i8,             // MaxRetry is the maximum amount of failed check in attempts before the agent quits
	pub failedcheckin: i8,             // FailedCheckin is a count of the total number of failed check ins
	pub skew:          i64,           // Skew is size of skew added to each WaitTime to vary check in attempts
	pub proto:         String,          // Proto contains the transportation protocol the agent is using (i.e. h2 or hq)
	pub initial:       bool,            // initial identifies if the agent has successfully completed the first initial check in
	pub killdate:      i64,           // killDate is a unix timestamp that denotes a time the executable will not run after (if it is 0 it will not be used)
	pub secret:        Vec<u8>,          // secret is used to perform symmetric encryption operations
    pub psk:           String,          // Pre-Shared Key
    pub send_channel:  Option<Sender<Job>>,
    pub recv_channel:  Option<Receiver<Job>>,

}
lazy_static! {
    pub static ref AGENTS: RwLock<Mutex<LinkedHashMap<usize, Agent>>> = RwLock::new(Mutex::new(LinkedHashMap::new()));
}


pub fn new_agent(msg: messages::BaseMessage){
    let length = AGENTS.read().unwrap().lock().unwrap().clone().len();
    let agents = AGENTS.write().unwrap();
    let mut agent = Agent::default();
    let (s,r) = bounded(100);

    agent.id = msg.id;
    agent.architecture = msg.agent_info.sysinfo.architecture;
    agent.platform = msg.agent_info.sysinfo.platform;
    agent.username = msg.agent_info.sysinfo.username;
    agent.hostname = msg.agent_info.sysinfo.hostname;
    agent.os_type = msg.agent_info.os_version;
    agent.ips = msg.agent_info.sysinfo.ips;
    agent.pid = msg.agent_info.sysinfo.pid;
    agent.icheckin = Some(time::SystemTime::now());
    agent.proto = msg.agent_info.proto;
    let session_id = length + 1;
    agent.send_channel = Some(s);
    agent.recv_channel = Some(r);

    agents.lock().unwrap().insert(session_id, agent);
    return;

}

pub fn stats_checkin(orig_msg: messages::BaseMessage) -> messages::BaseMessage {
    //let mut agnlants = agnLANTS.read().unwrap().lock().unwrap();
    for (id, agn) in AGENTS.read().unwrap().lock().unwrap().iter_mut() {
        colors::note_logger(format!("agn ID : {:?}", agn.id.clone() ));
        colors::note_logger(format!("Msg ID : {:?}", orig_msg.id.clone() ));
        if agn.id == orig_msg.id {
            agn.scheckin = Some(time::SystemTime::now());
            if agn.recv_channel.as_ref().unwrap().is_empty() == false {
                let agn_job = agn.recv_channel.as_ref().unwrap().recv().unwrap();
                colors::note_logger(format!("Received Job : {:?}", agn_job.clone() ));
                let mut rmsg = messages::BaseMessage::default();
                rmsg.id = agn.id.clone();
                match agn_job.job_type.clone().as_str(){
                    "ls" => {
                        rmsg.payload_type = "NativeCmd".to_string();
                        rmsg.native_cmd.job = agn_job.job_id;
                        rmsg.native_cmd.command = agn_job.job_type.clone();
                        rmsg.native_cmd.args = agn_job.args;
                        return rmsg;
                    }

                    "cd" => {
                        rmsg.payload_type = "NativeCmd".to_string();
                        rmsg.native_cmd.job = agn_job.job_id;
                        rmsg.native_cmd.command = agn_job.job_type.clone();
                        rmsg.native_cmd.args = agn_job.args;
                        return rmsg;
                    }
                    "pwd" => {
                        rmsg.payload_type = "NativeCmd".to_string();
                        rmsg.native_cmd.job = agn_job.job_id;
                        rmsg.native_cmd.command = agn_job.job_type.clone();
                        rmsg.native_cmd.args = agn_job.args;
                        return rmsg;
                    }
                    "cmd" => {
                        rmsg.payload_type = "CmdPayload".to_string();
                        rmsg.cmd_payload.job = agn_job.job_id;
                        rmsg.cmd_payload.command = agn_job.job_type;
                        rmsg.cmd_payload.payload = agn_job.args;
                        return rmsg;
                    }
                    "load" => {
                        rmsg.payload_type = "Module".to_string();
                        rmsg.module.job = agn_job.job_id;
                        rmsg.module.command = "load".to_string();
                        rmsg.module.args = agn_job.args;
                        return rmsg;
                    }
                    "use" => {
                        rmsg.payload_type = "Module".to_string();
                        rmsg.module.job = agn_job.job_id;
                        rmsg.module.command = "use".to_string();
                        rmsg.module.args = agn_job.args;
                        return rmsg;
                    }
                    "move" => {
                        let interface =   &*INTERFACE.lock().unwrap().clone();
                        let mut interface_vec: Vec<String> = Vec::new();
                        interface_vec.push(interface.to_string());

                        rmsg.payload_type = "Module".to_string();
                        rmsg.module.job = agn_job.job_id;
                        rmsg.module.command = "move".to_string();
                        rmsg.module.args = agn_job.args;
                        rmsg.module.interface = interface_vec;
                        return rmsg;
                    }
                    "kill" => {
                        rmsg.payload_type = "AgentControl".to_string();
                        rmsg.agent_ctrl.job = agn_job.job_id;
                        rmsg.agent_ctrl.command = "kill".to_string();
                        let agnlants = AGENTS.write().unwrap();
                        agnlants.lock().unwrap().remove(id);
                        return rmsg;
                    }
                    _ => {
                        rmsg.payload_type = "ServerOk".to_string();
                        return rmsg;
                    }
                }
            } else {
                let mut rmsg = messages::BaseMessage::default();
                rmsg.payload_type = "ServerOk".to_string();
                return rmsg;
            }
        }
    }

    let mut rmsg = messages::BaseMessage::default();
    rmsg.payload_type = "ServerOk".to_string();
    return rmsg;

}


pub fn add_job(agnlant_id: Uuid, cmd: Vec<String>){
    let agnlants = AGENTS.read().unwrap().lock().unwrap().clone();
    let rand_string: String = thread_rng()
    .sample_iter(&rand::distributions::Alphanumeric)
    .take(15)
    .collect();

    let mut new_job = Job::default();
    new_job.job_id = rand_string;
    new_job.job_type = cmd[0].to_string();
    new_job.status = "Created".to_string();
    new_job.args = cmd[1 .. cmd.len()].to_vec();
    new_job.created = Some(time::SystemTime::now());

    if agnlant_id == Uuid::nil(){
        return;
    } else{
        if agnlants.len() < 1 {
            return;
        }

        for (id, agn) in agnlants.iter() {
            if agn.id == agnlant_id{
                agn.send_channel.clone().unwrap().send(new_job.clone()).unwrap();
                colors::note_logger(format!("Session: {}, Time: {:?}, {:?}", id, time::SystemTime::now(), new_job.clone()));
                return
            }
        }
    }
}

pub fn job_results(orig_msg: messages::BaseMessage) {
    let results_vec = orig_msg.cmd_result.stdout.clone();
    let results = results_vec.join("\n\r");
    colors::note_logger(format!("Job Results: {:#?} for Uuid: {:#?}", orig_msg.cmd_result.job.clone(), orig_msg.id));
    std::io::stdout().write_all(results.as_bytes()).unwrap();
}

pub fn is_agent(agent_id: Uuid) -> bool{

    let agents = AGENTS.read().unwrap().lock().unwrap().clone();
    for (_id, agent) in agents.iter() {
        if agent_id == agent.id{
            return true;
        }
    }
    return false;
}

pub fn handle_alerts(orig_msg: messages::BaseMessage){
    let alert_engine = orig_msg.alert.engine;
    let alert_title = orig_msg.alert.title;
    let alert_details = orig_msg.alert.details;
    let agent_id = orig_msg.id;
    let agents = AGENTS.read().unwrap().lock().unwrap().clone();
    for (_id, agent) in agents.iter(){
        if agent_id == agent.id{
            let ips = agent.ips.clone();
            colors::note_logger(format!("Alert Engine: {:#?} Agent: {:#?}", alert_engine, ips[1]));
            colors::note_logger(format!("Alert Title: {:#?} Details: {:#?}", alert_title, alert_details));
        }
    }
    colors::note_logger(format!("Alert Engine: {:#?}", alert_engine));
    colors::note_logger(format!("Alert Title: {:#?} Details: {:#?}", alert_title, alert_details));

    return;

}