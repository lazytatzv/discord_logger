use std::env;
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[derive(Debug)]
struct Data {
    token: String,
    channel_id: String,
}

static GLOBAL_DATA: OnceLock<Arc<Mutex<Data>>> = OnceLock::new();


fn get_data() -> Arc<Mutex<Data>> {
    GLOBAL_DATA.get().unwrap().clone()
}

fn init_data() {
    let data = Data {
       token: "xxx".to_string(),
       channel_id: "xxx".to_string(),
    };

    GLOBAL_DATA.set(Arc::new(Mutex::new(data))).unwrap();
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }


    }
}

// 初期化処理
async fn init() {
    let token = env::var("TOKEN")
        .expect("Expected a token in the environment");

    let channel_id = env::var("CHANNEL_ID")
        .expect("Expected a channel_id in the environment");

    let data = get_data();
    {
        let mut locked = data.lock().await;
        locked.token = token.clone();
        locked.channel_id = channel_id.clone();
    }


    // botの権限
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // botのインスタンスを作成し、botとしてログイン
    let mut client = 
        Client::builder(&token, intents)
        .event_handler(Handler)
        .await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

}

#[tokio::main]
async fn main() {
    init_data();
    init().await;

}
