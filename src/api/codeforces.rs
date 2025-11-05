use reqwest::{Client, header};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Codeforces{
    status: String,
    result: ResultData,
}

#[derive(Deserialize, Debug)]
struct ResultData{
    problems: Vec<Problem>,
}

#[derive(Deserialize, Debug)]
struct Problem{
    contestId: Option<u32>,
    index: String,
    name: String,
    #[serde(rename = "type")]
    problem_type: String,
    points: Option<f32>,
    rating: Option<u32>,
    tags: Vec<String>,
}

pub async fn get_problems() -> Result<Codeforces, Box<dyn std::error::Error>> {
    
    let client = Client::new();
    let url = "https://codeforces.com/api/problemset.problems";

    let response = client
        .get(url)
        .header(header::USER_AGENT, "discord-bot/1.0")
        .send()
        .await?
        .json::<Codeforces>()
        .await?;

    Ok(response)
}