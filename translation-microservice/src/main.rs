use google_translator::translate_one_line;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::message::Message;
use std::env;
use dotenv;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // println!("Hello, world!");
    // let text = vec!["Hello, world!", "내 이름은 민수야.", "나는 20살이야."]
    //     .iter()
    //     .map(|x| x.to_string())
    //     .collect();
    // let input_lang = "auto";
    // let output_lang = "en";
    // let result = translate(text, input_lang, output_lang).await.unwrap();

    // for x in result.output_text {
    //     println!("{:?}", x)
    // }

    let kafka_host = env::var("KAFKA_HOST").expect("KAFKA_HOST environment variable not set");
    println!("Connecting to Kafka at {}", kafka_host);
    let consumer: StreamConsumer<rdkafka::consumer::DefaultConsumerContext> = ClientConfig::new()
        .set("bootstrap.servers", kafka_host)
        .set("group.id", "my-consumer-group1")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Failed to create consumer");

    let sub = consumer.subscribe(&["translate-fr-ja"]);
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
                    let translated = translate_one_line(
                        payload.to_string(),
                        "auto", 
                        "fr",
                    );
                    println!("received message: {} {:?}", payload, translated.await);
                }
                consumer.commit_message(&msg, rdkafka::consumer::CommitMode::Async)
                    .expect("Failed to commit offset");
            }
        }
    }
    println!("execting because above problem");


}

