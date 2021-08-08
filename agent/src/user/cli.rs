use crate::colors::*;


//use prettytable::{Table, Row, Cell};
use shellwords::split;
use rustyline::Editor;
use std::fs::File;
use lazy_static::lazy_static;
use std::sync::{RwLock, Arc};
//use colored::*;

lazy_static! {
    pub static ref SHELLMENUCONTEXT: Arc<RwLock<&'static str>> = Arc::new(RwLock::new("main"));
    pub static ref PROMPT: Arc<RwLock<String>> = Arc::new(RwLock::new("Threat Hunting".to_string()));


}

pub fn start_cli(){
    let mut rl = Editor::<()>::new();
    if rl.load_history("./agent_history.txt").is_err() {
        println!("No previous history.");
        File::create("./agent_history.txt").expect("Couldn't create history file");
    }

    loop {
        let tmp = PROMPT.clone();
        let line = rl.readline(&tmp.read().unwrap());
        let cmdline = split(&line.unwrap());
        if cmdline.clone().unwrap().len() > 0{
            let cmd = &cmdline.clone().unwrap();
            if cmd.len() > 0 {
                let context = SHELLMENUCONTEXT.clone();
                let menu_context = context.read().unwrap();
                match *menu_context{
                    "main" =>{
                        if cmd.len() > 0{
                            match cmd[0].as_str(){
                                _ => {
                                    colors::error_logger("Empty Command: Execute Commands or Get Info, Probably".to_string());
                                    continue;

                                }
                            }
                        }
                    }
                    _ => {
                        colors::error_logger("No Command".to_string());
                    }
                }
            }
        }
    }

}