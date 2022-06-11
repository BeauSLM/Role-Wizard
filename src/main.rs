use serenity::utils::MessageBuilder;
use serenity::{async_trait, framework, model, prelude::*};
use framework::standard::macros::{command, group, hook};
use framework::standard::{
    Args,
    CommandResult,
    StandardFramework
};

use model::prelude::*;
use channel::Message;
use gateway::{GatewayIntents, Ready};

#[group]
#[commands(role)]
struct WhyDoINeedThis;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[hook]
async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    println!("Got command '{}' by user '{}'", command_name, msg.author.name);
    true
}

#[hook]
async fn after(_: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => println!("Processed command '{}'", command_name),
        Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[hook]
async fn unknown_command(_: &Context, _: &Message, command_name: &str) {
    print!("Could not find command named '{}'", command_name);
}

#[hook]
async fn normal_message(_: &Context, msg: &Message) {
    print!("Message is not a command '{}'", msg.content);
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    
    let framework = StandardFramework::new()
        .configure(|c| c
            .with_whitespace(true)
            .prefix("!") // there's a lot of configuration options here
            .delimiters([", ", ","]))
        // add hooks (these are optional, but nice)
        .before(before)
        .after(after)
        .unrecognised_command(unknown_command)
        .group(&WHYDOINEEDTHIS_GROUP);
    
    let intents = GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Client built successfuly");
    
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
} 

#[command]
async fn role(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let potential_role_name = args.rest();
    
    // NOTE: checks local cache only
    // there's def a cool nice way to do this without any panicking
    // TODO: guarantee that there's a guild in the cache, or handle the case when there isn't
    let guild = msg.guild(&ctx.cache).expect("There's a guild");
    let http = &ctx.http;
    
    let message = if let Some(Role {id, ..}) = guild.role_by_name(potential_role_name) {
            let user = &msg.author;
            guild.member(http, user)
                .await?
                .add_role(http, id)
                .await?;
            MessageBuilder::new()
                .push("Here you go, ")
                .user(user)
                .push('!')
                .build()
        } else {
            format!("Couldn't find role named '{potential_role_name}'")
        };
    
    msg.channel_id
        .say(http, message)
        .await?;
    
    Ok(())
}
