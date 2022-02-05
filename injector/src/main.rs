#![allow(dead_code, unused_imports, unused_variables, unused_mut, non_snake_case)]
#[macro_use] extern crate prettytable;
extern crate winapi;
use crate::utilities::util::ProcessInformationIterator;
pub mod inject;
pub mod utilities;
pub mod debugger;
//use crate::utilities::elevate::se_priv_enable;
use crate::inject::injector::*;
use crate::debugger::debug::run_debugger;

use log::*;
use std::fs::File;
use std::io::Read;
use tokio::spawn;
use std::fs;

#[tokio::main]
async fn main(){
    env_logger::init();

    //let err = se_priv_enable("SeDebugPrivilege".to_string());
    //println!("ERR: {}", err);

    let filename = "hook.dll".to_string();
    let file_vec = get_file_as_byte_vec(&filename).await;
    let path = std::path::Path::new("hook.dll");
    loop {
        for process_information in ProcessInformationIterator::new() {
            if process_information.name.contains("ransomware"){
                println!("PROCESS: {}, PID: {}", process_information.pid, process_information.name);
                //inject(file_vec.clone(), process_information.pid);
                debugger::process::enum_symbols(process_information.pid);
                
                //dj_inject(path.clone(), process_information.pid);
                use std::{thread, time};

                let ten_millis = time::Duration::from_millis(10);
                let now = time::Instant::now();
        
                thread::sleep(ten_millis);
                //run_debugger(process_information.pid);
                
            }
        }
    }

    /*
    loop {
        for process_information in ProcessInformationIterator::new() {
            if process_information.name.contains("YourPhone"){
                println!("PROCESS: {}, PID: {}", process_information.pid, process_information.name);
                //inject(file_vec.clone(), process_information.pid);
                dj_inject(path.clone(), process_information.pid)
            }
        }
        use std::{thread, time};

        let ten_millis = time::Duration::from_millis(10);
        let now = time::Instant::now();

        thread::sleep(ten_millis);
    }
    */
}


async fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).unwrap();
    let metadata = fs::metadata(&filename).unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}