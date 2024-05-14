use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use scraper::{Html, Selector};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn wisdom(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(fortune().await.unwrap()).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn wise_o_meter(ctx: Context<'_>) -> Result<(), Error> {
    let coin = rand::random();

    if coin {
        ctx.say("Hmmm, yes, very wise").await?;
    } else {
        ctx.say("Mmmm, no, very unwise").await?;
    }
    Ok(())
}
#[poise::command(prefix_command)]
async fn torintalks(ctx: Context<'_>) -> Result<(), Error> {
    let phrase = t_read::t_read("./sweet/sweet.txt").unwrap();
    if ctx.author().id == 792734296237539328 {
        ctx.say(format!("Torin says...\n{phrase}")).await?;
    }
    Ok(())
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("Discord token was not found.")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![wisdom(), wise_o_meter(), torintalks()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}

async fn fortune() -> reqwest::Result<String> {
    let text = reqwest::get("https://www.fortune.levillage.org/cookies.php")
        .await?
        .text()
        .await?;

    let parsed_text = Html::parse_document(&text);
    let cookie_selector = Selector::parse("p").unwrap();

    let paragraph = parsed_text
        .select(&cookie_selector)
        .nth(2)
        .unwrap()
        .text()
        .collect::<Vec<_>>();

    println!("{:?}", &paragraph[1..paragraph.len() - 1]);

    let mut output = String::new();

    for line in &paragraph[1..paragraph.len() - 1] {
        output = output + line;
    }

    output = output.trim().to_string();

    println!("{}", output);

    Ok(output)
}
