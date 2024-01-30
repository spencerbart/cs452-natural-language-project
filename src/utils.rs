use anyhow::Result;
use serde_json::{json, Value};

const GPT_URL: &str = "https://api.openai.com/v1/chat/completions";
const MODEL: &str = "gpt-3.5-turbo";

pub async fn gpt_request(system_message: String, user_message: String) -> Result<String> {
    let client = reqwest::Client::new();

    let response = client
        .post(GPT_URL)
        .json(&json!({
            "model": MODEL,
            "messages": [
                {
                    "role": "system",
                    "content": system_message
                },
                {
                    "role": "user",
                    "content": user_message
                }
            ]
        }))
        .bearer_auth(std::env::var("OPENAI_API_KEY")?)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if !response.status().is_success() {
        println!("{}", response.text().await?);
        return Err(anyhow::anyhow!("GPT request failed"));
    }

    let response = response.json::<Value>().await?;

    Ok(response.get("choices").unwrap()[0]
        .get("message")
        .unwrap()
        .get("content")
        .unwrap()
        .to_string())
}
