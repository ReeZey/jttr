use dotenvy::dotenv;
use tokio::fs;
use std::{time::Duration, env, collections::HashMap};

use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;

static ENDPOINT: &str = "https://www.tiktok.com/api/user/list";

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    
    println!(r#"
    _            _      _   _            _   _ _    _        _           _                       
    (_)          | |    | | | |          | | (_) |  | |      | |         (_)                      
     _  __ _  ___| | __ | |_| |__   ___  | |_ _| | _| |_ ___ | | __  _ __ _ _ __  _ __   ___ _ __ 
    | |/ _` |/ __| |/ / | __| '_ \ / _ \ | __| | |/ / __/ _ \| |/ / | '__| | '_ \| '_ \ / _ \ '__|
    | | (_| | (__|   <  | |_| | | |  __/ | |_| |   <| || (_) |   <  | |  | | |_) | |_) |  __/ |   
    | |\__,_|\___|_|\_\  \__|_| |_|\___|  \__|_|_|\_\\__\___/|_|\_\ |_|  |_| .__/| .__/ \___|_|   
   _/ |                                                                    | |   | |              
  |__/                                                                     |_|   |_|              
    "#);
    println!("LET IT RIP");


    let env_vars: HashMap<String, String> = env::vars().collect();
    let tiktok_cookie = env_vars.get("COOKIE2").unwrap();

    let mut min_cursor = 0;
    let mut followers = vec![];
    loop {
        let params = [("count", 199), ("minCursor", min_cursor), ("scene", 67)];
    
        let mut header_map = HeaderMap::new();
        header_map.insert("Cookie", HeaderValue::from_str(&tiktok_cookie).unwrap());
    
        let client = reqwest::Client::new();
        let res = client.get(ENDPOINT.clone())
            .headers(header_map)
            .query(&params)
            .send()
            .await.unwrap();
    
        let json_response: Value = res.json().await.unwrap();
    
        let response_obj = json_response.as_object().unwrap();

        min_cursor = response_obj.get("minCursor").unwrap().as_i64().unwrap();

        let user_list = match response_obj.get("userList") {
            Some(user_list) => user_list.as_array().unwrap(),
            None => break,
        };

        followers.extend(user_list.clone());

        println!("found {} more users", user_list.len());

        //tiktok gillar verkligen inte att man spammar deras api
        tokio::time::sleep(Duration::from_millis(250)).await;
    }

    println!("users: {}", followers.len());
    
    let json_data: Value = followers.into();
    let pretty_json_data: String = serde_json::to_string_pretty(&json_data).unwrap();
    
    fs::write("users.json", pretty_json_data).await.unwrap();
}
