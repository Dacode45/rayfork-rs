#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod loading {
    use super::*;
    use libc::{c_char, c_int, c_void};
    use std::ffi::CStr;

    // Rayfork requires opengl headers be included at compile time rather than requiring a loadProc function for gltypes
    // To get around this we use glad rather than gl_generator and the gl crate and force OpenGL ES
    // The stuff below looks complicated, but its just boilerplate to get a callback into C. We use a trampoline in loadGL to eventually call the
    // gladLoadGLLoader

    static mut rfrsLoadProc: Option<
        unsafe extern "C" fn(*const c_char, *mut c_void) -> *mut c_void,
    > = None;
    static mut rfrsLoadProcUserData: Option<*mut c_void> = None;

    pub unsafe extern "C" fn loadGLWrapper(name: *const c_char) -> *mut c_void {
        let cb = rfrsLoadProc.as_ref().unwrap();
        let user_data = rfrsLoadProcUserData.clone().unwrap();
        cb(name, user_data)
    }

    pub unsafe extern "C" fn loadGLWithGlad(
        load_proc: unsafe extern "C" fn(*const c_char, *mut c_void) -> *mut c_void,
        user_data: *mut c_void,
    ) -> c_int {
        rfrsLoadProc = Some(load_proc);
        rfrsLoadProcUserData = Some(user_data);
        gladLoadGLLoader(Some(loadGLWrapper))
    }

    pub fn loadGL<F: Fn(&str) -> *mut c_void>(f: F) -> c_int {
        unsafe extern "C" fn wrapper<F: Fn(&str) -> *mut c_void>(
            name: *const c_char,
            ctx: *mut c_void,
        ) -> *mut c_void {
            let cstr: &CStr = unsafe { CStr::from_ptr(name) };
            let str_slice: &str = cstr.to_str().unwrap();
            (*(ctx as *const F))(str_slice)
        }
        unsafe { loadGLWithGlad(wrapper::<F>, &f as *const F as *mut c_void) }
    }
}
