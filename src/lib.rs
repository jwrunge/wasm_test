use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn greet(name: *const c_char) -> *const u8 {
    let c_str = unsafe { CStr::from_ptr(name) };
    let str_slice: &str = c_str.to_str().unwrap();
    let s: String = str_slice.to_owned();  // Use str_buf as needed
    println!("{}", s);

    s.as_ptr()
}

#[no_mangle]
pub extern "C" fn greet_len(name: *const c_char) -> u8 {
    let c_str = unsafe { CStr::from_ptr(name) };
    let str_slice: &str = c_str.to_str().unwrap();
    let s: String = str_slice.to_owned();  // Use str_buf as needed
    let len = s.len() as u8;

    len
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//        greet("world");
//     }
// }
