#![feature(test)]

extern crate regex;
extern crate test;

use regex::Captures;
use regex::Regex;

use std::ffi::{CString, CStr};
use std::iter::Iterator;
use std::mem;
use std::os::raw::{c_char, c_void};

//
// replace_all을 나이스하게 하는 방법을 아직 찾아보지 않았다.
//
fn escape(reg: &Regex, search_value: &str) -> String {
    reg.replace_all(search_value, |caps: &Captures| {
        caps.iter().fold(
            String::new(),
            |mut acc, m| {
                if m.is_some() {
                    acc.push_str(["\\", m.unwrap().as_str()].concat().as_str());
                }
                acc
            }
        )
    }).into_owned()
}

fn __to_string(v: *const c_char) -> String {
    let s = unsafe {
        CStr::from_ptr(v).to_str().unwrap()
    };
    s.to_string()
}

fn __to_ptr(s: String) -> *const c_char {
    let s = CString::new(s).unwrap();
    s.into_raw()
}

//
// Regex 인스턴스를 힙에 올리고 포인터를 반환
//
#[no_mangle]
pub fn create_regexp(_regstr: *const c_char) -> *const c_void {
    let regstr = __to_string(_regstr);
    let _reg = Regex::new(regstr.as_str()).unwrap();
    let reg: Box<Regex>= Box::new(_reg);
    let ptr = &*reg as *const Regex as *const c_void;
    mem::forget(reg);
    ptr
}

//
// 생성된 Regex 인스턴스 사용
//
#[no_mangle]
pub fn escape_as_reg(_reg: *const c_void, _search_value: *const c_char) -> *const c_char {
    let reg: &Regex = unsafe { mem::transmute(_reg) };
    let search_value = __to_string(_search_value);
    let result = escape(reg, search_value.as_str());
    __to_ptr(result)
}

//
// 정규식 문자열로..
//
#[no_mangle]
pub fn escape_as_regstr(_regstr: *const c_char, _search_value: *const c_char) -> *const c_char {
    let regstr = __to_string(_regstr);
    let search_value = __to_string(_search_value);
    let reg = Regex::new(regstr.as_str()).unwrap();
    let result = escape(&reg, search_value.as_str());
    __to_ptr(result)
}

#[no_mangle]
pub fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    #[test]
    fn escape() {
        let reg = super::Regex::new(r"[\-\[\]{}()*+?.,\\\^$|#\s]").unwrap();

        assert_eq!(super::escape(&reg, "a-b"), "a\\-b");
        assert_eq!(super::escape(&reg, "a[b"), "a\\[b");
        assert_eq!(super::escape(&reg, "a]b"), "a\\]b");
        assert_eq!(super::escape(&reg, "a{b"), "a\\{b");
        assert_eq!(super::escape(&reg, "a}b"), "a\\}b");
        assert_eq!(super::escape(&reg, "a(b"), "a\\(b");
        assert_eq!(super::escape(&reg, "a)b"), "a\\)b");
        assert_eq!(super::escape(&reg, "a*b"), "a\\*b");
        assert_eq!(super::escape(&reg, "a+b"), "a\\+b");
        assert_eq!(super::escape(&reg, "a?b"), "a\\?b");
        assert_eq!(super::escape(&reg, "a.b"), "a\\.b");
        assert_eq!(super::escape(&reg, "a,b"), "a\\,b");
        assert_eq!(super::escape(&reg, "a\\b"), "a\\\\b");
        assert_eq!(super::escape(&reg, "a^b"), "a\\^b");
        assert_eq!(super::escape(&reg, "a$b"), "a\\$b");
        assert_eq!(super::escape(&reg, "a|b"), "a\\|b");
        assert_eq!(super::escape(&reg, "a#b"), "a\\#b");
        assert_eq!(super::escape(&reg, "a\\sb"), "a\\\\sb");
        assert_eq!(super::escape(&reg, "a-[]{}()*+?.,\\^$|#\\s한b"), "a\\-\\[\\]\\{\\}\\(\\)\\*\\+\\?\\.\\,\\\\\\^\\$\\|\\#\\\\s한b");
    }

    //
    // 약 0.5~0.6초 정도 나온다. 
    // Rust 구현 자체도 그닥 빠르지 않다.
    //
    #[bench]
    fn bench_escape_10_iter(b: &mut Bencher) {
        b.iter(|| (0..10).for_each(|_| escape()));
    }
}
