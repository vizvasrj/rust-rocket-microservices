// use kafka::consumer::{Consumer, FetchOffset};
use std::env;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::message::Message;

#[tokio::main]
async fn main() {
    let kafka_host = env::var("KAFKA_HOST").expect("KAFKA_HOST environment variable not set");
    println!("Connecting to Kafka at {}", kafka_host);
    let consumer: StreamConsumer<rdkafka::consumer::DefaultConsumerContext> = ClientConfig::new()
        .set("bootstrap.servers", kafka_host)
        .set("group.id", "my-consumer-group1")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Failed to create consumer");

    let sub = consumer.subscribe(&["quickstart"]);
    if sub.is_err() {
        eprintln!("cannot subscr err = {}", sub.err().unwrap().to_string());
        return;
    }


    loop {
        match consumer.recv().await {
            Err(err) => {
                eprintln!("Error while receiving message: {:?}", err);
                break
            },
            Ok(msg) => {
                if let Some(Ok(payload)) = msg.payload_view::<str>() {
                    println!("received message: {}", payload);
                }
                consumer.commit_message(&msg, rdkafka::consumer::CommitMode::Async)
                    .expect("Failed to commit offset");
            }
        }
    }
    println!("execting because above problem");
    // std::thread::sleep(std::time::Duration::from_secs(60));

    // let hosts = vec![kafka_host];
    // let consumer_res = Consumer::from_hosts(hosts)
    //     .with_topic("quickstart".to_owned())
    //     .with_fallback_offset(FetchOffset::Latest)
    //     .create();

    // if consumer_res.is_err() {
    //     println!("error while listing:: {}", consumer_res.err().unwrap().to_string());
    //     // std::thread::sleep(std::time::Duration::from_secs(600));

    //     return
    // }    
    
    // let mut consumer = consumer_res.unwrap();

    // println!("Hello, world!");

    // loop {
    //     for ms in consumer.poll().unwrap().iter() {
    //         for m in ms.messages() {
    //             // if the consumer receives an event, this block is executed
    //             println!("{:?}", String::from_utf8_lossy(m.value));
    //         }

    //         consumer.consume_messageset(ms).unwrap();
    //     }

    //     consumer.commit_consumed().unwrap();
    // }


}
