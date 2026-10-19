use crate::examples::closures;
use crate::examples::concurrency;
use crate::examples::joined_futures;
use crate::examples::task_message_passing;
use crate::examples::custom_timeout;
use crate::examples::future_streams;
use crate::examples::unsafe_rust;

pub mod examples;

fn main() {
   unsafe_rust::run_unsafe_addition_to_static_var();
}
