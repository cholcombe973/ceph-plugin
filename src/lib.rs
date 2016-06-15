extern crate libc;

mod objclass;

use std::ffi::{CStr, CString};

static CLS_METHOD_RD: ::std::os::raw::c_int       =   0x1;
static CLS_METHOD_WR: ::std::os::raw::c_int       =   0x2;
static CLS_METHOD_PUBLIC: ::std::os::raw::c_int   =   0x4;

#[no_mangle]
pub static __cls_ver_maj: ::std::os::raw::c_int = 1;
#[no_mangle]
pub static __cls_ver_min: ::std::os::raw::c_int = 0;
#[no_mangle]
pub static __cls_ver__: ::std::os::raw::c_int = 1_0;

/*
#[no_mangle]
pub static __cls_name: *const char = "rust_hello";
*/
#[repr(C)]
pub struct Safe<T>{ x: T }
unsafe impl<T> Send for Safe<T> {}
unsafe impl<T> Sync for Safe<T> {}
#[no_mangle] pub static FOO: Safe<*const u8> = Safe {x: b"rust_hellow\0" as *const u8 };

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

    // this log message, at level 0, will always appear in the ceph-osd
    // log file.
    unsafe{
        objclass::cls_log(0, CString::new("Hello from Rust").unwrap().as_ptr());
    }

    unsafe{
        objclass::cls_register(CString::new("rust_hello").unwrap().as_ptr(), &mut cls_ptr);
    }

    // There are two flags we specify for methods:
    //
    //    RD : whether this method (may) read prior object state
    //    WR : whether this method (may) write or update the object
    //
    // A method can be RD, WR, neither, or both.  If a method does
    // neither, the data it returns to the caller is a function of the
    // request and not the object contents.

/*
    objclass::cls_register_method(cls_ptr,
                                  CString::new("say_hello").unwrap().as_ptr(),
                                  CLS_METHOD_RD,
                                  class_call_ptr,
                                  &mut say_hello_ptr);
*/
}
