use twilight_command_parser::{CommandParserConfig, Command, Parser};
use twilight_http::Client as Client;
use twilight_gateway::{
    cluster::{Cluster, ShardScheme},
    Intents};

#[tokio::main]
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = env::var("TOKEN")?;
    let shard = Shard::new(token, );

    let scheme = ShardScheme::try_from((0..1, 1))?;
    let intents = Intents::GUILD_MESSAGES | Intents:: GUILDS;

    let cluster = Cluster::builder(token, intents)
        .shard_scheme(scheme)
        .build()
        .await?;
    
    let mut events = cluster.events();

    let cluster_spawn = cluster.clone();

    let mut commands = CommandParserConfig::new();
    commands.add_prefix("mp/");
    //commands.add_prefix(println("<@{}>", shard.id));

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    while let Some(event) = events.next().await {
        println!("EVENT: {:?}", event);
    }

    Ok(())
}


