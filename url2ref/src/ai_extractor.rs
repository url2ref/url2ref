//! AI-based metadata extraction for web pages.
//!
//! This module provides functionality to extract reference metadata using
//! AI language models when standard metadata parsers fail to find values.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::attribute::{Attribute, AttributeType, Author, Date};
use crate::curl;

/// Errors that can occur during AI extraction
#[derive(Error, Debug)]
pub enum AiExtractionError {
    #[error("AI API call failed: {0}")]
    ApiError(String),

    #[error("Failed to parse AI response: {0}")]
    ParseError(String),

    #[error("No API key provided for AI extraction")]
    NoApiKey,

    #[error("HTTP request failed")]
    CurlError(#[from] curl::CurlError),

    #[error("JSON parsing failed")]
    JsonError(#[from] serde_json::Error),
}

/// Supported AI providers
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiProvider {
    #[default]
    OpenAI,
    Anthropic,
}

/// Options for AI-based metadata extraction
#[derive(Clone, Default)]
pub struct AiExtractionOptions {
    /// Whether AI extraction is enabled
    pub enabled: bool,
    /// Which AI provider to use
    pub provider: AiProvider,
    /// API key for the selected provider
    pub api_key: Option<String>,
    /// Model to use (e.g., "gpt-4o-mini", "claude-3-haiku-20240307")
    pub model: Option<String>,
}

/// Response structure for extracted metadata from AI
#[derive(Debug, Deserialize, Serialize)]
pub struct AiExtractedMetadata {
    pub title: Option<String>,
    pub authors: Option<Vec<String>>,
    pub date: Option<String>,
    pub site: Option<String>,
    pub publisher: Option<String>,
    pub language: Option<String>,
}

impl Default for AiExtractedMetadata {
    fn default() -> Self {
        Self {
            title: None,
            authors: None,
            date: None,
            site: None,
            publisher: None,
            language: None,
        }
    }
}

/// OpenAI API request structures
#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    response_format: OpenAiResponseFormat,
    temperature: f32,
}

#[derive(Serialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAiResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessageResponse,
}

#[derive(Deserialize)]
struct OpenAiMessageResponse {
    content: String,
}

/// Anthropic API request structures
#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

#[derive(Deserialize)]
struct AnthropicContent {
    text: String,
}

/// Extract text content from HTML, stripping tags
fn extract_text_from_html(html: &str) -> String {
    // Simple HTML tag stripping - remove script and style content first
    let no_script = regex::Regex::new(r"(?is)<script[^>]*>.*?</script>")
        .map(|re| re.replace_all(html, "").to_string())
        .unwrap_or_else(|_| html.to_string());
    
    let no_style = regex::Regex::new(r"(?is)<style[^>]*>.*?</style>")
        .map(|re| re.replace_all(&no_script, "").to_string())
        .unwrap_or_else(|_| no_script.clone());
    
    // Remove all HTML tags
    let no_tags = regex::Regex::new(r"<[^>]+>")
        .map(|re| re.replace_all(&no_style, " ").to_string())
        .unwrap_or_else(|_| no_style.clone());
    
    // Decode HTML entities
    let decoded = htmlescape::decode_html(&no_tags)
        .unwrap_or_else(|_| no_tags.clone());
    
    // Normalize whitespace
    let normalized = regex::Regex::new(r"\s+")
        .map(|re| re.replace_all(&decoded, " ").to_string())
        .unwrap_or_else(|_| decoded.clone());
    
    // Truncate to reasonable length for API calls (first ~4000 chars)
    let truncated: String = normalized.chars().take(4000).collect();
    truncated.trim().to_string()
}

