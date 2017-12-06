#[macro_use] extern crate serenity;

use std::env;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use serenity::prelude::*;
use serenity::model::*;
use serenity::framework::standard::StandardFramework;

struct Handler;

impl EventHandler for Handler {

    fn on_message(&self, _: Context, msg: Message) {
        println!("Message received:
        {author:>width$}
        {message:>width$}\n"
        , author=msg.author, message=msg.content, width=4
        );
    }
    
    ///The bot has successfully connected.
    fn on_ready(&self, _: Context, ready: Ready) {
        let user = &ready.user;
        println!("{} is connected!", user.name);
    }
    
    //Created or was added to a guild
    fn on_guild_create(&self, _: Context, guild: Guild, _is_new: bool) {
        println!("Guild added: {} - {}", guild.name, guild.id);
        confirm_server(guild.id);
    }
    
    // guild was deleted
    fn on_guild_delete(&self, _: Context, guild: PartialGuild, _: Option<Arc<RwLock<Guild>>>) { 
        println!("Guild deleted: {} - {}", guild.name, guild.id);
        let path = server_path(guild.id);
        if let Err(why) = fs::remove_dir(&path) {
            println!("Failed to delete path '{}': {}", path, why)
        }
    }
}

fn confirm_dir(path: &str) {
    if !Path::new(path).exists() {
        println!("Directory '{}' didn't exist, creating.", path);
        if let Err(why) = fs::create_dir(path) {
            panic!("Fatal error: {}", why);
        }
    }
}

fn server_path(path: GuildId) -> String {
    return format!("./servers/{}", path);
}

fn confirm_server(path: GuildId) {
    confirm_dir(&server_path(path));
}

fn init() {
    confirm_dir("./servers");
}

fn main() {
    init();
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler);    
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .on("ping", ping)
        .on("test", test));
    println!("Starting...");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});

command!(test(_context, message) {
    let _ = message.reply("F");
});