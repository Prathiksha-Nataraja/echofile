use serde_derive::Deserialize;
use serde_json::Value as serdeValue;
use starlark::environment::{Module, Globals, FrozenModule};
use starlark::{starlark_module, starlark_simple_value};
use starlark::values::{ Value, StarlarkValue, Heap};
use starlark::syntax::{AstModule, Dialect, DialectTypes};
use starlark::eval::{Evaluator, ReturnFileLoader};
use starlark::values::none::NoneType;
use std::collections::HashMap;
use std::fmt::{Display, self};
use std::fs;
use std::path::Path;
use starlark::environment::GlobalsBuilder;
use starlark::any::ProvidesStaticType;
use std::cell::RefCell;



#[derive(Debug, Clone, Deserialize)]
pub struct Task {
    kind : String,
    name: String,
    input_args: HashMap<String,String>,
    properties : HashMap<String, String>,
    depend_on: String,
}

#[derive(Debug, ProvidesStaticType, Default)]
struct WorkFlow(RefCell<Vec<Task>>);

impl WorkFlow {
    fn add(&self, kind : String, name : String, input_args : HashMap<String, String>, properties : HashMap<String, String>, depend_on : String) {
        self.0.borrow_mut().push(Task {
            kind,
            name,
            input_args,
            properties,
            depend_on
        })
    }
}

#[starlark_module]
fn starlark_workflow(builder: &mut GlobalsBuilder) {
    fn task(
        kind: String,
        name: String,
        input_args:Value,
        properties: Value,
        depend_on: String,
        eval: &mut Evaluator,
    ) -> anyhow::Result<NoneType> {

        let ip_args = serde_json::from_str(&input_args.to_json()?).unwrap();
        let property = serde_json::from_str(&properties.to_json()?).unwrap();
        // let depnd = serde_json::from_str(&depend_on.to_owned());
        
        eval.extra.unwrap().downcast_ref::<WorkFlow>().unwrap().add(
            kind,
            name,
            ip_args,
            property,
            depend_on
        );
       
        Ok(NoneType)
    }
}
fn main() {
    let content: String = std::fs::read_to_string("src/example.star").unwrap();

    let ast = AstModule::parse(
        "src/example.star",
        content.to_owned(),
        &Dialect::Standard,
    )
    .unwrap();
    // We build our globals adding some functions we wrote
    let globals = GlobalsBuilder::new().with(starlark_workflow).build();
    let module = Module::new();
    let store = WorkFlow::default();
    {
        let mut eval = Evaluator::new(&module);
        // We add a reference to our store
        eval.extra = Some(&store);
        eval.eval_module(ast, &globals).unwrap();
    }

    println!("{:#?}", store.0);
}
