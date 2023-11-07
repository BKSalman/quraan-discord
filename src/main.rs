use poise::serenity_prelude as serenity;
use quraan_discord::{parse_xml, Quraan};
use serenity::GatewayIntents;

struct Data {
    quraan: Quraan,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// get a specific surah by the Arabic name of the surah
#[poise::command(slash_command, prefix_command)]
async fn surah(
    ctx: Context<'_>,
    #[description = "Name of the surah"] name: String,
) -> Result<(), Error> {
    if let Some(surah) = ctx.data().quraan.suar.iter().find(|s| s.name == name) {
        let ayat = surah
            .ayat
            .iter()
            .map(|a| format!("{} ({}) ", a.text, a.index))
            .collect::<Vec<String>>()
            .concat();
        ctx.send(|f| f.content(ayat)).await?;
    } else {
        ctx.send(|f| f.content("No surah with this name")).await?;
    }

    Ok(())
}

/// get a specific ayah by the number and the Arabic name of the surah
#[poise::command(slash_command, prefix_command)]
async fn ayah(
    ctx: Context<'_>,
    #[description = "Name of the surah"] surah_name: String,
    #[description = "Number of the ayah"] ayah_number: usize,
) -> Result<(), Error> {
    if let Some(surah) = ctx.data().quraan.suar.iter().find(|s| s.name == surah_name) {
        if let Some(ayah) = surah.ayat.iter().find(|a| a.index == ayah_number) {
            ctx.send(|f| f.content(format!("{} ({})", ayah.text, ayah.index)))
                .await?;
        } else {
            ctx.send(|f| {
                f.content(format!(
                    "Ayah number {ayah_number} does not exist in surah {surah_name}"
                ))
            })
            .await?;
        }
    } else {
        ctx.send(|f| f.content("No surah with this name")).await?;
    }

    Ok(())
}

/// get a specific ayah's Arabic tafseer by the number and the Arabic name of the surah
#[poise::command(slash_command, prefix_command)]
async fn ayah_ar_tafseer(
    ctx: Context<'_>,
    #[description = "Name of the surah"] surah_name: String,
    #[description = "Number of the ayah"] ayah_number: usize,
) -> Result<(), Error> {
    if let Some(surah) = ctx.data().quraan.suar.iter().find(|s| s.name == surah_name) {
        if let Some(ayah) = surah.ayat.iter().find(|a| a.index == ayah_number) {
            ctx.send(|f| {
                f.content(format!(
                    "{}",
                    ayah.tafseer
                        .get("ar")
                        .unwrap_or(&String::from("Arabic tafseer was not loaded"))
                ))
            })
            .await?;
        } else {
            ctx.send(|f| {
                f.content(format!(
                    "Ayah number {ayah_number} does not exist in surah {surah_name}"
                ))
            })
            .await?;
        }
    } else {
        ctx.send(|f| f.content("No surah with this name")).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("no .env file");

    let quraan = parse_xml().expect("parse quraan xml");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![surah(), ayah(), ayah_ar_tafseer()],
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
