use std::time::Duration;
use trpl;

pub fn concurrency_example_1() {
    trpl::block_on(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });
        for i in 1..5 {
            println!("hi number {i} from second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
