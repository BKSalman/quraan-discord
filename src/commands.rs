use std::collections::HashSet;

use crate::get_page_image_name;
use crate::{Context, Error};

/// get a specific surah by the Arabic name of the surah
#[poise::command(slash_command, prefix_command)]
pub async fn list_suar(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();

    let suar = data.quraan.ayat.iter().fold(Vec::new(), |mut acc, ayah| {
        let surah_name = ayah.sura_name_ar.trim().to_string();
        if !acc.contains(&format!("- {}\n", surah_name)) {
            acc.push(format!("- {}\n", surah_name));
        }
        acc
    });

    ctx.send(|r| r.content(suar.concat())).await?;

    Ok(())
}

/// get a specific surah by the Arabic name of the surah
#[poise::command(slash_command, prefix_command)]
pub async fn page(
    ctx: Context<'_>,
    #[description = "Page number"] page_number: u32,
) -> Result<(), Error> {
    let path = get_page_image_name(page_number);

    let file = tokio::fs::File::open(path)
        .await
        .expect("could not open page file, did you extract the pages from `data/arabic-quran`?");

    ctx.send(|r| {
        r.attachment(poise::serenity_prelude::AttachmentType::File {
            file: &file,
            filename: format!("{page_number}.png"),
        })
    })
    .await?;

    Ok(())
}

/// get a specific surah by the Arabic name of the surah
#[poise::command(slash_command, prefix_command)]
pub async fn surah_text(
    ctx: Context<'_>,
    #[description = "Name of the surah"] name: String,
) -> Result<(), Error> {
    let data = ctx.data();

    ctx.defer().await?;

    let surah_ayat_pages: Vec<_> = data
        .quraan
        .ayat
        .iter()
        .filter(|a| a.sura_name_ar == name || a.sura_name_en == name)
        .collect();

    if surah_ayat_pages.len() < 1 {
        ctx.send(|f| f.content(format!("No surah with the name: {name}")))
            .await?;

        return Ok(());
    }

    let (messages, _, _) = surah_ayat_pages.iter().fold(
        (Vec::from([String::new()]), 0, 0),
        |(mut messages, mut current, mut length), ayah| {
            if length >= 1700 {
                messages.push(String::new());
                current += 1;
                length = 0;
            }

            let ayah_text = format!("{} ({}) ", ayah.aya_text_emlaey, ayah.aya_no);
            length += ayah_text.chars().count();
            messages[current].push_str(&ayah_text);

            (messages, current, length)
        },
    );

    for message in messages {
        ctx.send(|f| f.content(message)).await?;
    }

    Ok(())
}

/// get a specific surah by the Arabic name of the surah
#[poise::command(slash_command, prefix_command)]
pub async fn surah(
    ctx: Context<'_>,
    #[description = "Name of the surah"] name: String,
) -> Result<(), Error> {
    let data = ctx.data();

    ctx.defer().await?;

    let surah_ayat_pages: Vec<_> = data
        .quraan
        .ayat
        .iter()
        .filter(|a| a.sura_name_ar == name || a.sura_name_en == name)
        .collect();

    if surah_ayat_pages.len() < 1 {
        ctx.send(|f| f.content(format!("No surah with the name: {name}")))
            .await?;

        return Ok(());
    }

    let surah_ayat_pages = surah_ayat_pages.iter().fold(
        HashSet::with_capacity(surah_ayat_pages.iter().map(|a| a.page).max().unwrap() as usize),
        |mut acc, ayah| {
            acc.insert(ayah.page);

            acc
        },
    );

    if surah_ayat_pages.len() < 1 {
        ctx.send(|f| f.content(format!("Could not get pages for surah: {name}")))
            .await?;

        return Ok(());
    }

    let mut surah_ayat_pages = surah_ayat_pages.into_iter().collect::<Vec<_>>();

    surah_ayat_pages.sort();

    for chunk in surah_ayat_pages.chunks(8) {
        let mut files = Vec::new();
        for page_number in chunk {
            let path = get_page_image_name(*page_number);

            let file = tokio::fs::File::open(path).await.expect(
                "could not open page file, did you extract the pages from `data/arabic-quran`?",
            );
            files.push((file, format!("{page_number}.png")));
        }

        ctx.send(|r| {
            for (file, filename) in files.iter() {
                r.attachment(poise::serenity_prelude::AttachmentType::File {
                    file: &file,
                    filename: filename.to_string(),
                });
            }

            r
        })
        .await?;
    }

    Ok(())
}

/// get a specific ayah by the number and the Arabic name of the surah
#[poise::command(slash_command, prefix_command)]
pub async fn ayah(
    ctx: Context<'_>,
    #[description = "Name of the surah"] surah_name: String,
    #[description = "Number of the ayah"] ayah_number: u32,
) -> Result<(), Error> {
    if let Some(ayah) = ctx.data().quraan.ayat.iter().find(|a| {
        (a.sura_name_ar == surah_name || a.sura_name_en == surah_name) && a.aya_no == ayah_number
    }) {
        ctx.send(|f| f.content(format!("{} ({})", ayah.aya_text, ayah.aya_no)))
            .await?;
    } else {
        ctx.send(|f| {
            f.content(format!(
                "Ayah number {ayah_number} does not exist in surah {surah_name}"
            ))
        })
        .await?;
    }

    Ok(())
}

/// get a specific ayah's Arabic tafseer by the number and the Arabic name of the surah
#[poise::command(slash_command, prefix_command)]
pub async fn ayah_ar_tafseer(
    ctx: Context<'_>,
    #[description = "Name of the surah"] surah_name: String,
    #[description = "Number of the ayah"] ayah_number: u32,
) -> Result<(), Error> {
    let data = ctx.data();

    if let Some(ayah) = data
        .quraan
        .ayat
        .iter()
        .find(|p| p.aya_no == ayah_number && p.sura_name_ar == surah_name)
    {
        ctx.send(|f| {
            f.content(format!(
                "التفسير الميسر لسورة {surah_name} - آية {ayah_number}\n\n {}",
                ayah.aya_tafseer
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

    Ok(())
}
