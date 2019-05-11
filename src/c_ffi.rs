use super::pinyin;
use super::Args;
use super::Style;

use libc::{c_char, size_t};
use std::ffi::{CStr, CString};
use std::vec::Vec;

ffi_fn! {
    fn c_pinyin(str_utf8: *const c_char, style: Style, heteronym: bool) -> *const Vec<Vec<String>>{
        let str =  unsafe { CStr::from_ptr(str_utf8).to_str() };
        let result = pinyin(str.unwrap(), &Args{
            style: style,
            heteronym: heteronym,
        });

        return Box::into_raw(Box::new(result));
    }
}

ffi_fn! {
    fn free_c_pinyin_result(result_ptr: *const Vec<Vec<String>>){
        unsafe { Box::from_raw(result_ptr as *mut Vec<Vec<String>>); }
    }
}

ffi_fn! {
    fn get_c_pinyin_result_count(result_ptr: *const Vec<Vec<String>>) -> usize{
        let result = unsafe { &*result_ptr };
        return result.len();
    }
}

ffi_fn! {
    fn get_c_pinyin_best_single_result_utf8(result_ptr: *const Vec<Vec<String>>, index :size_t) -> *const c_char{
        let result = unsafe { &*result_ptr };
        let candidates = result.get(index).unwrap();
        let best = candidates.get(0).unwrap();
        let cstring = CString::new(best.clone().into_bytes()).unwrap();
        let raw = cstring.into_raw();
        return raw;
    }
}

ffi_fn! {
    fn free_c_string(str_ptr: *mut c_char){
        unsafe { CString::from_raw(str_ptr) };
    }
}
