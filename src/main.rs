use poise::serenity_prelude as serenity;
use quraan_discord::{
    commands::{ayah, ayah_ar_tafseer, list_suar, page, surah, surah_text},
    parse_tafseer_mouaser, Data,
};
use serenity::GatewayIntents;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("no .env file");

    let quraan = parse_tafseer_mouaser().expect("parse tafseer mouaser xml");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                surah(),
                surah_text(),
                ayah(),
                ayah_ar_tafseer(),
                page(),
                list_suar(),
            ],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(GatewayIntents::non_privileged().union(GatewayIntents::MESSAGE_CONTENT))
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { quraan })
            })
        });

    framework.run().await.unwrap();
}
