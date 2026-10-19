use trpl::StreamExt;

pub fn example_1() {
    trpl::block_on( async {
        let values = [1,2,3];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}")
        }
    })
}