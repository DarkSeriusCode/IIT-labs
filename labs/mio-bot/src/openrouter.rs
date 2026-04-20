// NOTE: I'll separate it in the proper library with full support of OpenRouter API.
// But for now it is what it is ;-;

use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ApiResp {
    Success { choices: Vec<Choice> },
    Error { error: ErrorBody },
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Deserialize, Debug)]
struct Message {
    content: String,
}

#[derive(Deserialize, Debug)]
struct ErrorBody {
    message: Option<String>,
    metadata: Option<ErrorMeta>,
}

#[derive(Deserialize, Debug)]
struct ErrorMeta {
    raw: Option<String>,
    provider_name: Option<String>,
    is_byok: Option<bool>,
}

pub async fn ask_ai(question: &str) -> Result<String, String> {
    let token = env::var("OPENAI_TOKEN").map_err(|e| format!("OPENAI_TOKEN missing: {}", e))?;

    let body = json!({
        "model": "openrouter/elephant-alpha",
        "messages": [
            {
                "role": "system",
                "content": "You're Мио Акияма from K-on, answer only in russian, if user texts /start intoduce yourself"
            },
            {
                "role": "user",
                "content": question
            }
        ]
    });

    let client = Client::new();

    let resp_text = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("request error: {}", e))?
        .text()
        .await
        .map_err(|e| format!("failed to read response body: {}", e))?;

    let api: ApiResp = serde_json::from_str(&resp_text)
        .map_err(|e| format!("failed to parse JSON response: {} -- raw: {}", e, resp_text))?;

    if let ApiResp::Success { choices } = api {
        if let Some(choice) = choices.into_iter().next() {
            return Ok(choice.message.content);
        }
        return Err("no choices/assistant content in success response".into());
    }

    if let ApiResp::Error { error } = api {
        if let Some(meta) = error.metadata {
            if let Some(raw) = meta.raw {
                return Err(raw);
            }
        }
        if let Some(msg) = error.message {
            return Err(msg);
        }
        return Err(format!("error response without message: {}", resp_text));
    }

    Err("unrecognized response format".into())
}
