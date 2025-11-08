// =====================
//   Imports and Dependencies
// =====================

use reqwest::{header, Client};
use serde::Deserialize;

// =====================
//   API Response Structures
// =====================

// Main response structure from Codeforces API
#[derive(Deserialize, Debug)]
pub struct Codeforces {
    pub status: String,
    pub result: ResultData,
}

// Result data containing the array of problems
#[derive(Deserialize, Debug)]
pub struct ResultData {
    pub problems: Vec<Problem>,
}

// Individual problem structure with all relevant fields
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
//   Codeforces API Integration
// =====================

// Main function to fetch problems from Codeforces API
// Supports optional rating range filtering (min_rating and max_rating)
pub async fn get_problems(
    min_rating: Option<u32>,
    max_rating: Option<u32>,
) -> Result<Codeforces, Box<dyn std::error::Error + Send + Sync>> {
    // =====================
    //   HTTP Client Setup
    // =====================

    // Initialize HTTP client for API requests
    let client = Client::new();
    let url = "https://codeforces.com/api/problemset.problems";

    // =====================
    //   API Request Execution
    // =====================

    // Make GET request to Codeforces API with User-Agent header
    let mut response = client
        .get(url)
        .header(header::USER_AGENT, "discord-bot/1.0")
        .send()
        .await?
        .json::<Codeforces>()
        .await?;

    // =====================
    //   Rating-Based Filtering
    // =====================

    // Filter problems by rating if ranges are specified
    if min_rating.is_some() || max_rating.is_some() {
        response.result.problems.retain(|problem| {
            if let Some(rating) = problem.rating {
                // Check rating against specified bounds
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
                    // Neither specified (shouldn't happen but handle gracefully)
                    true
                }
            } else {
                // Skip problems without rating when filtering is requested
                false
            }
        });
    }

    // =====================
    //   Response Return
    // =====================

    // Return filtered response
    Ok(response)
}
