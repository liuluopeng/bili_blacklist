use core::time;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::thread::sleep;
// use url::Url;
use csv::Writer;
use dotenv::dotenv;
use std::env;
use url::Url;

#[tokio::main]
async fn main() {
    // 开始拉黑:

    dotenv().ok();
    println!("Hello, world!");
    let client = Client::new();

    let mut csv_contents = String::new();
    let file = File::open("result.csv")
        .unwrap()
        .read_to_string(&mut csv_contents);

    let mut reader = csv::Reader::from_reader(csv_contents.as_bytes());
    for record in reader.records() {
        let record = record.unwrap();
        println!("uid: {}  用户名: {}", &record[0], &record[1]);

        let uid = &record[0];
        let uname = &record[1];

        let url = "https://api.bilibili.com/x/relation/modify";
        let mut params = vec![];

        params.push(("fid", uid));
        params.push(("act", "5")); // block the user
        params.push(("re_src", "11")); // from bilibili space
        let csrf2 = env::var("CSRF2").expect("not");
        params.push(("csrf", csrf2.as_str()));


        let response = client
            .post(url)
            .header(
                "Cookie",
                format!("SESSDATA={}", env::var("SESSDATA2").expect("not found")),
            )
            .form(&params)
            .send()
            .await
            .unwrap();

        let body = response.text().await.unwrap();
        println!("{:?}", body);

        // let data: Value = serde_json::from_str(&body).unwrap();
        // println!("{:?}", data);

        sleep(time::Duration::from_secs(2));
    }
}

//
