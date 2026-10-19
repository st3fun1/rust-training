use std::time::Duration;

use trpl::Either;

async fn custom_timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
     match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
     }

}

pub fn run_custom_timeout_example() {
    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(2)).await;
            "finally finished"
        };

        match custom_timeout(slow, Duration::from_secs(2)).await {
                Ok(message) => println!("Succeded with '{message}'"),
                Err(duration) => {
                    println!("Failed after {} seconds", duration.as_secs())
                }
            }
    });
}