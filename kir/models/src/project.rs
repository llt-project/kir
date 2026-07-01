use std::collections::HashMap;
use crate::decl::Module;

#[repr(C)]
pub struct HProject {
    pub ptr: *mut Project,
}

pub struct Project {
    pub mods: Vec<Module>,
    pub unobfuscated_names: HashMap<String, bool>, // мапа для хранения имен, которые не должны быть обфусцированы
}