/// Build the system prompt for metadata extraction
fn build_system_prompt() -> String {
    r#"You are a metadata extraction assistant. Your task is to extract bibliographic reference information from web page content.

Extract the following fields if present:
- title: The main title of the article/page
- authors: List of author names (as an array of strings). Look for bylines like "By John Smith" or author credits.
- date: Publication date in ISO format (YYYY-MM-DD) if possible
- site: The website or publication name (e.g., "The New York Times", "BBC News", "Ekstra Bladet")
- publisher: The publishing organization or company. If publishing organization is not explicitly stated, use your best judgment based on the site.
- language: ISO 639-1 language code (e.g., "en", "de", "fr", "da" for Danish)

Return ONLY a valid JSON object with these fields. Use null for fields you cannot determine.
Do not include any explanation or markdown formatting."#.to_string()
}

/// Build the user prompt with the page content
fn build_user_prompt(url: &str, text_content: &str) -> String {
    format!(
        "Extract metadata from this web page.\n\nURL: {}\n\nContent:\n{}",
        url, text_content
    )
}

/// Extract metadata using OpenAI API
fn extract_with_openai(
    url: &str,
    html_content: &str,
    options: &AiExtractionOptions,
) -> Result<AiExtractedMetadata, AiExtractionError> {
    let api_key = options.api_key.as_ref().ok_or(AiExtractionError::NoApiKey)?;
    let model = options.model.clone().unwrap_or_else(|| "gpt-4o-mini".to_string());
    
    let text_content = extract_text_from_html(html_content);
    
    let request = OpenAiRequest {
        model,
        messages: vec![
            OpenAiMessage {
                role: "system".to_string(),
                content: build_system_prompt(),
            },
            OpenAiMessage {
                role: "user".to_string(),
                content: build_user_prompt(url, &text_content),
            },
        ],
        response_format: OpenAiResponseFormat {
            format_type: "json_object".to_string(),
        },
        temperature: 0.1,
    };
    
    let request_body = serde_json::to_string(&request)?;
    
    println!("[url2ref] AI OpenAI: Sending request to API with model: {}", request.model);
    
    let response = curl::post_json(
        "https://api.openai.com/v1/chat/completions",
        &request_body,
        &[("Authorization", &format!("Bearer {}", api_key))],
    ).map_err(|e| AiExtractionError::ApiError(e.to_string()))?;
    
    println!("[url2ref] AI OpenAI: Received response (length: {} bytes)", response.len());
    
    let response: OpenAiResponse = serde_json::from_str(&response)?;
    
    let content = response.choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| AiExtractionError::ParseError("No response from API".to_string()))?;
    
    println!("[url2ref] AI OpenAI: Extracted content: {}", content);
    
    let metadata: AiExtractedMetadata = serde_json::from_str(&content)
        .map_err(|e| AiExtractionError::ParseError(format!("Failed to parse JSON: {}", e)))?;
    
    Ok(metadata)
}

/// Extract metadata using Anthropic API
fn extract_with_anthropic(
    url: &str,
    html_content: &str,
    options: &AiExtractionOptions,
) -> Result<AiExtractedMetadata, AiExtractionError> {
    let api_key = options.api_key.as_ref().ok_or(AiExtractionError::NoApiKey)?;
    let model = options.model.clone().unwrap_or_else(|| "claude-3-haiku-20240307".to_string());
    
    let text_content = extract_text_from_html(html_content);
    
    let combined_prompt = format!(
        "{}\n\n{}",
        build_system_prompt(),
        build_user_prompt(url, &text_content)
    );
    
    let request = AnthropicRequest {
        model,
        max_tokens: 1024,
        messages: vec![
            AnthropicMessage {
                role: "user".to_string(),
                content: combined_prompt,
            },
        ],
    };
    
    let request_body = serde_json::to_string(&request)?;
    
    let response = curl::post_json(
        "https://api.anthropic.com/v1/messages",
        &request_body,
        &[
            ("x-api-key", api_key),
            ("anthropic-version", "2023-06-01"),
        ],
    ).map_err(|e| AiExtractionError::ApiError(e.to_string()))?;
    
    let response: AnthropicResponse = serde_json::from_str(&response)?;
    
    let content = response.content
        .first()
        .map(|c| c.text.clone())
        .ok_or_else(|| AiExtractionError::ParseError("No response from API".to_string()))?;
    
    // Try to extract JSON from the response (Anthropic might include extra text)
    let json_content = extract_json_from_text(&content)?;
    
    let metadata: AiExtractedMetadata = serde_json::from_str(&json_content)
        .map_err(|e| AiExtractionError::ParseError(format!("Failed to parse JSON: {}", e)))?;
    
    Ok(metadata)
}

