#![allow(dead_code, unused_imports, unused_variables, unused_mut, non_snake_case)]
#[macro_use] extern crate prettytable;
pub mod hook;
pub mod util;
pub mod processes;
use hook::anti_debugging::*;
use hook::hook_api::*;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use sharedlib::{Lib, Func, Symbol};
use libc::c_int;
use winapi::shared::minwindef::{TRUE, BOOL};
use winapi::shared::minwindef::{DWORD, UINT, HINSTANCE, LPVOID};
use winapi::shared::windef::HWND;
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::ntdef::LPSTR;
use std::ffi::{CString, CStr};
use libc::{c_char, c_void};
use std::ffi::OsString;
use std::os::windows::prelude::*;
use std::process;

const DLL_PROCESS_DETACH: DWORD = 0;
const DLL_PROCESS_ATTACH: DWORD = 1;
const DLL_THREAD_ATTACH: DWORD = 2;
const DLL_THREAD_DETACH: DWORD = 3;

pub trait ToWide {
  fn to_wide(&self) -> Vec<u16>;
  fn to_wide_null(&self) -> Vec<u16>;
}
impl<T> ToWide for T where T: AsRef<OsStr> {
  fn to_wide(&self) -> Vec<u16> {
      self.as_ref().encode_wide().collect()
  }
  fn to_wide_null(&self) -> Vec<u16> {
      self.as_ref().encode_wide().chain(Some(0)).collect()
  }
}
pub trait FromWide where Self: Sized {
  fn from_wide_null(wide: &[u16]) -> Self;
}
impl FromWide for OsString {
  fn from_wide_null(wide: &[u16]) -> OsString {
      let len = wide.iter().take_while(|&&c| c != 0).count();
      OsString::from_wide(&wide[..len])
  }
}
pub fn to_wstring(str: &str) -> Vec<u16> {
  let v: Vec<u16> = OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();
  v
}

pub fn message_box(title: &str, message: &str) {
  unsafe {
    let path_to_lib = "user32.dll";
    let lib = Lib::new(path_to_lib).unwrap();
    let symbol: Func<extern "C" fn(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT)
                                   -> c_int> = lib.find_func("MessageBoxW").unwrap();
    let mbw = symbol.get();
    mbw(0 as HWND,
        to_wstring(message).as_ptr(),
        to_wstring(title).as_ptr(),
        0);
  }
}

extern {
  fn malloc(size: usize) -> *mut u8;
  fn memcpy(dest: *mut u8, src: *const u8, size: usize) -> *mut u8;
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn SAYHELLO(_reserved: *mut c_void, _reserved2: *mut c_void,  userdata: *mut c_void) { //*const c_char

  message_box("INSIDE PROCESS", "INJECTED INTO PROCESS");
  

}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(hinst: winapi::shared::minwindef::HINSTANCE, reason: DWORD, reserved: LPVOID) -> BOOL {
  match reason {
    DLL_PROCESS_DETACH => {}
    DLL_PROCESS_ATTACH => {
      initialize();
      //get_api_run();
      processes::process::get_api_run();
      //modify_peb();
    }
    DLL_THREAD_ATTACH => {}
    DLL_THREAD_DETACH => {}
    _ => {}
  };


  return TRUE;
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn initialize() {
  message_box("INSIDE PROCESS", "INJECTED INTO PROCESS");
}