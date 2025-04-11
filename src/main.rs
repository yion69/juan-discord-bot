extern crate serenity;

use std::vec;

use dotenv::dotenv;
use poise::serenity_prelude ;
#[allow(unused_imports)]
use serenity::{all::{Context, EventHandler, GatewayIntents, Message, Ready}, async_trait, client::Client};

use discord_bot::weather_crate::{ WeatherResponse, fetch_weather,weather_response_format };

use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

#[allow(dead_code)]
struct UserData {
    pub key: String
}
#[allow(dead_code)]
type PoiseError = Box<dyn std::error::Error + Send + Sync>;
#[allow(dead_code)]
type PoiseContext<'a> = poise::Context<'a, UserData, PoiseError>;

#[allow(dead_code)]
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                eprintln!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[poise::command(prefix_command, slash_command)]
async fn ping(
    ctx: PoiseContext<'_>,
) -> Result<(), PoiseError> {
    ctx.say("pong").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, subcommands("custom","chiangmai","singapore","sanjose"))]
async fn weather(
    ctx: PoiseContext<'_>
) -> Result<(), PoiseError> {
    ctx.say("weather city_name | e.g. weather Yangon").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
async fn custom(
    ctx: poise::Context<'_,UserData, PoiseError>, 
    #[description = "City Name"] city_name: String,
    #[description = "Country Code"] country_code: String
) -> Result<(), PoiseError> {

    let key = &ctx.data().key;
    let result: WeatherResponse  = match fetch_weather(&city_name, &country_code, &key).await {
        Ok(res) => res,
        Err(e) => { 
            eprintln!("Error => {}",e);
            return Err(Box::new(poise::serenity_prelude::Error::Other("Failed to fetch weather data")))
         }
    };

    let formatted_ctx = weather_response_format(result);
    ctx.say(formatted_ctx).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
async fn sanjose(
    ctx: poise::Context<'_,UserData, PoiseError>
) -> Result<(), PoiseError> {

    let key = &ctx.data().key;
    let result: WeatherResponse  = match fetch_weather("San Jose", "US", &key).await {
        Ok(res) => res,
        Err(e) => { 
            eprintln!("Error => {}",e);
            return Err(Box::new(poise::serenity_prelude::Error::Other("Failed to fetch weather data")))
         }
    };

    let formatted_ctx = weather_response_format(result);
    ctx.say(formatted_ctx).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
async fn singapore(
    ctx: poise::Context<'_,UserData, PoiseError>
) -> Result<(), PoiseError> {

    let key = &ctx.data().key;
    let result: WeatherResponse  = match fetch_weather("Singapore", "SG", &key).await {
        Ok(res) => res,
        Err(e) => { 
            eprintln!("Error => {}",e);
            return Err(Box::new(poise::serenity_prelude::Error::Other("Failed to fetch weather data")))
         }
    };

    let formatted_ctx = weather_response_format(result);
    ctx.say(formatted_ctx).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
async fn chiangmai(
    ctx: poise::Context<'_,UserData, PoiseError>
) -> Result<(), PoiseError> {

    let key = &ctx.data().key;
    let result: WeatherResponse  = match fetch_weather("Chiang Mai", "TH", &key).await {
        Ok(res) => res,
        Err(e) => { 
            eprintln!("Error => {}",e);
            return Err(Box::new(poise::serenity_prelude::Error::Other("Failed to fetch weather data")))
         }
    };

    let formatted_ctx = weather_response_format(result);
    ctx.say(formatted_ctx).await?;

    Ok(())
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {

    dotenv().ok();

    let discord_token = secret_store.get("DISCORD_TOKEN").expect("Token not found");
    let api_key = secret_store.get("API_KEY").expect("Key not found");

    let intents  = serenity_prelude::GatewayIntents::non_privileged();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                ..Default::default()
            },
            commands: vec![ping(),weather()],
            ..Default::default()
         })
         .setup(|ctx, _ready, framework | {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(UserData {
                    key: api_key
                })
            })
         })
        .build();

    let client = serenity_prelude::ClientBuilder::new(discord_token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?; 

    Ok(client.into())
}

