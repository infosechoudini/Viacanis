#![feature(const_fn, recover)]
use colour::red_ln;
use libc::c_void;
#[macro_use]
use prettytable::{Table, Row, Cell};
use prettytable::{Attr, color};
use win32_error::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
extern crate winapi;
use winapi::um::wincrypt::{
    CryptAcquireContextA, CryptDecrypt, CryptDestroyKey, CryptEncrypt, CryptExportKey, CryptGenKey,
    CryptImportKey, CryptReleaseContext, CALG_AES_192, CRYPT_EXPORTABLE, CRYPT_VERIFYCONTEXT,
    HCRYPTKEY, HCRYPTPROV, PLAINTEXTKEYBLOB, PROV_RSA_AES,
};
use winapi::um::winnt::{
    DELETE, FILE_ATTRIBUTE_NORMAL, FILE_READ_DATA, FILE_SHARE_READ, FILE_WRITE_DATA, HANDLE,
};
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE, UINT, FALSE};
use winapi::shared::ntdef::LPCSTR;

use crate::message_box;

use serde_yaml;
use serde_yaml::Value;
use yaml_rust::yaml::{Hash, Yaml};
use yaml_rust::YamlLoader;
use std::io::prelude::*;
use walkdir::WalkDir;
use lazy_static::*;
use std::sync::Mutex;


lazy_static! {
  static ref SYMBOLS: Mutex<HashMap<String, bool>> = Mutex::new(HashMap::new());

}

#[derive(Default, Debug, Clone)]
pub struct Features{
    rule_name: String,
    condition: String,
    apis: Vec<API>,
}

#[derive(Default, Debug, Clone)]
pub struct API{
    api: String,
}

pub fn get_api_run(){
  let mut file = File::create("C:\\Users\\user01\\Desktop\\log.txt").unwrap();
  
  enum_symbols();
  //let features = get_rules();



}

pub fn run_rules(rules: (Vec<Features>, String)){
  let (rules, name) = rules;
  let mut conditional_names_len = 0;
  let mut condition_results: HashMap<String, String> = HashMap::new();
  for feature in rules.clone(){
    let name = feature.condition;
    conditional_names_len += 1;
    for api in feature.apis{
      let api_strings: Vec<&str> = api.api.split(".").collect();
      let library = api_strings[0].to_owned();
      let symbol = api_strings[1].to_owned();
      let addr = get_module_symbol_address(&library, &symbol);
      if addr.is_some(){
        SYMBOLS.lock().unwrap().insert(symbol.clone(), true);
        condition_results.insert(name.to_owned(), symbol);
      }
    }
  }

  let mut table = Table::new();
  table.add_row(row![bFg->"Condition", bFg->"Symbols"]);
  let mut is_condition_met = 0;

  for feature in rules.clone(){
    for api in feature.apis {
      let api_strings: Vec<&str> = api.api.split(".").collect();
      let library = api_strings[0].to_owned();
      let symbol = api_strings[1].to_owned();
      for check_symbol in SYMBOLS.lock().unwrap().keys(){
        if check_symbol == &symbol{
          table.add_row(row![bFg->feature.condition, bFg->symbol]);
        }
      }
    }
  }


  message_box("ALERT", "FOUND SOME STUFF");
  red_ln!("THREAT BEHAVIOR ALERT: {}", name.to_string());
  table.printstd();

  return;

}
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::winnt::PROCESS_CREATE_THREAD;
use winapi::um::winnt::PROCESS_VM_OPERATION;
use winapi::um::winnt::PROCESS_VM_WRITE;
use winapi::um::winnt::PROCESS_VM_READ;
use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
use winapi::um::winnt::PROCESS_ALL_ACCESS;

