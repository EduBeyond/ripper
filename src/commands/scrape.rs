use super::*;

use scraper::{selectable::Selectable, Html};

/// Set a variable
#[derive(Parser)]
pub struct Args {
    url: String,

    #[clap(long, short)]
    class_name: Option<String>,

    #[clap(long, short)]
    id: Option<String>,

    #[clap(long, default_value_t = true)]
    include_head: bool,
}

pub async fn command(args: Args) -> Result<()> {
    let html = reqwest::get(&args.url).await?.text().await?;

    let document = Html::parse_document(&html);

    let head = match args.include_head {
        true => document
            .select(&scraper::Selector::parse("head").unwrap())
            .next(),
        false => None,
    };

    let head = match head {
        Some(head) => head.inner_html(),
        None => "".to_string(),
    };

    let body = document
        .select(&scraper::Selector::parse("body").unwrap())
        .next()
        .unwrap();

    let id_selector =
        scraper::Selector::parse(&format!("#{}", args.id.unwrap_or_default())).unwrap();

    let keep = body.select(&id_selector).next().unwrap();

    let out = format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        {head}
    </head>
    <body>
        {body}
    </body>
</html>
"#,
        head = head,
        body = keep.html(),
    );

    println!("{}", out);

    Ok(())
}
