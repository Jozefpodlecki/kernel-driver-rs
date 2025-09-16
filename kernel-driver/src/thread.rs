use core::ptr::null_mut;

use wdk::{nt_success, paged_code, println};
use wdk_sys::ntddk::{
    DbgPrint, KeDelayExecutionThread, KeWaitForSingleObject, ObReferenceObjectByHandle, ObfDereferenceObject, ZwClose,
    ZwWaitForSingleObject,
};

use wdk_sys::_KWAIT_REASON::Executive;
use wdk_sys::_MODE::KernelMode;
use wdk_sys::{
    _WDF_EXECUTION_LEVEL, _WDF_SYNCHRONIZATION_SCOPE, DRIVER_OBJECT, HANDLE, LARGE_INTEGER, PVOID, THREAD_ALL_ACCESS,
    ULONG, WDF_DRIVER_CONFIG, WDF_NO_HANDLE, WDF_NO_OBJECT_ATTRIBUTES, WDF_OBJECT_ATTRIBUTES, WDFDEVICE,
    WDFDEVICE_INIT, WDFDRIVER, call_unsafe_wdf_function_binding,
};
use wdk_sys::{NTSTATUS, PCUNICODE_STRING, PDRIVER_OBJECT, STATUS_SUCCESS, ntddk::PsCreateSystemThread};

use alloc::{ffi::CString, slice, string::String};

use crate::GUID_DEVINTERFACE;

static mut THREAD_OBJECT: *mut PVOID = null_mut();
static mut THREAD_HANDLE: HANDLE = null_mut();
static mut THREAD_EXIT: bool = false;

#[unsafe(link_section = "INIT")]
#[unsafe(export_name = "DriverEntry")]
extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, registry_path: PCUNICODE_STRING) -> NTSTATUS {
    println!("Enter: driver_entry");

    let mut driver_config = {
        let wdf_driver_config_size: ULONG;

        const WDF_DRIVER_CONFIG_SIZE: usize = core::mem::size_of::<WDF_DRIVER_CONFIG>();
        const { assert!(WDF_DRIVER_CONFIG_SIZE <= ULONG::MAX as usize) }

        wdf_driver_config_size = WDF_DRIVER_CONFIG_SIZE as ULONG;

        WDF_DRIVER_CONFIG {
            Size: wdf_driver_config_size,
            EvtDriverDeviceAdd: Some(evt_driver_device_add),
            EvtDriverUnload: Some(evt_driver_unload),
            ..WDF_DRIVER_CONFIG::default()
        }
    };

    let driver_handle_output = WDF_NO_HANDLE.cast::<WDFDRIVER>();

    let nt_status = unsafe {
        call_unsafe_wdf_function_binding!(
            WdfDriverCreate,
            driver as PDRIVER_OBJECT,
            registry_path,
            WDF_NO_OBJECT_ATTRIBUTES,
            &raw mut driver_config,
            driver_handle_output,
        )
    };

    if !nt_success(nt_status) {
        println!("Error: WdfDriverCreate failed {nt_status:#010X}");
        return nt_status;
    }

    STATUS_SUCCESS
}

#[unsafe(link_section = "PAGE")]
unsafe extern "C" fn evt_driver_device_add(_driver: WDFDRIVER, mut device_init: *mut WDFDEVICE_INIT) -> NTSTATUS {
    paged_code!();

    println!("Enter: evt_driver_device_add");

    let mut attributes = WDF_OBJECT_ATTRIBUTES {
        Size: core::mem::size_of::<WDF_OBJECT_ATTRIBUTES>() as ULONG,
        ExecutionLevel: _WDF_EXECUTION_LEVEL::WdfExecutionLevelInheritFromParent,
        SynchronizationScope: _WDF_SYNCHRONIZATION_SCOPE::WdfSynchronizationScopeInheritFromParent,
        ..WDF_OBJECT_ATTRIBUTES::default()
    };

    let mut device = WDF_NO_HANDLE as WDFDEVICE;
    let mut nt_status = unsafe {
        call_unsafe_wdf_function_binding!(
            WdfDeviceCreate,
            &raw mut device_init,
            &raw mut attributes,
            &raw mut device,
        )
    };

    if !nt_success(nt_status) {
        println!("Error: WdfDeviceCreate failed {nt_status:#010X}");
        return nt_status;
    }

    let mut client_id = null_mut();

    let nt_status = PsCreateSystemThread(
        &raw mut THREAD_HANDLE as *mut _,
        THREAD_ALL_ACCESS,
        null_mut(), // ObjectAttributes
        null_mut(), // ProcessHandle (NULL = system process)
        client_id,  // ClientId
        Some(thread_routine),
        null_mut(), // Context
    );

    if !nt_success(nt_status) {
        println!("Error: PsCreateSystemThread failed {nt_status:#010X}");
        return nt_status;
    }

    let nt_status = ZwClose(THREAD_HANDLE);

    println!("client_id: {:?}", client_id);

    let nt_status = ObReferenceObjectByHandle(
        THREAD_HANDLE,
        THREAD_ALL_ACCESS,
        null_mut(),
        KernelMode as i8,
        THREAD_OBJECT,
        null_mut(),
    );

    if !nt_success(nt_status) {
        println!("Error: ObReferenceObjectByHandle failed {nt_status:#010X}");
        return nt_status;
    }

    let nt_status = unsafe {
        call_unsafe_wdf_function_binding!(
            WdfDeviceCreateDeviceInterface,
            device,
            &GUID_DEVINTERFACE,
            core::ptr::null_mut(),
        )
    };

    if !nt_success(nt_status) {
        println!("Error: WdfDeviceCreateDeviceInterface failed {nt_status:#010X}");
        return nt_status;
    }

    println!("Exit: evt_driver_device_add");

    nt_status
}

unsafe extern "C" fn evt_driver_unload(_driver: WDFDRIVER) {
    println!("Enter: evt_driver_unload");

    THREAD_EXIT = true;

    if !THREAD_HANDLE.is_null() {
        let mut interval = LARGE_INTEGER { QuadPart: -100_000_000 };
        // let nt_status = ZwWaitForSingleObject(THREAD_HANDLE, 0, &mut interval);

        let nt_status = KeWaitForSingleObject(THREAD_OBJECT as *mut _, Executive, KernelMode as i8, 0, &mut interval);

        if !nt_success(nt_status) {
            println!("KeWaitForSingleObject fail");
            return;
        }

        // let status = ZwClose(THREAD_HANDLE);

        //  if !nt_success(nt_status) {
        //     println!("ZwClose fail");
        //     return;
        // }

        ObfDereferenceObject(THREAD_OBJECT as *mut _);
        THREAD_HANDLE = null_mut();
    }

    println!("Exit: evt_driver_unload");
}

unsafe extern "C" fn thread_routine(context: PVOID) {
    let msg = CString::new("Hello World!\n").unwrap();

    loop {
        if THREAD_EXIT {
            break;
        }

        DbgPrint(msg.as_ptr().cast());

        // Sleep 10 seconds (10000000 * 10 = 100000000 100-ns units)
        let interval: LARGE_INTEGER = LARGE_INTEGER { QuadPart: -100_000_000 };
        let nt_status = KeDelayExecutionThread(KernelMode as i8, 0, &interval as *const _ as *mut _);

        if !nt_success(nt_status) {
            println!("KeDelayExecutionThread fail");
            break;
        }
    }
}
