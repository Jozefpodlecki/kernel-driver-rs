#![no_std]

extern crate alloc;

use wdk_sys::{
   PDRIVER_OBJECT,
   NTSTATUS,
   PCUNICODE_STRING,
};

use alloc::{
    ffi::CString,
    slice,
    string::String,
};
use wdk_sys::ntddk::DbgPrint;

#[cfg(not(test))]
extern crate wdk_panic;

#[cfg(not(test))]
use wdk_alloc::WdkAllocator;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

#[unsafe(export_name = "DriverEntry")]
pub unsafe extern "system" fn driver_entry(
   driver: PDRIVER_OBJECT,
   registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
   let string = CString::new("Hello World!\n").unwrap();


    unsafe {
        DbgPrint(c"%s".as_ptr().cast(), string.as_ptr());
    }

   0
}