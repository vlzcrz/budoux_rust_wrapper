//! BudouX Rust Wrapper
//!
//! A Rust implementation of [BudouX](https://github.com/google/budoux),
//! a line break organizer tool for Japanese text.
//!
//! # Example
//!
//! ```
//! use budoux_rust_wrapper::load_default_japanese_parser;
//!
//! let parser = load_default_japanese_parser();
//! let result = parser.parse("今日は天気です。");
//! assert_eq!(result, vec!["今日は", "天気です。"]);
//! ```

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Error type for BudouX operations
#[derive(Error, Debug)]
pub enum BudouXError {
    #[error("Failed to load model: {0}")]
    ModelLoadError(String),
}

type Result<T> = std::result::Result<T, BudouXError>;

/// Feature type in the model
type Feature = HashMap<String, i32>;

/// Model type containing feature scores
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    /// Unigram features with window size 1
    #[serde(rename = "UW1")]
    pub uw1: Feature,
    /// Unigram features with window size 2
    #[serde(rename = "UW2")]
    pub uw2: Feature,
    /// Unigram features with window size 3
    #[serde(rename = "UW3")]
    pub uw3: Feature,
    /// Unigram features with window size 4
    #[serde(rename = "UW4")]
    pub uw4: Feature,
    /// Unigram features with window size 5
    #[serde(rename = "UW5")]
    pub uw5: Feature,
    /// Unigram features with window size 6
    #[serde(rename = "UW6")]
    pub uw6: Feature,
    /// Bigram features with window size 1
    #[serde(rename = "BW1")]
    pub bw1: Feature,
    /// Bigram features with window size 2
    #[serde(rename = "BW2")]
    pub bw2: Feature,
    /// Bigram features with window size 3
    #[serde(rename = "BW3")]
    pub bw3: Feature,
    /// Trigram features with window size 1
    #[serde(rename = "TW1")]
    pub tw1: Feature,
    /// Trigram features with window size 2
    #[serde(rename = "TW2")]
    pub tw2: Feature,
    /// Trigram features with window size 3
    #[serde(rename = "TW3")]
    pub tw3: Feature,
    /// Trigram features with window size 4
    #[serde(rename = "TW4")]
    pub tw4: Feature,
}

/// The Japanese model data embedded in the binary
static JAPANESE_MODEL: Lazy<Model> = Lazy::new(|| {
    let model_json = include_str!("models/ja.json");
    serde_json::from_str(model_json).expect("Failed to parse Japanese model")
});

/// BudouX parser for segmenting text
#[derive(Debug, Clone)]
pub struct Parser {
    model: Model,
}

impl Parser {
    /// Create a new parser with the given model
    pub fn new(model: Model) -> Self {
        Self { model }
    }

