use std::os::raw::{c_int};
type OnDataCallback = unsafe extern "C" fn(data: *mut u8, len: usize) -> c_int;

static mut onDataCallback_: Option<OnDataCallback> = None;

#[no_mangle]
pub extern "C" fn registerOnDataCallback(cb: Option<OnDataCallback>) -> c_int
{
    unsafe{onDataCallback_ = cb;}
    return 0;
}

#[no_mangle]
pub extern "C" fn doSomething()
{
    
    let mut s = String::from("hello world\0");
    unsafe {
        unsafe{onDataCallback_.unwrap()(s.as_mut_ptr() , 100)};
    }
}