fn enum_symbols(){
  unsafe {
    use std::process::id;

   // let phandle = winapi::um::processthreadsapi::OpenProcess(PROCESS_ALL_ACCESS,0 , std::process::id());
    println!("{:#?}", std::process::id());
    let ori_handle = winapi::um::processthreadsapi::GetCurrentProcess();
    let mut  phandle: winapi::shared::ntdef::HANDLE = std::mem::zeroed();
    winapi::um::handleapi::DuplicateHandle(ori_handle, ori_handle, ori_handle , &mut phandle, 0, FALSE, winapi::um::winnt::DUPLICATE_SAME_ACCESS);

    let mask = "!"
    .encode_utf16()
    .chain(std::iter::once(0))
    .collect::<Vec<u16>>();

    

    let module_snap = winapi::um::tlhelp32::CreateToolhelp32Snapshot(winapi::um::tlhelp32::TH32CS_SNAPMODULE, std::process::id());
    if module_snap == winapi::um::handleapi::INVALID_HANDLE_VALUE{
      let err = Win32Error::new();
      println!("EVENT ERROR: {}", err);
      return;
    }
    let mut me32: winapi::um::tlhelp32::LPMODULEENTRY32= std::mem::zeroed();

    (*me32).dwSize = std::mem::size_of::<winapi::um::tlhelp32::MODULEENTRY32> as u32; 


    let me_first_bool = winapi::um::tlhelp32::Module32First(module_snap, me32);
    if me_first_bool == 0{

      println!("BAD SNAPSHOT");
      let err = Win32Error::new();
      println!("EVENT ERROR: {}", err);
      return;
    }

    while winapi::um::tlhelp32::Module32Next(module_snap, me32) == 1{
      let module_vec: Vec<u8> = std::mem::transmute((*me32).szModule.to_vec());
      let name_of_module = String::from_utf8(module_vec).unwrap();

      println!("MODULE NAME: {:#?}", name_of_module );
      let module_base_addr = (*me32).modBaseAddr;
      let status = winapi::um::dbghelp::SymEnumSymbolsW(phandle, module_base_addr as u64, mask.as_ptr(), Some(enum_sym_proc), std::ptr::null_mut());
      if status == 0 {
        let err = Win32Error::new();
        println!("EVENT ERROR: {}", err);
      }
    }
  }

}

#[repr(C)]
struct RustObject {
    psyminfo: winapi::um::dbghelp::PSYMBOL_INFOW,
    symbolsize: u64, 
    usercontext: winapi::um::winnt::PVOID,
    // Other members...
}
fn from_wide_string(s: &[u16]) -> String 
{ 
	use std::ffi::OsString; 
	use std::os::windows::ffi::OsStringExt; 
	let slice = s.split(|&v| v == 0).next().unwrap(); 
	OsString::from_wide(slice).to_string_lossy().into() 
}

#[no_mangle]
unsafe extern "system" fn enum_sym_proc(psyminfo: winapi::um::dbghelp::PSYMBOL_INFOW, symbolsize: u32, usercontext: winapi::um::winnt::PVOID) -> BOOL{
  unsafe {
    println!("IN CALLBACK");
    message_box("CALLBACK", "IN CALLBACK");
    let syminfo = psyminfo;
    let wchar_name = (*syminfo).Name;
    println!("CHAR LEN: {:#}", wchar_name.len());
    let name = from_wide_string(&wchar_name);
    println!("ADDRESS: {:#?}, NAME: {:#?}", (*syminfo).Address, name);
    return TRUE;
  }
}

/// Returns a module symbol's absolute address.
fn get_module_symbol_address(module: &str, symbol: &str) -> Option<usize> {
    let module = module
      .encode_utf16()
      .chain(std::iter::once(0))
      .collect::<Vec<u16>>();
    let symbol = std::ffi::CString::new(symbol).unwrap();
    unsafe {
      let handle = winapi::um::libloaderapi::GetModuleHandleW(module.as_ptr());
      match winapi::um::libloaderapi::GetProcAddress(handle, symbol.as_ptr()) as usize {
        0 => None,
        n => Some(n),
      }
    }
  }


pub fn get_module(module: &str) -> bool{
  let module = module
  .encode_utf16()
  .chain(std::iter::once(0))
  .collect::<Vec<u16>>();
  unsafe {
    let handle = winapi::um::libloaderapi::GetModuleHandleW(module.as_ptr());
    if handle != std::ptr::null_mut() {
      return true;
    }

    return false;
  }
}




