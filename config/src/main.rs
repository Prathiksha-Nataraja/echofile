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
    depend_on: HashMap<String,String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Workflow {
    name : String,
    version : String,
}


#[derive(Debug, ProvidesStaticType, Default)]
struct Tasks{
    tasks : RefCell<Vec<Task>>,
    workflows: RefCell<Vec<Workflow>>
}

// #[derive(Debug, ProvidesStaticType)]
// struct Workflows(RefCell<Workflow>);

impl Tasks {
    fn add(&self, kind : String, name : String, input_args : HashMap<String, String>, properties : HashMap<String, String>, depend_on : HashMap<String, String>) {
        self.tasks.borrow_mut().push(Task {
            kind,
            name,
            input_args,
            properties,
            depend_on
        })
    }
    fn adds(&self, name : String, version : String ) {
        // self.0.push(Workflow { name, version})
        self.workflows.borrow_mut().push(Workflow {name, version })
    }
}

#[starlark_module]
fn starlark_workflow(builder: &mut GlobalsBuilder) {
    fn task(
        kind: String,
        name: String,
        input_args:Value,
        properties: Value,
        depend_on: Value,
        eval: &mut Evaluator,
    ) -> anyhow::Result<NoneType> {

        let ip_args = serde_json::from_str(&input_args.to_json()?).unwrap();
        let property = serde_json::from_str(&properties.to_json()?).unwrap();
        let depnd = serde_json::from_str(&depend_on.to_json()?).unwrap();
        
        eval.extra.unwrap().downcast_ref::<Tasks>().unwrap().add(
            kind,
            name,
            ip_args,
            property,
            depnd
        );
       
        Ok(NoneType)
    }

    fn workflows(
        name: String,
        version : String,
        eval : &mut Evaluator
    ) -> anyhow::Result<NoneType> {
        eval.extra.unwrap().downcast_ref::<Tasks>().unwrap().adds(
            name,
            version,
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
    let store = Tasks::default(); 
    {
        let mut eval = Evaluator::new(&module);
        // We add a reference to our store
        eval.extra = Some(&store);
        
        eval.eval_module(ast, &globals).unwrap();
    }

    println!("{:#?}", store);
}
