mod attributes;
mod class;
mod constants;
mod fields;
mod methods;
mod read;

use class::*;

#[macro_use]
extern crate bitflags;

// Non-goals:
//      - Optimisation
//      - Completeness (i.e. will not cover entire spec)
//      - Verification (i.e. will accept functional programs forbidden by spec)
fn main() {
    parse_class();
}
