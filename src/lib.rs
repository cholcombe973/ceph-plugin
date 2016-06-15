mod objclass;
use std::ffi::CString;

static CLS_METHOD_RD: ::std::os::raw::c_int       =   0x1;
static CLS_METHOD_WR: ::std::os::raw::c_int       =   0x2;
static CLS_METHOD_PUBLIC: ::std::os::raw::c_int   =   0x4;

struct Hello{

}
impl Hello{
    pub extern fn say_hello(){

    }
    pub extern fn record_hello(){

    }
}

#[no_mangle]
pub extern "C" fn __cls_init() {
    let mut h = Hello{};

    // this log message, at level 0, will always appear in the ceph-osd
    // log file.
    objclass::cls_log(0, CString::new("Hello from Rust").unwrap().as_ptr());

    objclass::cls_register(CString::new("rust_hello").unwrap().as_ptr(), &mut h);

    // There are two flags we specify for methods:
    //
    //    RD : whether this method (may) read prior object state
    //    WR : whether this method (may) write or update the object
    //
    // A method can be RD, WR, neither, or both.  If a method does
    // neither, the data it returns to the caller is a function of the
    // request and not the object contents.

    objclass::cls_register_method(&mut h,
                                  CString::new("say_hello").unwrap().as_ptr(),
                                  CLS_METHOD_RD,
                                  h.say_hello(),
                                  &h.say_hello());
    /*objclass::cls_register_method(h_class,
                                  CString::new("record_hello").unwrap().as_ptr(),
                                  CLS_METHOD_WR | CLS_METHOD_PROMOTE,
                                  record_hello,
                                  &h_record_hello);*/
}
