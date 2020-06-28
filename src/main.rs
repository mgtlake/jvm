#[macro_use]
extern crate bitflags;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::env;
use std::fs::File;

use class::*;

use crate::execution::*;

mod attributes;
mod class;
mod constants;
mod execution;
mod fields;
mod instructions;
mod methods;
mod read;

// TODO encode jvm primitives as types
// TODO work out how references should work - conflict with rust type system?
// TODO work out memory allocaiton - pc register, stack, heap, method area, constant pool, native method stacks
// TODO frames

// Non-goals:
//      - Optimisation
//      - Completeness (i.e. will not cover entire spec)
//      - Verification (i.e. will accept functional programs forbidden by spec)
fn main() {
    let args: Vec<_> = env::args().collect();
    let path = args.get(1).unwrap();
    println!("Path {}", path);
    let reader = &mut File::open(path).unwrap();
    let class = parse_class(reader).unwrap();
    if class.has_method("<clinit>".to_string()) {
        load_frame("<clinit>", &class, vec![]).exec();
    }
    if !class.has_method("main".to_string()) {
        println!("No main method");
    }
    let result = load_frame("main", &class, vec![]).exec();
    println!("Result: {:?}", result);
}
