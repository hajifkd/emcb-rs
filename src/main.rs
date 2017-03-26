extern crate emval_sys;

use emval_sys::*;

fn main() {
    unsafe {
        static VOID_ID: &'static str = "rust_void\0";
        _embind_register_void(VOID_ID.as_ptr() as _,
                              VOID_ID.as_ptr() as _);

        static FUNC_ID: &'static str = "rust_func\0";

        static mut ARGS_LIST: [*const usize; 1] = [0 as _; 1];

        let cbfn = || {
            println!("Hello, callback in native Rust!");
        };

        let cbptr = Box::into_raw(Box::new(&cbfn as &Fn()));

        ARGS_LIST[0] = VOID_ID.as_ptr() as _;

        extern fn hoge(arg1: isize) {
            println!("Hello, involer!");
            let cb: Box<&Fn()> = unsafe { Box::from_raw(arg1 as _) };
            cb();
        }

        _embind_register_function(FUNC_ID.as_ptr() as _,
                                  1,
                                  //(VOID_ID.as_ptr()) as _,
                                  ARGS_LIST.as_ptr() as _,
                                  "vi\0".as_ptr() as _,
                                  hoge as _,
                                  cbptr as _);
    }
}
