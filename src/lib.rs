use libc::c_char;
use std::ffi::CStr;
use std::io::{self, Read};


#[no_mangle] 
pub extern "C" fn getenv(name: *const c_char) -> *const c_char {
    let name = unsafe { CStr::from_ptr(name) };
    println!("[+] Intercepted a getenv() call for: {:?}", name);
    
    let mut p = vec![0u8; 512];
    let stdin = io::stdin();
    let sz = stdin.lock().read(&mut p).unwrap();
    
    p.truncate(sz);
    p.push(b'\0');
    p.leak().as_ptr().cast()
}

#[no_mangle] 
pub extern "C" fn secure_getenv(name: *const c_char) -> *const c_char {
    getenv(name)
}
