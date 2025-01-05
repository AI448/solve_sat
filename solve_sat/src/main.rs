#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(impl_trait_in_assoc_type)]

mod analyze;
mod binary_clause_theory;
mod calculate_lbd;
mod clause_theory;
mod cnf;
mod engine;
mod plbd_watcher;
mod pricer;
mod read_cnf;
mod simplify;
mod solve;
use read_cnf::read_cnf;
use solve::solve;

fn main() {
    let cnf = read_cnf(std::io::BufReader::new(std::io::stdin()));
    solve(&cnf);
}
