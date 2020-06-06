//this module contains the engine for running rhai scripts and
// interoperating with the server

use std::collections::HashMap;

use rhai::{
    Engine,
    EvalAltResult,
    Scope,
    INT,
    Array,
    RegisterFn,
    Dynamic,
};

use crate::sfuncs::*;


pub struct ScriptEngine<'a> {
    engine: Engine,
    scope: Scope<'a>,
}

//default configuration and functions
const DEFCONF: &'static str = "
let eWin_size = [500, 500];
";

impl ScriptEngine<'_> {
    pub fn new<'a>() -> ScriptEngine<'a> {
        println!("Initializing ScriptEngine...");
        let mut engine = Engine::new();
        let mut scope = Scope::new();

        println!("Initializing variables...");
        engine.eval_with_scope::<Dynamic>(&mut scope, DEFCONF).unwrap();

        println!("ScriptEngine init complete!");
        ScriptEngine {
            engine: engine,
            scope: scope,
        }
    }
    pub fn eval(&mut self, code: &str) -> Dynamic {
        self.engine.eval_with_scope::<Dynamic>(&mut self.scope, code).unwrap()
    }
}
