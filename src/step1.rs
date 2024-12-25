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
    dotenv().ok();
    println!("Hello, world!");
    let client = Client::new();

    let url = "https://api.bilibili.com/x/relation/blacks";

    // ps  pn
    let mut now_pn = 1;
    let ps = 50;

    let response = client
        .get(url)
        .header(
            "Cookie",
            format!("SESSDATA={}", env::var("SESSDATA1").expect("not found")),
        )
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap();
    let data: Value = serde_json::from_str(&body).unwrap();

    let mut blacklist = vec![];
    // 获取黑名单总数
    let total_blacklist_number = data["data"]["total"].clone().as_i64().unwrap() as usize;
    println!("大号黑名单总数: {:?}", total_blacklist_number);

    // 然后从1到 total/50+1
    while now_pn <= total_blacklist_number / ps + 1 {
        let url = "https://api.bilibili.com/x/relation/blacks";
        let mut params = vec![];
        params.push(("ps", "50"));

        let now_pn_string = now_pn.to_string();
        params.push(("pn", &now_pn_string));

        let url = Url::parse_with_params(url, params).unwrap();
        let response = client
            .get(url)
            .header(
                "Cookie",
                format!("SESSDATA={}", env::var("SESSDATA1").expect("not found")),
            )
            .send()
            .await
            .unwrap();
        let body = response.text().await.unwrap();
        let data: Value = serde_json::from_str(&body).unwrap();

        let binding = data["data"]["list"].clone();
        let j = binding.as_array().unwrap();

        for (idx, black_user) in j.iter().enumerate() {
            println!(" {:?} {:?}", idx, black_user["mid"]);

            blacklist.push(vec![
                black_user["mid"].clone().to_string(),
                black_user["uname"].clone().to_string(),
            ]);
        }

        now_pn += 1;
        sleep(time::Duration::from_secs(2));
    }

    // 获取黑名单
    println!("{:?}", blacklist);

    // 保存成csv
    let mut csv_file = File::create("result.csv").unwrap();
    let mut wtr = Writer::from_writer(csv_file);

    // 写入短名行
    let headers = ["uid", "uname"];
    wtr.write_record(headers).unwrap();
    // 将数据写入 CSV 文件
    for row in blacklist {
        wtr.write_record(row).unwrap();
    }

    // 确保所有数据都被写入文件
    wtr.flush().unwrap();
}

//
