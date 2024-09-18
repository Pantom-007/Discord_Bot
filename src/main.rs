use twilight_command_parser::{CommandParserConfig, Command, Parser};
use twilight_gateway::{
    cluster::{Cluster, ShardScheme},
    Intents,
    Event};
use twilight_http::Client;
use futures::StreamExt;
use std::{
    env, 
    error::Error
};
use std::convert::TryFrom;

async fn handle_event(event: Event, id: u64, parser: &Parser<'_>, client: Client) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::ShardConnected(_) => {println!("Shard {} is now connected", id)},
        Event::ShardDisconnected(_) => {println!("Shard {} is now disconnected", id)},
        Event::MessageCreate(message) => {
            // Message is sent
            match parser.parse(&message.content) {
                Some(Command { name: "dab", .. }) => {
                    client.create_message(message.channel_id).content("dab")?.await?;
                }
            }
        },
        // Ignore other events
        _ => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    let args: Vec<String> = env::args().collect();
    let token = &args[1];

    let scheme = ShardScheme::try_from((0..1, 1))?;
    let intents = Intents::GUILD_MESSAGES | Intents:: GUILDS;

    let client = Client::new(token);

    let current_user = client.current_user().await?;

    let cluster = Cluster::builder(token, intents)
        .shard_scheme(scheme)
        .http_client(client)
        .build()
        .await?;
    
    let mut events = cluster.events();

    let cluster_spawn = cluster.clone();

    // Set up parser config
    let mut config = CommandParserConfig::new();
    // Add commands
    config.add_command("dab", true);
    // Add prefixes
    config.add_prefix("mp/");
    config.add_prefix(format!("<@{}>", current_user.id));
    config.add_prefix(format!("<@!{}>", current_user.id));

    // Build parser with config
    let parser = Parser::new(config);

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    while let Some((id, event)) = events.next().await {
        tokio::spawn(handle_event(event, id, &parser, client.clone()));
    }

    Ok(())
}


