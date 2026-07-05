use qrcode::{QrCode, types::Color};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;
use wasm_bindgen::prelude::*;

pub const MAX_INPUT_LEN: usize = 2048;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum QrGeneratorError {
    #[error("Input cannot be empty.")]
    EmptyInput,

    #[error("Input is too long. Maximum length is {MAX_INPUT_LEN} characters.")]
    InputTooLong,

    #[error("Input must be a valid http or https URL.")]
    InvalidUrl,

    #[error("QR generation failed: {0}")]
    EncodeError(String),

    #[error("Failed to serialise QR data for JavaScript: {0}")]
    SerialisationError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QrModule {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QrMatrix {
    pub size: u32,
    pub modules: Vec<QrModule>,
}

/// Validate and normalise user input.
///
/// NOTE: for this minimal version we only accept http/https URLs.
/// To be broaden later to support mailto:, tel:, plain text, Wi-Fi QR codes, etc.
pub fn validate_input(input: &str) -> Result<String, QrGeneratorError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(QrGeneratorError::EmptyInput);
    }

    if trimmed.chars().count() > MAX_INPUT_LEN {
        return Err(QrGeneratorError::InputTooLong);
    }

    let parsed = Url::parse(trimmed).map_err(|_| QrGeneratorError::InvalidUrl)?;

    match parsed.scheme() {
        "http" | "https" => {}
        _ => return Err(QrGeneratorError::InvalidUrl),
    }

    // This host check does not work at the moment, to be checked later
    // if parsed.host_str().is_none() {
    //     return Err(QrGeneratorError::InvalidUrl);
    // }

    Ok(trimmed.to_owned())
}

/// Generate QR matrix data.
///
/// The returned structure contains:
/// - `size`: width/height of the QR matrix in modules
/// - `modules`: dark module positions only
pub fn generate_matrix(input: &str) -> Result<QrMatrix, QrGeneratorError> {
    let valid_input = validate_input(input)?;

    let code = QrCode::new(valid_input.as_bytes())
        .map_err(|err| QrGeneratorError::EncodeError(err.to_string()))?;

    let size = code.width();
    let mut modules = Vec::new();

    for y in 0..size {
        for x in 0..size {
            if code[(x, y)] == Color::Dark {
                modules.push(QrModule {
                    x: x as u32,
                    y: y as u32,
                });
            }
        }
    }

    Ok(QrMatrix {
        size: size as u32,
        modules,
    })
}

/// WASM export: validate whether the input can be encoded.
#[wasm_bindgen]
pub fn is_valid_qr_input(input: &str) -> bool {
    validate_input(input).is_ok()
}

/// WASM export: generate QR data for JavaScript.
///
/// JavaScript receives:
/// {
///   size: number,
///   modules: [{ x: number, y: number }]
/// }
#[wasm_bindgen]
pub fn generate_qr(input: &str) -> Result<JsValue, JsValue> {
    let matrix = generate_matrix(input).map_err(|err| JsValue::from_str(&err.to_string()))?;

    serde_wasm_bindgen::to_value(&matrix).map_err(|err| {
        JsValue::from_str(&QrGeneratorError::SerialisationError(err.to_string()).to_string())
    })
}
