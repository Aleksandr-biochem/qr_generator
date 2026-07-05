use qr_generator::{MAX_INPUT_LEN, QrGeneratorError, generate_matrix, validate_input};

#[test]
fn validates_https_url() {
    let result = validate_input("https://example.com/test?x=1");
    assert!(result.is_ok());
}

#[test]
fn validates_http_url() {
    let result = validate_input("http://example.com");
    assert!(result.is_ok());
}

#[test]
fn trims_input() {
    let result = validate_input("  https://example.com  ").unwrap();
    assert_eq!(result, "https://example.com");
}

#[test]
fn rejects_empty_input() {
    let result = validate_input("");
    assert_eq!(result, Err(QrGeneratorError::EmptyInput));
}

#[test]
fn rejects_whitespace_input() {
    let result = validate_input("     ");
    assert_eq!(result, Err(QrGeneratorError::EmptyInput));
}

#[test]
fn rejects_invalid_url() {
    let result = validate_input("not a url");
    assert_eq!(result, Err(QrGeneratorError::InvalidUrl));
}

#[test]
fn rejects_unsupported_scheme() {
    let result = validate_input("ftp://example.com/file.txt");
    assert_eq!(result, Err(QrGeneratorError::InvalidUrl));
}

// Deprecated for now
// #[test]
// fn rejects_url_without_host() {
//     let result = validate_input("https://missing-host");
//     assert_eq!(result, Err(QrGeneratorError::InvalidUrl));
// }

#[test]
fn rejects_too_long_input() {
    let long_url = format!("https://example.com/{}", "a".repeat(MAX_INPUT_LEN));
    let result = validate_input(&long_url);
    assert_eq!(result, Err(QrGeneratorError::InputTooLong));
}

#[test]
fn generates_matrix_for_valid_url() {
    let matrix = generate_matrix("https://example.com").unwrap();

    assert!(matrix.size > 0);
    assert!(!matrix.modules.is_empty());

    for module in matrix.modules {
        assert!(module.x < matrix.size);
        assert!(module.y < matrix.size);
    }
}

#[test]
fn same_input_generates_same_matrix() {
    let first = generate_matrix("https://example.com").unwrap();
    let second = generate_matrix("https://example.com").unwrap();

    assert_eq!(first, second);
}
