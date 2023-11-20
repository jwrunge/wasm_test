use std::ffi::{CStr, CString};
use std::os::raw::c_char;

const SOME_MEMORY_ALLOCATION: &str = "Will this show up?";
const INTS_IN_MEMORY: i32 = 100;

#[no_mangle]
pub fn greet() -> String {
    println!("{}, {}", SOME_MEMORY_ALLOCATION, INTS_IN_MEMORY);
    let _int = INTS_IN_MEMORY.clone();
    let _str = SOME_MEMORY_ALLOCATION.clone();
    let s = format!("Hello!");
    s
}

#[no_mangle]
pub extern "C" fn greet_len(name: *const c_char) -> u8 {
    let c_str = unsafe { CStr::from_ptr(name) };
    let str_slice: &str = c_str.to_str().unwrap();
    let s: String = str_slice.to_owned();  // Use str_buf as needed
    let len = s.len() as u8;

    len
}

static HELLO: &'static str = "hello from rust";

#[no_mangle]
pub extern "C" fn get_hello() -> *mut c_char {
    let s = CString::new(HELLO).unwrap();
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn get_hello_len() -> usize {
    HELLO.len()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//        greet("world");
//     }
// }
