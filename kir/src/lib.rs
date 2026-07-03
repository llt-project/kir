use std::collections::HashMap;
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;

use models::project::{
    HProject,
    Project,
};

// тест функции
#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> *mut c_char {
    let result = a + b;
    let s = CString::new(result.to_string()).unwrap();
    s.into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn print(s: *const c_char) {
    if s.is_null() { return }

    unsafe {
        let s = std::ffi::CStr::from_ptr(s).to_str().unwrap_or("");
        println!("{s}");
    }
}

// интерфейсы
#[unsafe(no_mangle)]
pub extern "C" fn project_new(s: *const *const c_char, len: usize) -> HProject {
    let mut map = HashMap::new();

    unsafe {
        for &item in std::slice::from_raw_parts(s, len) { // создаем срез из указателя и длины для перебора элементов
            if item.is_null() { continue }
            map.insert(CStr::from_ptr(item).to_string_lossy().to_string(), true);
        }
    }

    let project: Box<Project> = Box::new(Project {
        unobfuscated_names: map,
    });

    HProject {
        ptr: Box::into_raw(project),
    }
}