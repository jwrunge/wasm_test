use std::ptr;

#[no_mangle]
pub fn greet(name: &str) -> *const u8 {
    let s = format!("Hello, {}!", name).to_string();
    let len = s.len() as u8;
    let ptr = ptr::addr_of!(s);

    println!("{:#04x}, {:#04x}", ptr as u8, len);

    s.as_ptr()
}

#[no_mangle]
pub fn greetLen(name: &str) -> u8 {
    let s = format!("Hello, {}!", name).to_string();
    let len = s.len() as u8;
    let ptr = ptr::addr_of!(s);

    println!("{:#04x}, {:#04x}", ptr as u8, len);

    len
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn it_works() {
        let t = greet("world");
        // println!("{:#04x}, {:#04x}", t.0, t.1);
    }
}
