mod read;
mod constants;
mod fields;
mod class;

use class::*;

// Non-goals:
//      - Optimisation
//      - Completeness (i.e. will not cover entire spec)
//      - Verification (i.e. will accept functional programs forbidden by spec)
fn main() {
    parse_class();
}