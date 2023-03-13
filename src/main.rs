use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct CatFact {
    fact: String,
    length: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://catfact.ninja/fact")
        .await?
        .json::<CatFact>()
        .await?;

    println!("{:#?}", resp);
    Ok(())
}
