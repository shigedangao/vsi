use std::mem::transmute_copy;

use windows::Win32::System::Threading::GetCurrentProcessId;
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowThreadProcessId};
use windows::Win32::Foundation::{HWND, SetLastError, ERROR_SUCCESS, BOOL, LPARAM, GetLastError};

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
struct EnumData {
    process_id: u32,
    hwnd: HWND
}

fn get_process_id() -> u32 {
    unsafe {
        GetCurrentProcessId()
    }
}

unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let mut data: *mut EnumData = lparam.0 as *mut EnumData;
    let mut d = *data;
    let mut process_id: u32 = 0;
    GetWindowThreadProcessId(hwnd, &mut process_id);

    println!("{:?} / {:?}", d.process_id, process_id);
    if d.process_id == process_id {
        d.hwnd = hwnd;
        SetLastError(ERROR_SUCCESS);

        return BOOL(0);
    }

    BOOL(1)
}

fn find_window_from_process_id(process_id: u32) -> Option<HWND> {
    let ed = EnumData{ process_id, ..Default::default() };
    let ed_isize = &ed as *const _ as isize;
    
    println!("{:?}", ed);
    unsafe {
        let res = EnumWindows(Some(enum_proc), LPARAM(ed_isize)).as_bool();
        if !res && GetLastError() == ERROR_SUCCESS {
            return Some(ed.hwnd);
        }
    }

    None
}

pub fn find_window_from_process() -> Option<HWND> {
    let process_id = get_process_id();
    find_window_from_process_id(process_id)
}