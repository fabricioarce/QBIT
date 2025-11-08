// =====================
//   Imports
// =====================
use reqwest::{Client, header};
use serde::Deserialize;

// =====================
//   API Response Structures
// =====================

// =====================
//   Main Codeforces Response
// =====================
// Root structure for Codeforces API response
#[derive(Deserialize, Debug)]
pub struct Codeforces {
    pub status: String,
    pub result: ResultData,
}

// =====================
//   Result Data Structure
// =====================
// Contains the actual problem data from the API
#[derive(Deserialize, Debug)]
pub struct ResultData {
    pub problems: Vec<Problem>,
}

// =====================
//   Problem Structure
// =====================
// Represents a single Codeforces problem with all its metadata
#[derive(Deserialize, Debug)]
pub struct Problem {
    pub contest_id: Option<u32>,
    pub index: String,
    pub name: String,
    #[serde(rename = "type")]
    pub problem_type: String,
    pub points: Option<f32>,
    pub rating: Option<u32>,
    pub tags: Vec<String>,
}

// =====================
//   Get Problems Function
// =====================
// Fetches problems from Codeforces API with optional rating filter
pub async fn get_problems(
    min_rating: Option<u32>,
    max_rating: Option<u32>,
) -> Result<Codeforces, Box<dyn std::error::Error + Send + Sync>> {
    // =====================
    //   HTTP Client Setup
    // =====================
    // Create new HTTP client for API requests
    let client = Client::new();
    let url = "https://codeforces.com/api/problemset.problems";

    // =====================
    //   API Request
    // =====================
    // Send GET request to Codeforces API and parse JSON response
    let mut response = client
        .get(url)
        .header(header::USER_AGENT, "discord-bot/1.0")
        .send()
        .await?
        .json::<Codeforces>()
        .await?;

    // =====================
    //   Rating Filter
    // =====================
    // Filter problems by rating if ranges are specified
    if min_rating.is_some() || max_rating.is_some() {
        response.result.problems.retain(|problem| {
            if let Some(rating) = problem.rating {
                // Check if problem rating is within specified range
                if let Some(min) = min_rating {
                    if let Some(max) = max_rating {
                        // Both min and max specified
                        rating >= min && rating <= max
                    } else {
                        // Only min specified
                        rating >= min
                    }
                } else if let Some(max) = max_rating {
                    // Only max specified
                    rating <= max
                } else {
                    // No filter (shouldn't reach here)
                    true
                }
            } else {
                // Skip problems without rating
                false
            }
        });
    }

    Ok(response)
}
