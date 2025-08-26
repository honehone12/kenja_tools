use std::env;
use mongodb::{bson::{doc, Document}, Client as MongoClient};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;

    let source_db = env::var("MRG_SRC_DB")?;
    let source_cl = env::var("MRG_SRC_CL")?;
    let target_db = env::var("MRG_DST_DB")?;
    let target_cl = env::var("MRG_DST_CL")?;

    let mongo_uri = env::var("MONGO_URI")?;
    let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;    

    let source = mongo_client.database(&source_db).collection::<Document>(&source_cl);

    source.aggregate(vec![
        // if there are duplicated docs, replace will actually throw error
        // because mongo does not automatically remove docs with _id.
        // so remove _id first here.
        doc! {"$unset": "_id"},
        doc! {"$merge": doc! {
            "into": doc! {"db": target_db, "coll": target_cl},
            "on": "mal_id",
            "whenMatched": "replace"
        }}
    ]).await?;

    info!("done");
    Ok(())
}
