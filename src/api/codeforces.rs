use reqwest::{Client, header};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Codeforces{
    pub status: String,
    pub result: ResultData,
}

#[derive(Deserialize, Debug)]
pub struct ResultData{
    pub problems: Vec<Problem>,
}

#[derive(Deserialize, Debug)]
pub struct Problem{
    pub contest_id: Option<u32>,
    pub index: String,
    pub name: String,
    #[serde(rename = "type")]
    pub problem_type: String,
    pub points: Option<f32>,
    pub rating: Option<u32>,
    pub tags: Vec<String>,
}

pub async fn get_problems(min_rating: Option<u32>, max_rating: Option<u32>) -> Result<Codeforces, Box<dyn std::error::Error + Send + Sync>> {
    
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
                if let Some(min) = min_rating {
                    if let Some(max) = max_rating {
                        rating >= min && rating <= max
                    } else {
                        rating >= min
                    }
                } else if let Some(max) = max_rating {
                    rating <= max
                } else {
                    true
                }
            } else {
                false // Skip problems without rating
            }
        });
    }

    Ok(response)
}