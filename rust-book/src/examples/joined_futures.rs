use std::{thread, time::Duration};

use trpl;

pub fn joined_futures_example() {
    trpl::block_on(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut3= async {
            for i in 1..5 {
                println!("hi number {i} from the third task!");
                trpl::sleep(Duration::from_millis(0)).await;
            }
        };

        trpl::join(fut1, fut3).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name} ran from {ms}ms");
}


pub fn run_fn_with_slow_calls() {
    trpl::block_on(async {
        let a = async {
            println!("'a' started");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.")
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            print!("'b' finished.");
        };

        trpl::select(a,b).await;
    });
}

pub fn run_fn_with_slow_calls_true_await() {
    let one_ms = Duration::from_millis(1);

    trpl::block_on(async {
         let a = async {
        println!("'a' started");
        slow("a", 30);
        trpl::yield_now().await;
        slow("a", 10);
        trpl::yield_now().await;

        slow("a", 20);
        trpl::yield_now().await;
        println!("'a' finished.");

    };

       let b = async {
        println!("'b' started");
        slow("b", 75);
        trpl::yield_now().await;
        slow("b", 10);
        trpl::yield_now().await;

        slow("b", 15);
        trpl::yield_now().await;
                 slow("b", 350);
        trpl::yield_now().await;
        println!("'b' finished.");

    };
      trpl::select(a, b).await;
    });
}