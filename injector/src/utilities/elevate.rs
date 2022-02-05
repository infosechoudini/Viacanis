use winapi::shared::minwindef::{TRUE, BOOL};
use winapi::shared::minwindef::{DWORD, UINT, HINSTANCE, LPVOID};
use winapi::shared::windef::HWND;
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::ntdef::LPSTR;
use std::ffi::{CString, CStr};
use libc::{c_char, c_void};
use std::ffi::OsString;
use std::os::windows::prelude::*;
use log::*;
use std::os::windows::io::*;
use win32_error::*;
use std::ptr;
use std::ffi::OsStr;


#[derive(Default, Clone)]
struct LUID {
    lowpart: u32,
    highpart: i32,
}

#[derive(Clone)]
struct LUID_AND_ATTRIBUTES {
    luid:  *mut winapi::shared::ntdef::LUID,
    attributes: u32,
}

#[derive(Clone)]
#[repr(C)]
struct TOKEN_PRIV {
    privilege_count: u32,
    privileges: LUID_AND_ATTRIBUTES,
}

const SE_PRIVILEGE_ENABLED: u32 = (0x00000002) as u32;

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
  fn to_wstring(str: &str) -> Vec<u16> {
    let v: Vec<u16> = OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();
    v
  }

pub fn se_priv_enable(s: String) -> String{

    let debug_string = to_wstring(&s).as_ptr();
    unsafe {

        let current_process_handle = winapi::um::processthreadsapi::GetCurrentProcess();

        info!("GOT PROCESS");

        let mut luid: winapi::shared::ntdef::LUID = winapi::shared::ntdef::LUID{
            LowPart: 0, 
            HighPart: 0};
        let lookup_priv_resp = winapi::um::winbase::LookupPrivilegeValueW(ptr::null_mut(), debug_string, &mut luid as *mut winapi::shared::ntdef::LUID);

        info!("LOOKUP PRIVS DONE");

        if lookup_priv_resp == 0{
            return "Cannot Get SEDEBUG Privs".to_string();
        }



        let token_priv = winapi::um::winnt::TOKEN_PRIVILEGES{
            PrivilegeCount: 1,
            Privileges: [winapi::um::winnt::LUID_AND_ATTRIBUTES{
                Luid: winapi::shared::ntdef::LUID{
                    LowPart: (luid).LowPart, 
                    HighPart: (luid).HighPart}, 
                Attributes: SE_PRIVILEGE_ENABLED}]};

        let priv_box = Box::new(token_priv);
        let priv_pointer = Box::into_raw(priv_box);


        let adjust_token_bool = winapi::um::securitybaseapi::AdjustTokenPrivileges(current_process_handle, 0, priv_pointer, 0, ptr::null_mut(), ptr::null_mut());

        info!("ADJUSTED TOKEN PRIVS");
        if adjust_token_bool == 0 {
            return "Error Getting Privs".to_string();
        }


        return "".to_string();
    }

}

pub fn impersonate_process(pid: u32){
    unsafe {
        let mut attr: winapi::um::minwinbase::SECURITY_ATTRIBUTES = std::mem::zeroed();
        let mut new_token: winapi::um::winnt::HANDLE = std::mem::zeroed();
        let primary_token = get_primary_token(pid);
        winapi::um::securitybaseapi::ImpersonateLoggedOnUser(*primary_token);
        winapi::um::securitybaseapi::DuplicateTokenEx(*primary_token, winapi::um::winnt::TOKEN_ALL_ACCESS,  &attr, winapi::um::winnt::SecurityDelegation, winapi::um::winnt::TokenPrimary, &new_token);



    }
}

fn get_primary_token(pid: u32) -> winapi::um::winnt::HANDLE{
    unsafe{
        let mut token: winapi::um::winnt::HANDLE = std::mem::zeroed();
        let handle = winapi::um::processthreadsapi::OpenProcess(winapi::um::winnt::PROCESS_QUERY_INFORMATION, TRUE, pid);
        winapi::um::processthreadsapi::OpenProcessToken(handle,winapi::um::winnt::TOKEN_DUPLICATE| winapi::um::winnt::TOKEN_ASSIGN_PRIMARY |winapi::um::winnt::TOKEN_QUERY , &token);
        return token;
    }

}
