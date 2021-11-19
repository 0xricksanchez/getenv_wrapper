use libc::c_char;
use std::env::VarError;
use std::ffi::CStr;
use std::io::{self, Read};
use std::str::Utf8Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // CStr to &str conversion error
    UTF8(Utf8Error),

    // Error when reading from stdin
    ReadError,

    // NULL Result
    NullPtr,

    // Could not read environment variable from __environ
    FailedToFetchEnv(VarError),
}

pub fn handler(name: &str) -> Result<*const c_char> {
    if name == "TZ" {
        println!("[+] Intercepted a getenv call for {}", name);
        let mut buf = vec![0u8; 512];
        let _ = io::stdin()
            .lock()
            .read(&mut buf)
            .map_err(|_| Error::ReadError)
            .map(|len| {
                buf.truncate(len);
                buf.push(b'\0');
            });
        return Ok(buf.leak().as_ptr().cast());
    } else {
        println!("[*] Ignoring a getenv call for {}", name);
        unsafe {
            let mut next = libc_environ;
            while !next.is_null() && !{ *next }.is_null() {
                let entry = CStr::from_ptr(*next).to_str().map_err(Error::UTF8);
                let (key, value) = entry
                    .map(|x| x.split_once("="))
                    .map_or(("", ""), |kv| kv.map_or(("", ""), |kv| kv));
                if name == key {
                    return Ok(value.as_ptr().cast());
                } else {
                    next = next.offset(1);
                }
            }
            return Ok(vec![0u8; 1].leak().as_ptr().cast());
        }
    }
}

/// # Safety
///
/// Wrapper for our internal getenv handler
pub unsafe fn getenv_intern(name: *const c_char) -> Result<*const c_char> {
    if !name.is_null() {
        let res = CStr::from_ptr(name).to_str();
        return res.map_err(Error::UTF8).and_then(handler);
    }
    Err(Error::NullPtr)
}

/// # Safety
///
/// This function tries to intercept the getenv call to the environment variable "TZ"
/// while returning the set values for all others regularly
#[no_mangle]
pub unsafe extern "C" fn getenv(name: *const c_char) -> *const c_char {
    getenv_intern(name).map_or_else(
        |e| {
            println!("{:#?}", e);
            vec![0u8; 1].leak().as_ptr().cast()
        },
        |c| c,
    )
}

/// # Safety
///
/// This function just calls the nightmarish implementation above, be ready
#[no_mangle]
pub unsafe extern "C" fn secure_getenv(name: *const c_char) -> *const c_char {
    getenv(name)
}

extern "C" {
    #[link_name = "environ"]
    static libc_environ: *const *const std::os::raw::c_char;
}
