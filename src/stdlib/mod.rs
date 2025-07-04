use crate::interpreter::Value;

pub mod io;
pub mod math;
pub mod string;
pub mod fs;
mod json;

// func(value_1) -> value, string as result
pub type NativeFn = fn(&[Value]) -> Result<Value, String>;

pub struct Module {
    pub name: &'static str,
    pub funcs: &'static [(&'static str, NativeFn)],
}

// standard registry of modules
pub const REGISTRY_STD: &[Module] = &[
    io::IO_MOD,
    math::MATH_MOD,
    string::STRING_MOD,
];

pub const REGISTRY_OPTIONAL: &[Module] = &[
    fs::FS_MOD,
    json::JSON_MOD,
];