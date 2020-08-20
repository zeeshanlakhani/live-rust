use reqwest::{self, Error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Repo {
    #[serde(rename = "stargazers_count")]
    pub stars: u64,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}",
        owner = "capsule-rs",
        repo = "capsule"
    );
    println!("{}", request_url);
    let client = reqwest::Client::new();
    let repo = client
        .get(request_url.as_str())
        .header(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(env!("CARGO_PKG_NAME")),
        )
        .send()
        .await?
        .json::<Repo>()
        .await?;

    println!("{:?}", repo.stars);
    Ok(())
}
