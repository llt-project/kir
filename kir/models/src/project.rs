use std::collections::HashMap;

#[repr(C)]
pub struct HProject {
    pub ptr: *mut Project,
}

pub struct Project {
    pub unobfuscated_names: HashMap<String, bool>,
}