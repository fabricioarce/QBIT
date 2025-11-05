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

pub async fn get_problems(min_rating: Option<u32>, max_rating: Option<u32>) -> Result<Codeforces, Box<dyn std::error::Error>> {
    
    let client = Client::new();
    let url = "https://codeforces.com/api/problemset.problems";

    let mut response = client
        .get(url)
        .header(header::USER_AGENT, "discord-bot/1.0")
        .send()
        .await?
        .json::<Codeforces>()
        .await?;

    // Filter problems by rating if ranges are specified
    if min_rating.is_some() || max_rating.is_some() {
        response.result.problems.retain(|problem| {
            if let Some(rating) = problem.rating {
                match (min_rating, max_rating) {
                    (Some(min), Some(max)) => rating >= min && rating <= max,
                    (Some(min), None) => rating >= min,
                    (None, Some(max)) => rating <= max,
                    (None, None) => true,
                }
            } else {
                false // Skip problems without rating
            }
        });
    }

    Ok(response)
}