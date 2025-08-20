use anyhow::Result;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    content: Vec<Content>,
}

#[derive(Debug, Deserialize)]
struct Content {
    text: String,
}

pub struct AiIsEven {
    api_key: String,
    client: Client,
}

impl AiIsEven {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    pub fn is_even(&self, number: i32) -> Result<String> {
        let prompt = format!(
            "Is {} an even number? You must respond with EXACTLY 'True' or 'False' (case-sensitive, no other text, no punctuation, no explanation).",
            number
        );

        let request = ClaudeRequest {
            model: "claude-opus-4-1-20250805".to_string(),
            max_tokens: 10,
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "API request failed with status: {}",
                response.status()
            ));
        }

        let claude_response: ClaudeResponse = response.json()?;
        let answer = claude_response
            .content
            .first()
            .ok_or_else(|| anyhow::anyhow!("No response from Claude"))?
            .text
            .trim();

        let hash = self.hash_response(answer);
        Ok(hash)
    }

    fn hash_response(&self, response: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(response.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}

pub async fn is_even_async(api_key: String, number: i32) -> Result<String> {
    let prompt = format!(
        "Is {} an even number? You must respond with EXACTLY 'True' or 'False' (case-sensitive, no other text, no punctuation, no explanation).",
        number
    );

    let request = ClaudeRequest {
        model: "claude-opus-4-1-20250805".to_string(),
        max_tokens: 10,
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "API request failed with status: {}",
            response.status()
        ));
    }

    let claude_response: ClaudeResponse = response.json().await?;
    let answer = claude_response
        .content
        .first()
        .ok_or_else(|| anyhow::anyhow!("No response from Claude"))?
        .text
        .trim();

    let mut hasher = Sha256::new();
    hasher.update(answer.as_bytes());
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_response() {
        let ai_is_even = AiIsEven::new("dummy_key".to_string());
        
        let true_hash = ai_is_even.hash_response("True");
        println!("SHA256('True') = {}", true_hash);
        
        let false_hash = ai_is_even.hash_response("False");
        println!("SHA256('False') = {}", false_hash);
        
        assert_ne!(true_hash, false_hash);
    }
}