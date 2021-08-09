use uuid::Uuid;
use std::time;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub enum PayloadType {
    CmdPayload,
    AgentInfo,
    AgentControl,
    SysInfo,
    Alert,
    CmdResult,
    NativeCmd,
    Module
}


#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct BaseMessage{
    pub id: Uuid,
    pub payload_type: String,
    pub cmd_payload: CmdPayload,
    pub agent_info: AgentInfo,
    pub file_transfer: FileTransfer,
    pub native_cmd: NativeCmd,
    pub agent_ctrl: AgentControl, 
    pub module: Module,
    pub cmd_result: CmdResult,
    pub alert: Alert,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Alert {
    pub engine: String,
    pub title: String,
    pub description: String,    
    pub details: String,
    pub raw: Vec<u8>
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct CmdPayload {
    pub payload: Vec<String>,
    pub command: String,
    pub job: String,

}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct AgentInfo{
    pub	os_version:       String,
    pub	wait_time:      time::Duration,
    pub	failedcheckin: i8,
    pub	proto:         String,
    pub	sysinfo:       SysInfo,
    pub	killdate:      i64,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SysInfo {
    pub	platform:    String,   
    pub	architecture: String,   
    pub	username:     String,  
    pub	hostname:     String,
    pub	pid:          u32,      
    pub	ips:          Vec<String>, 

}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct FileTransfer {
    pub job:          String,
	pub filelocation: String,
	pub fileblob:     String,
	pub isdownload:   bool,
}


#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct CmdResult{
    pub job :    String,
	pub stdout:  Vec<String>,
	pub stderr:  Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct NativeCmd {
	pub job:     String,
    pub command: String,
    pub args:    Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct AgentControl {
    pub job: String,
    pub command: String,
    pub args: String,
    pub result: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Module{
    pub id: Uuid,
    pub payload_type: String,
    pub job: String,
    pub command: String,
    pub args: Vec<String>,
    pub result: String,
    pub interface: Vec<String>,

}

