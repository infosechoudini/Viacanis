#![feature(const_fn, recover)]
use detour::static_detour;
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

static_detour! {
    static CryptAcquireContextAHook: unsafe extern "system" fn(HCRYPTPROV, LPCSTR, LPCSTR, DWORD, DWORD) -> bool;
}


type FnCryptAcquireContextA = unsafe extern "system" fn(HCRYPTPROV, LPCSTR, LPCSTR, DWORD, DWORD)  -> bool;

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

pub fn get_api_run(){
  let mut file = File::create("C:\\Users\\user01\\Desktop\\log.txt").unwrap();

  let mut check: Vec<String> = Vec::new();
  let mut file_access_check: Vec<String> = Vec::new();
  let mut create_snapshot = false;
  let address = get_module_symbol_address("advapi32.dll", "CryptAcquireContextA");
  if address.is_some() {
      message_box("GOTCHA", "advapi32.dll LOADED");
      println!("ADVAPI32 LOADED");
      check.push("advapi32".to_string());
      message_box("Address", &address.unwrap().to_string());
  } 
  let address = get_module_symbol_address("bcryptprimitives.dll", "BCryptEncrypt");
  if address.is_some() {
    message_box("GOTCHA", "bcryptprimitives.dll LOADED");
    message_box("Address", &address.unwrap().to_string());
    check.push("bcryptprimitives".to_string());

  }

  let address = get_module_symbol_address("ncrypt.dll", "NCryptImportKey");
  if address.is_some() {
    message_box("GOTCHA", "ncrypt.dll LOADED");
    message_box("Address", &address.unwrap().to_string());
    check.push("ncrypt".to_string());

  } 

  let address = get_module_symbol_address("cryptbase.dll", "cryptencrypt");
  if address.is_some() {
    message_box("GOTCHA", "cryptbase.dll LOADED");
    message_box("Address", &address.unwrap().to_string());
    check.push("cryptbase".to_string());

  } 
  let module_handle_check = get_module("cryptbase.dll");
  if module_handle_check == true{
    message_box("GOTCHA", "cryptbase.dll LOADED");
    println!("CRYPTBASE LOADED");
    check.push("cryptbase".to_string());

  }
  let module_handle_check = get_module("ncrypt.dll");
  if module_handle_check == true{
    message_box("GOTCHA", "ncrypt.dll LOADED");
    println!("NCRYPT LOADED");
    check.push("ncrypt".to_string());

  }  
  let module_handle_check = get_module("bcryptprimitives.dll");
  if module_handle_check == true{
    message_box("GOTCHA", "bcryptprimitives.dll LOADED");
    println!("BCRYPTPRIMITIVES LOADED");
    check.push("bcryptprimitives".to_string());
  }

  let address = get_module_symbol_address("kernel32.dll", "CreateFileA");
  if address.is_some() {
    file_access_check.push("CreateFileA".to_string());

  } 

  let address = get_module_symbol_address("kernel32.dll", "CreateFileA");
  if address.is_some() {
    file_access_check.push("CreateFileA".to_string());

  } 
  let address = get_module_symbol_address("kernel32.dll", "DeleteFileA");
  if address.is_some() {
    file_access_check.push("DeleteFileA".to_string());

  } 
  let address = get_module_symbol_address("kernel32.dll", "WriteFile");
  if address.is_some() {
    file_access_check.push("WriteFile".to_string());

  } 

  let address = get_module_symbol_address("kernel32.dll", "CreateToolhelp32Snapshot");
  if address.is_some() {
    create_snapshot = true;

  } 
  if create_snapshot == true {
    if &check.len() > &0 {
        if &file_access_check.len() > &0 {
        file.write_all(b"*************ALERT: MULTIPLE CRYPTO MODULES LOADED*************");
        file.write_all(b"Engine: Process Monitor, Method: DLL Injection");
        file.write_all(b"CRYPTO MODULES LOADED");
        for x in check{
          file.write_all((format!("DLL: {}", x)).as_bytes());
        }
      }
    }
  }

}