/// Try to extract a JSON object from text that might contain other content
fn extract_json_from_text(text: &str) -> Result<String, AiExtractionError> {
    // First try to parse the whole thing
    if serde_json::from_str::<serde_json::Value>(text).is_ok() {
        return Ok(text.to_string());
    }
    
    // Try to find JSON object in the text
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            let potential_json = &text[start..=end];
            if serde_json::from_str::<serde_json::Value>(potential_json).is_ok() {
                return Ok(potential_json.to_string());
            }
        }
    }
    
    Err(AiExtractionError::ParseError("Could not find valid JSON in response".to_string()))
}

/// Main entry point for AI-based metadata extraction
pub fn extract_metadata(
    url: &str,
    html_content: &str,
    options: &AiExtractionOptions,
) -> Result<AiExtractedMetadata, AiExtractionError> {
    if !options.enabled {
        return Ok(AiExtractedMetadata::default());
    }
    
    match options.provider {
        AiProvider::OpenAI => extract_with_openai(url, html_content, options),
        AiProvider::Anthropic => extract_with_anthropic(url, html_content, options),
    }
}

/// Convert AI-extracted metadata to an Attribute for a specific field
pub fn get_attribute_from_ai(
    metadata: &AiExtractedMetadata,
    attribute_type: AttributeType,
) -> Option<Attribute> {
    match attribute_type {
        AttributeType::Title => metadata.title.clone().map(Attribute::Title),
        AttributeType::Author => {
            metadata.authors.as_ref().and_then(|authors| {
                if authors.is_empty() {
                    None
                } else {
                    Some(Attribute::Authors(
                        authors.iter().map(|a| Author::Generic(a.clone())).collect()
                    ))
                }
            })
        }
        AttributeType::Date => {
            metadata.date.as_ref().and_then(|date_str| {
                parse_ai_date(date_str).map(Attribute::Date)
            })
        }
        AttributeType::Site => metadata.site.clone().map(Attribute::Site),
        AttributeType::Publisher => metadata.publisher.clone().map(Attribute::Publisher),
        AttributeType::Language => metadata.language.clone().map(Attribute::Language),
        _ => None,
    }
}

/// Parse a date string from AI response into a Date
fn parse_ai_date(date_str: &str) -> Option<Date> {
    use chrono::NaiveDate;
    
    // Try ISO format first (YYYY-MM-DD)
    if let Ok(nd) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        return Some(Date::YearMonthDay(nd));
    }
    
    // Try year-month (YYYY-MM)
    if date_str.len() == 7 {
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() == 2 {
            if let (Ok(year), Ok(month)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                return Some(Date::YearMonth { year, month });
            }
        }
    }
    
    // Try just year (YYYY)
    if date_str.len() == 4 {
        if let Ok(year) = date_str.parse::<i32>() {
            return Some(Date::Year(year));
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_text_from_html() {
        let html = r#"<html><head><script>alert('x')</script></head>
            <body><h1>Title</h1><p>Content here</p></body></html>"#;
        let text = extract_text_from_html(html);
        assert!(text.contains("Title"));
        assert!(text.contains("Content here"));
        assert!(!text.contains("script"));
        assert!(!text.contains("alert"));
    }
    
    #[test]
    fn test_parse_ai_date() {
        assert!(parse_ai_date("2024-01-15").is_some());
        assert!(parse_ai_date("2024-01").is_some());
        assert!(parse_ai_date("2024").is_some());
        assert!(parse_ai_date("invalid").is_none());
    }
    
    #[test]
    fn test_extract_json_from_text() {
        let text = r#"Here's the JSON: {"title": "Test"} hope it helps"#;
        let json = extract_json_from_text(text).unwrap();
        assert_eq!(json, r#"{"title": "Test"}"#);
    }
}