    /// Parse the input sentence and return a list of semantic chunks
    pub fn parse(&self, sentence: &str) -> Vec<String> {
        if sentence.is_empty() {
            return Vec::new();
        }

        let chars: Vec<char> = sentence.chars().collect();
        let mut chunks = vec![chars[0].to_string()];

        // Calculate base score
        let base_score = -self.calculate_base_score() * 0.5;

        for i in 1..chars.len() {
            let mut score = base_score;

            // UW1: 3 characters before
            if i > 2 {
                score += self.get_feature_score(&self.model.uw1, &chars[i - 3].to_string());
            }

            // UW2: 2 characters before
            if i > 1 {
                score += self.get_feature_score(&self.model.uw2, &chars[i - 2].to_string());
            }

            // UW3: 1 character before
            score += self.get_feature_score(&self.model.uw3, &chars[i - 1].to_string());

            // UW4: current character
            score += self.get_feature_score(&self.model.uw4, &chars[i].to_string());

            // UW5: 1 character after
            if i + 1 < chars.len() {
                score += self.get_feature_score(&self.model.uw5, &chars[i + 1].to_string());
            }

            // UW6: 2 characters after
            if i + 2 < chars.len() {
                score += self.get_feature_score(&self.model.uw6, &chars[i + 2].to_string());
            }

            // BW1: 2 characters before (bigram)
            if i > 1 {
                let bigram = format!("{}{}", chars[i - 2], chars[i - 1]);
                score += self.get_feature_score(&self.model.bw1, &bigram);
            }

            // BW2: 1 character before and current (bigram)
            let bigram = format!("{}{}", chars[i - 1], chars[i]);
            score += self.get_feature_score(&self.model.bw2, &bigram);

            // BW3: current and 1 character after (bigram)
            if i + 1 < chars.len() {
                let bigram = format!("{}{}", chars[i], chars[i + 1]);
                score += self.get_feature_score(&self.model.bw3, &bigram);
            }

            // TW1: 3 characters before (trigram)
            if i > 2 {
                let trigram = format!("{}{}{}", chars[i - 3], chars[i - 2], chars[i - 1]);
                score += self.get_feature_score(&self.model.tw1, &trigram);
            }

            // TW2: 2 characters before and current (trigram)
            if i > 1 {
                let trigram = format!("{}{}{}", chars[i - 2], chars[i - 1], chars[i]);
                score += self.get_feature_score(&self.model.tw2, &trigram);
            }

            // TW3: 1 character before, current, and 1 character after (trigram)
            if i + 1 < chars.len() {
                let trigram = format!("{}{}{}", chars[i - 1], chars[i], chars[i + 1]);
                score += self.get_feature_score(&self.model.tw3, &trigram);
            }

            // TW4: current and 2 characters after (trigram)
            if i + 2 < chars.len() {
                let trigram = format!("{}{}{}", chars[i], chars[i + 1], chars[i + 2]);
                score += self.get_feature_score(&self.model.tw4, &trigram);
            }

            // If score is positive, start a new chunk
            if score > 0.0 {
                chunks.push(chars[i].to_string());
            } else {
                // Otherwise, append to the last chunk
                let last_idx = chunks.len() - 1;
                chunks[last_idx].push(chars[i]);
            }
        }

        chunks
    }

    // Helper method to calculate the base score
    fn calculate_base_score(&self) -> f64 {
        let mut sum = 0;
        sum += self.model.uw1.values().sum::<i32>();
        sum += self.model.uw2.values().sum::<i32>();
        sum += self.model.uw3.values().sum::<i32>();
        sum += self.model.uw4.values().sum::<i32>();
        sum += self.model.uw5.values().sum::<i32>();
        sum += self.model.uw6.values().sum::<i32>();
        sum += self.model.bw1.values().sum::<i32>();
        sum += self.model.bw2.values().sum::<i32>();
        sum += self.model.bw3.values().sum::<i32>();
        sum += self.model.tw1.values().sum::<i32>();
        sum += self.model.tw2.values().sum::<i32>();
        sum += self.model.tw3.values().sum::<i32>();
        sum += self.model.tw4.values().sum::<i32>();
        sum as f64
    }

    // Helper method to get a feature score
    fn get_feature_score(&self, feature: &Feature, key: &str) -> f64 {
        feature.get(key).copied().unwrap_or(0) as f64
    }
}

/// Load a parser with the default Japanese model
pub fn load_default_japanese_parser() -> Parser {
    Parser::new(JAPANESE_MODEL.clone())
}

/// Load a parser from a JSON file
pub fn load_parser_from_file(path: &str) -> Result<Parser> {
    let model_json =
        std::fs::read_to_string(path).map_err(|e| BudouXError::ModelLoadError(e.to_string()))?;

    let model: Model = serde_json::from_str(&model_json)
        .map_err(|e| BudouXError::ModelLoadError(e.to_string()))?;

    Ok(Parser::new(model))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_japanese_parser() {
        let parser = load_default_japanese_parser();
        let result = parser.parse("今日は天気です。");
        assert_eq!(result, vec!["今日は", "天気です。"]);
    }

    #[test]
    fn test_empty_string() {
        let parser = load_default_japanese_parser();
        let result = parser.parse("");
        assert!(result.is_empty());
    }
}
