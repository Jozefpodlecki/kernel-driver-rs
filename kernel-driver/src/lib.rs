#![no_std]
#![allow(unsafe_op_in_unsafe_fn)]

extern crate alloc;

mod thread;

#[cfg(not(test))]
extern crate wdk_panic;

#[cfg(not(test))]
use wdk_alloc::WdkAllocator;
use wdk_sys::GUID;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

const GUID_DEVINTERFACE: GUID = GUID {
    Data1: 0xA1B2_C3D4u32,
    Data2: 0xE5F6u16,
    Data3: 0x7890u16,
    Data4: [0x12u8, 0x34u8, 0x56u8, 0x78u8, 0x9Au8, 0xBCu8, 0xDEu8, 0xF0u8],
};
