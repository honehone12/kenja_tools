use tokio::fs;
use clap::Parser;
use anyhow::bail;
use kenja_tools::documents::local::Img;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long)]
    pattern: String,
    #[arg(long)]
    img_list: String
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if !fs::try_exists(&args.img_list).await? {
        bail!("could not find img list file");
    }

    let s = fs::read_to_string(&args.img_list).await?;
    let img_list = serde_json::from_str::<Vec<Img>>(&s)?;

    let clean = img_list.into_iter().filter(|i| {
        match &i.path {
            Some(p) => !p.contains(&args.pattern),
            None => false
        }
    }).collect::<Vec<Img>>();

    let s = serde_json::to_string(&clean)?;
    fs::write(&args.img_list, s).await?;

    Ok(())
}