pub fn get_rules(){
  let none: Option<Vec<Features>> = None;

    for entry in WalkDir::new("C:/Users/user01/Desktop/rules")
    .into_iter()
    .filter_map(Result::ok)
    .filter(|e| !e.file_type().is_dir()){
        let filename = String::from(entry.file_name().to_string_lossy());
        if filename.contains("yaml"){
            let rule_file = std::fs::read_to_string(entry.clone().into_path()).unwrap();
            let parsed_rules = parse_rules(rule_file);
            run_rules(parsed_rules);
        }
    }
}


pub fn parse_rules(rule: String) -> (Vec<Features>, String) {
    let rule_parsed = YamlLoader::load_from_str(&rule).unwrap();
    
    let rule = &rule_parsed[0];
    let name = rule["rule"]["meta"]["name"].as_str().unwrap();
    let mut feature_vec = rule["rule"]["features"].as_vec().unwrap();
    let mut features = feature_vec[0].as_hash().unwrap();

    let mut all_conditionals: Vec<Features> = Vec::new();
    // We get the Top Level Feature Keys
    for entry in features.keys().next(){
        match entry.as_str().unwrap(){
            "and" => { 
                let mut second_level_conditonals_vec = features[entry].as_vec().unwrap().clone();
                all_conditionals = parse_features(second_level_conditonals_vec);
            },
            "or" =>{
                let mut second_level_conditonals_vec = features[entry].as_vec().unwrap().clone();
                all_conditionals = parse_features(second_level_conditonals_vec);
 
            },

            _ => break,
        }
    }

    return (all_conditionals, name.to_string());


}

pub fn parse_features(second_level_conditonals_vec: Vec<Yaml>) -> Vec<Features>{
    let mut feature_vec: Vec<Features> = Vec::new();
    for x in 0 .. second_level_conditonals_vec.len(){
        let mut second_level_conditonals = second_level_conditonals_vec[x].as_hash().unwrap().clone();
        for second_level in second_level_conditonals.keys().next(){
            //println!("Name: {:#?}", second_level);
            match second_level.as_str().unwrap(){
                "and" => {
                    let mut third_level_conditionals_len = second_level_conditonals[second_level].as_vec().unwrap().clone().len();
                    let mut api_vec: Vec<API> = Vec::new();
                    let mut conditional_feature: Features = Features::default();
                    let mut x = 0;
                    for x in 0..third_level_conditionals_len {
                        let mut third_level_conditional = second_level_conditonals[second_level][x].as_hash().unwrap().clone();
                        for third_entry in third_level_conditional.entries().next(){
                            match third_entry.key().clone().as_str().unwrap(){
                                "match" => {
                                    let value = third_entry.get().clone();
                                    conditional_feature.condition = value.clone().as_str().unwrap().to_string();
                                },
                                "api" => {
                                    let mut api: API = API::default();
                                    let value = third_entry.get().clone();
                                    api.api = value.clone().as_str().unwrap().to_string();
                                    api_vec.push(api.clone());
                                },

                                _ => {

                                    println!("DO NOTHING");
                                },
                            }
                        }
                    }
                    conditional_feature.apis = api_vec;
                    feature_vec.push(conditional_feature);
                },
                "or" => {
                  let mut third_level_conditionals_len = second_level_conditonals[second_level].as_vec().unwrap().clone().len();
                  println!("Array LEN: {:#?}", third_level_conditionals_len);
                  let mut api_vec: Vec<API> = Vec::new();
                  let mut conditional_feature: Features = Features::default();
                  let mut x = 0;
                  for x in 0..third_level_conditionals_len {
                      let mut third_level_conditional = second_level_conditonals[second_level][x].as_hash().unwrap().clone();
                      for third_entry in third_level_conditional.entries().next(){
                          match third_entry.key().clone().as_str().unwrap(){
                              "match" => {
                                  let value = third_entry.get().clone();
                                  conditional_feature.condition = value.clone().as_str().unwrap().to_string();
                              },
                              "api" => {
                                  let mut api: API = API::default();
                                  let value = third_entry.get().clone();
                                  api.api = value.clone().as_str().unwrap().to_string();
                                  api_vec.push(api.clone());
                              },

                              _ => {

                                  println!("DO NOTHING");
                              },
                          }
                      }
                  }
                  conditional_feature.apis = api_vec;
                  feature_vec.push(conditional_feature);
                },
                _ => continue,
            }
        }      
    }     

    return feature_vec;
}
