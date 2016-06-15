extern crate libc;

mod objclass;

use std::ffi::{CStr, CString};

//These need to be defined so Ceph keep track of our version info
#[no_mangle]
pub static __cls_ver_maj: ::std::os::raw::c_int = 1;
#[no_mangle]
pub static __cls_ver_min: ::std::os::raw::c_int = 0;
#[no_mangle]
pub static __cls_ver__: ::std::os::raw::c_int = 1_0;

/*
#[repr(C)]
pub struct Safe<T>{ x: T }
unsafe impl<T> Send for Safe<T> {}
unsafe impl<T> Sync for Safe<T> {}
#[no_mangle] pub static __cls_name: Safe<*const u8> = Safe {x: b"rust_hello\0" as *const u8 };
*/

///Empty struct to simulate a class and make Ceph happy
struct Hello{}

unsafe extern fn say_hello(ctx: objclass::cls_method_context_t,
                                                 indata: *mut ::std::os::raw::c_char,
                                                 datalen: ::std::os::raw::c_int,
                                                 outdata: *mut *mut ::std::os::raw::c_char,
                                                 outdatalen: *mut ::std::os::raw::c_int)-> ::std::os::raw::c_int{
    0
}

#[no_mangle]
pub extern "C" fn __cls_init() {
    let mut h = Hello{};
    let mut cls_ptr: *mut ::std::os::raw::c_void = &mut h as *mut _ as *mut ::std::os::raw::c_void;
    let mut say_hello_ptr: *mut ::std::os::raw::c_void = &mut say_hello as *mut _ as *mut ::std::os::raw::c_void;

    let mut class_call_ptr = Some(
        say_hello
    );

    unsafe{
        objclass::cls_log(0, CString::new("Hello from Rust").unwrap().as_ptr());
    }

    unsafe{
        objclass::cls_register(CString::new("rust_hello").unwrap().as_ptr(), &mut cls_ptr);
    }

    objclass::cls_register_method(cls_ptr,
                                  CString::new("say_hello").unwrap().as_ptr(),
                                  objclass::CLS_METHOD_RD,
                                  class_call_ptr,
                                  &mut say_hello_ptr);
}
