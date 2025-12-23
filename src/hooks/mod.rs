#![allow(static_mut_refs)]

use crate::{constants, core::resources};

use std::ffi::CString;
use winapi::ctypes::c_void as winapi_c_void;
use winapi::shared::minwindef::{HGLOBAL, HMODULE, HRSRC};
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};

use retour::GenericDetour;

type FnLoadResource = unsafe extern "C" fn(HMODULE, HRSRC) -> HGLOBAL;
type FnLockResource = unsafe extern "C" fn(HGLOBAL) -> *mut winapi_c_void;
type FnSizeofResource = unsafe extern "C" fn(HMODULE, HRSRC) -> u32;

static mut HOOK_LOAD: Option<GenericDetour<FnLoadResource>> = None;
static mut HOOK_LOCK: Option<GenericDetour<FnLockResource>> = None;
static mut HOOK_SIZE: Option<GenericDetour<FnSizeofResource>> = None;

fn is_target_png(data: &[u8]) -> bool {
    if data.len() < 24 {
        return false;
    }
    // Check PNG signature
    if &data[0..8] != [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
        return false;
    }
    // Check IHDR
    if &data[8..16] != [0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52] {
        return false;
    }
    // Width
    let width = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
    // Height
    let height = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
    width == constants::TARGET_WIDTH && height == constants::TARGET_HEIGHT
}

unsafe extern "C" fn hook_sizeof_resource(hmodule: HMODULE, hrsrc: HRSRC) -> u32 {
    unsafe { HOOK_SIZE.as_ref().unwrap().call(hmodule, hrsrc) }
}

unsafe extern "C" fn hook_load_resource(hmodule: HMODULE, hrsrc: HRSRC) -> HGLOBAL {
    let original_hglobal = unsafe { HOOK_LOAD.as_ref().unwrap().call(hmodule, hrsrc) };
    if !original_hglobal.is_null() {
        let data_ptr = unsafe { HOOK_LOCK.as_ref().unwrap().call(original_hglobal) };
        if !data_ptr.is_null() {
            let size = unsafe { HOOK_SIZE.as_ref().unwrap().call(hmodule, hrsrc) as usize };
            if size > 0 {
                let data = unsafe { std::slice::from_raw_parts(data_ptr as *const u8, size) };
                if is_target_png(data) {
                    log::info!("[+] Replacing target PNG resource with fake");
                    return unsafe { resources::FAKE_BG_RESOURCE as HGLOBAL };
                }
            }
        }
    }
    original_hglobal
}

unsafe extern "C" fn hook_lock_resource(hglobal: HGLOBAL) -> *mut winapi_c_void {
    if hglobal == unsafe { resources::FAKE_BG_RESOURCE as HGLOBAL } {
        unsafe { resources::FAKE_BG_RESOURCE as *mut winapi_c_void }
    } else {
        unsafe { HOOK_LOCK.as_ref().unwrap().call(hglobal) }
    }
}

fn setup_load_resource_hook() {
    unsafe {
        let kernel32_handle = GetModuleHandleA(CString::new("kernel32.dll").unwrap().as_ptr());
        if !kernel32_handle.is_null() {
            let fn_ptr = GetProcAddress(
                kernel32_handle,
                CString::new("LoadResource").unwrap().as_ptr(),
            );
            if !fn_ptr.is_null() {
                let target: FnLoadResource = std::mem::transmute(fn_ptr);
                HOOK_LOAD =
                    Some(GenericDetour::<FnLoadResource>::new(target, hook_load_resource).unwrap());
                HOOK_LOAD.as_mut().unwrap().enable().unwrap();
                log::info!("[+] Created LoadResource hook");
            } else {
                log::info!("[!] Failed to get LoadResource address");
            }
        }
    }
}

fn setup_lock_resource_hook() {
    unsafe {
        let kernel32_handle = GetModuleHandleA(CString::new("kernel32.dll").unwrap().as_ptr());
        if !kernel32_handle.is_null() {
            let fn_ptr = GetProcAddress(
                kernel32_handle,
                CString::new("LockResource").unwrap().as_ptr(),
            );
            if !fn_ptr.is_null() {
                let target: FnLockResource = std::mem::transmute(fn_ptr);
                HOOK_LOCK =
                    Some(GenericDetour::<FnLockResource>::new(target, hook_lock_resource).unwrap());
                HOOK_LOCK.as_mut().unwrap().enable().unwrap();
                log::info!("[+] Created LockResource hook");
            } else {
                log::info!("[!] Failed to get LockResource address");
            }
        }
    }
}

fn setup_sizeof_resource_hook() {
    unsafe {
        let kernel32_handle = GetModuleHandleA(CString::new("kernel32.dll").unwrap().as_ptr());
        if !kernel32_handle.is_null() {
            let fn_ptr = GetProcAddress(
                kernel32_handle,
                CString::new("SizeofResource").unwrap().as_ptr(),
            );
            if !fn_ptr.is_null() {
                let target: FnSizeofResource = std::mem::transmute(fn_ptr);
                HOOK_SIZE = Some(
                    GenericDetour::<FnSizeofResource>::new(target, hook_sizeof_resource).unwrap(),
                );
                HOOK_SIZE.as_mut().unwrap().enable().unwrap();
                log::info!("[+] Created SizeofResource hook");
            } else {
                log::info!("[!] Failed to get SizeofResource address");
            }
        }
    }
}

pub fn setup_hooks() {
    setup_load_resource_hook();
    setup_lock_resource_hook();
    setup_sizeof_resource_hook();

    log::info!("[+] All hooks enabled");
    log::info!("[+] Hooks are now active");
}
