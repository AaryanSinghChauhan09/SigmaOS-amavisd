# SigmaOS-amavisd Component Wiki

## Overview
SigmaOS-amavisd provides Amavis email content scanner and anti-virus support for the SigmaOS operating system. It demonstrates the email filtering subsystem and content analysis within the SigmaOS ecosystem, following the principles of clear abstraction and deterministic email initialization.

## Principles
- Single Responsibility
- Email Abstraction
- Fail-Fast on Email Initialization Errors
- Deterministic Email Setup
- Clear Abstraction Over Email APIs
- Composition over Inheritance for email extensions

## Architecture
SigmaOS-amavisd follows an email pattern:
- **Email Core**: Provides Amavis content scanning, virus detection, and email analysis
- **Content Analysis**: Supports email content inspection, spam detection, and virus signature scanning
- **Email Filtering**: Supports email filtering, spam classification, and virus quarantine
- **Error Handling**: Fail-fast on invalid email configurations or unsupported email types

## API
The component provides the following public APIs:
- `email::scan::message(message)` - Scan an email message for threats and spam
- `email::virus::detect(attachment)` - Detect viruses in email attachments
- `email::spam::classify(email)` - Classify an email as spam or ham
- `email::status::filter()` - Get email filter status and threat level
- Error types for email failure scenarios or unsupported email types

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific email configuration
2. **Act**: Execute the code under test (message scanning, virus detection)
3. **Assert**: Verify the expected email results

Example:
```rust
#[test]
fn test_email_scan_aaa() {
    // Arrange
    let test_message = Email::new().with_subject("test").with_body("test body");
    
    // Act
    let result = email::scan::message(test_message);
    
    // Assert
    assert!(result.is_ok());
    let scanned = result.unwrap();
    assert!(scanned.is_clean());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Email initialization is best-effort; gracefully degrades on unsupported configurations
- Comprehensive error types for all email failure scenarios
- Unit tests provided for message scanning and virus detection
- Integration tests for full email system with actual email content analysis

## TODO
- Add support for additional threat types (phishing, malware, spam variants)
- Expand content analysis capabilities (header analysis, body analysis, attachment scanning)
- Add filtering features (spam scoring, virus quarantine, email classification)
- Create integration tests with actual email content analysis systems
- Implement email persistence and filter state management