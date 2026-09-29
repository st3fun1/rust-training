use crate::examples::closures;
use crate::examples::concurrency;

pub mod examples;

fn main() {
   concurrency::concurrency_example_1();
   closures::closures_example();
}
