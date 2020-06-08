mod attributes;
mod class;
mod constants;
mod execution;
mod fields;
mod instructions;
mod methods;
mod read;

use crate::execution::*;
use class::*;

#[macro_use]
extern crate bitflags;

// TODO encode jvm primitives as types
// TODO work out how references should work - conflict with rust type system?
// TODO work out memory allocaiton - pc register, stack, heap, method area, constant pool, native method stacks
// TODO frames

// Non-goals:
//      - Optimisation
//      - Completeness (i.e. will not cover entire spec)
//      - Verification (i.e. will accept functional programs forbidden by spec)
fn main() {
    let class = parse_class().unwrap();
    let mut add_frame = load_frame(
        "add".to_string(),
        &class,
        vec![DataType::Integer(1), DataType::Integer(1)],
    );
    add_frame.exec();
}
