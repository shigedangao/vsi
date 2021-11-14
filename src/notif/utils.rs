use core::slice;

use uuid::Uuid;
use windows::Win32::UI::Shell::{
    SetCurrentProcessExplicitAppUserModelID,
    GetCurrentProcessExplicitAppUserModelID
};
use windows::Win32::Foundation::PWSTR;

pub fn set_app_model_id() {
    let uuid_string = Uuid::new_v4().to_hyphenated().to_string();
    // println!("{}", uuid_string);
    //let model_id = "CompanyName.ProductName.SubProduct.VersionInformation";
    let mut encoded = uuid_string.encode_utf16().chain([0u16]).collect::<Vec<u16>>();
    let pwstr = PWSTR(encoded.as_mut_ptr());

    unsafe {
        let res = SetCurrentProcessExplicitAppUserModelID(pwstr).unwrap();
    }
}

pub fn get_app_model_id() -> String {
    let mut cb_buffer = 257_u32;
    let pwstr: PWSTR;
    unsafe {
        pwstr = match GetCurrentProcessExplicitAppUserModelID() {
            Ok(res) => res,
            Err(err) => {
                println!("{:?}", err);
                panic!();
            }
        };

        let buffer = slice::from_raw_parts(pwstr.0, cb_buffer as usize - 1);
        let id = String::from_utf16_lossy(buffer);

        println!("id {}", id);
        return id;
    }
}
