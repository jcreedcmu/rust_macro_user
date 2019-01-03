#![feature(proc_macro_hygiene)]
extern crate macro_experiment;

use macro_experiment::declare;

fn main() {
    declare!(var);
    println!("{}", var_with_suffix);
}
