use std::time::Duration;

use trpl;

pub fn message_passing_example() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("received '{received}'");
    })
}

pub fn message_passing_concurrency_example() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
                        String::from("from"),

                                    String::from("hi"),

                                                String::from("the"),

                                                            String::from("future"),

        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    });
}

pub fn real_concurrency_of_message_passing() {
  trpl::block_on(async {
          let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();

               let tx1_fut = async move {
            let vals = vec![
            String::from("hi"),
                        String::from("from"),

                                    String::from("hi"),

                                                String::from("the"),

                                                            String::from("future"),

            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
            String::from("hi"),
                        String::from("from"),

                                    String::from("hi"),

                                                String::from("the"),

                                                            String::from("future"),

            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join!(tx_fut, tx1_fut, rx_fut);
  });


}
