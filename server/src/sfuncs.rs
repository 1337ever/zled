// this module contains utility functions for rhai scripts

use std::collections::HashMap;

use rhai::{
    Engine,
    RegisterFn,
    Dynamic,
};

//enum for the types of variables rhai scripts can modify
pub enum EngineTypes<'a> {
    String(&'a str),
    Int(u32),
    IntArr(Vec<u32>),
    StrArr(Vec<&'a str>),
}

pub type VarMap<'a> = HashMap<&'a str, EngineTypes<'a>>;
pub fn init_varmap<'a>() -> VarMap<'a> {
    let mut vm = VarMap::new();

    vm.insert("win_size", EngineTypes::IntArr(Vec::new()));

    vm
}
