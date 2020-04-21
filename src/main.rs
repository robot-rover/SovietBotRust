extern crate serenity;
extern crate dotenv;
#[macro_use]
extern crate diesel;

use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context, TypeMapKey};
use serenity::framework::standard::CommandResult;
use serenity::utils::MessageBuilder;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use std::sync::Mutex;
use schema::guilds;
use models::Guild;
use std::ops::Deref;
use std::borrow::Borrow;

pub mod schema;
pub mod models;
pub mod command;

struct Handler;

const TRIGGER_DEFAULT: &str = ">";

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot { return }

        let guild_id = if let Some(guild_id) = msg.guild_id {
            guild_id
        } else {
            return
        };

        let result = {
            let data = ctx.data.read();
            let mtx = data.get::<DatabaseConnection>().unwrap();
            let conn = mtx.lock().unwrap();
            let guild: Option<Guild> = match guilds::table
                .filter(guilds::id.eq(into_db(guild_id.0)))
                .first::<Guild>(conn.deref()).optional() {
                Ok(guild) => guild,
                Err(other) => {
                    eprintln!("{}", other);
                    return;
                }
            };
            guild.map(|guild| guild.trigger).flatten()
        };
        let trigger = result.as_ref().map(|trigger| trigger.as_str()).unwrap_or(TRIGGER_DEFAULT);

        if msg.content.starts_with(trigger) {
            if let Some(command_name) = msg.content.split_ascii_whitespace().next().map(|word| &word[trigger.len()..]) {
                let command = command::COMMAND_LIST
                    .iter()
                    .map(|&i| i)
                    .find(|&command| command.name().eq(command_name));
                if let Some(command) = command {
                    if let Err(error) = command.execute(ctx, msg) {
                        eprintln!("Logged an error: {}", error);
                    }
                }
            }

        }
    }
}

fn main() {
    dotenv::dotenv().ok();
    let token = env::var("TOKEN")
        .expect("TOKEN must be set");
    let connection = connect_database();

    let mut client = Client::new(token, Handler)
        .expect("Error creating client");
    {
        let mut data = client.data.write();
        data.insert::<DatabaseConnection>(Mutex::new(connection));
    }
    if let Err(why) = client.start() {
        println!("Error: {:?}", why);
    }
}

fn into_db(id: u64) -> i64 {
    unsafe {
        std::mem::transmute(id)
    }
}

fn from_db(db: i64) -> u64 {
    unsafe {
        std::mem::transmute(db)
    }
}

struct DatabaseConnection;

impl TypeMapKey for DatabaseConnection {
    type Value = Mutex<SqliteConnection>;
}

fn connect_database() -> SqliteConnection  {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
