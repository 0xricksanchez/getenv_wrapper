use libc::c_char;
use std::ffi::CStr;
use std::io::{self, Read};

/// # Safety
///
/// This function tries to intercept the getenv call to the environment variable "TZ"
/// while returning the set values for all others regularly
#[no_mangle] 
pub unsafe extern "C" fn getenv(name: *const c_char) -> *const c_char {
    let name_str = CStr::from_ptr(name).to_str();
    if name_str == Ok("TZ") {
        println!("[+] Intercepted a getenv() call for: {:?}", name_str);
        let mut p = vec![0u8; 512];
        let stdin = io::stdin();
        let sz = stdin.lock().read(&mut p).unwrap();
        
        p.truncate(sz);
        p.push(b'\0');
        return p.leak().as_ptr().cast();
    } else if name_str.is_err() {
        return vec![0u8; 1].leak().as_ptr().cast()
    } else {
        println!("[+] Ignoring a getenv() call for: {:?}", name_str);
        let mut next = libc_environ;
        while !next.is_null() && !{ *next }.is_null() {
            let env = CStr::from_ptr(*next);
            let env_str = env.to_str();
            let (key, val) =  match env_str {
                Ok(s) => {
                    let parts = s.split_once("=");
                    match parts {
                        Some(x) => { (x.0, x.1) },
                        None => { ("", "") }
                    }
                },
                Err(_) => { ("", "") }
            };
            
            if name_str == Ok(key) {
                return val.as_ptr().cast();
            } else {
                next = next.offset(1);
            }
        }
    }
    vec![0u8; 1].leak().as_ptr().cast()
}

/// # Safety
///
/// This function just calls the nightmarish implementation above, be ready
#[no_mangle] 
pub unsafe extern "C" fn secure_getenv(name: *const c_char) -> *const c_char {
    getenv(name)
}

extern {
    #[link_name = "environ"]
    static libc_environ: *const *const std::os::raw::c_char;
}
