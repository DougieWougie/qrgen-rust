# Testing Documentation

## Test Suite Overview

The qrgen project includes comprehensive test coverage with both unit tests and integration tests.

### Test Statistics

- **Total Tests**: 68
  - Unit Tests: 47
  - Integration Tests: 21
- **Success Rate**: 100%

## Running Tests

### Run All Tests

```bash
cargo test
```

### Run Tests in Release Mode (Faster)

```bash
cargo test --release
```

### Run Specific Test

```bash
# Run a specific unit test
cargo test test_parse_color_hex_lowercase

# Run a specific integration test
cargo test test_basic_qr_generation
```

### Run Tests with Output

```bash
cargo test -- --nocapture
```

### Run Tests in Parallel

```bash
cargo test -- --test-threads=4
```

## Unit Tests

Unit tests are located inline with the source code using Rust's `#[cfg(test)]` attribute.

### Main Module Tests (src/main.rs)

#### Error Correction Parsing Tests
- ✅ `test_parse_error_correction_l` - Tests L level
- ✅ `test_parse_error_correction_m` - Tests M level
- ✅ `test_parse_error_correction_q` - Tests Q level
- ✅ `test_parse_error_correction_h` - Tests H level
- ✅ `test_parse_error_correction_lowercase` - Tests case insensitivity
- ✅ `test_parse_error_correction_invalid` - Tests error handling

#### Color Parsing Tests
- ✅ `test_parse_color_black` - Named color: black
- ✅ `test_parse_color_white` - Named color: white
- ✅ `test_parse_color_red` - Named color: red
- ✅ `test_parse_color_blue` - Named color: blue
- ✅ `test_parse_color_darkblue` - Named color: darkblue
- ✅ `test_parse_color_hex_lowercase` - Hex color: #ff5733
- ✅ `test_parse_color_hex_uppercase` - Hex color: #FF5733
- ✅ `test_parse_color_hex_mixed_case` - Hex color: #FfA500
- ✅ `test_parse_color_invalid` - Invalid color name
- ✅ `test_parse_color_invalid_hex` - Invalid hex format
- ✅ `test_parse_color_invalid_hex_length` - Wrong hex length

#### QR Code Generation Tests
- ✅ `test_create_qr_code_basic` - Basic QR generation
- ✅ `test_create_qr_code_url` - QR code with URL
- ✅ `test_create_qr_code_custom_size` - Custom box size
- ✅ `test_create_qr_code_custom_border` - Custom border size
- ✅ `test_create_qr_code_all_error_levels` - All error correction levels
- ✅ `test_create_qr_code_custom_colors` - Custom fill and background colors
- ✅ `test_create_qr_code_empty_string` - Empty data
- ✅ `test_create_qr_code_long_text` - Very long text (1000 chars)
- ✅ `test_create_qr_code_special_characters` - Unicode and special chars
- ✅ `test_print_terminal` - Terminal ASCII output

### Template Module Tests (src/templates.rs)

#### WiFi Template Tests
- ✅ `test_wifi_template_with_data` - WPA encryption
- ✅ `test_wifi_template_wep_encryption` - WEP encryption
- ✅ `test_wifi_template_nopass` - No password

#### vCard Template Tests
- ✅ `test_vcard_template_full_data` - All fields
- ✅ `test_vcard_template_minimal_data` - Name and phone only
- ✅ `test_vcard_template_no_organization` - Without organization

#### SMS Template Tests
- ✅ `test_sms_template_with_message` - Phone and message
- ✅ `test_sms_template_phone_only` - Phone only
- ✅ `test_sms_template_with_commas_in_message` - Commas in message

#### Email Template Tests
- ✅ `test_email_template_full` - Email, subject, and body
- ✅ `test_email_template_address_only` - Email address only
- ✅ `test_email_template_with_subject_no_body` - Email and subject

#### Phone Template Tests
- ✅ `test_phone_template` - With country code
- ✅ `test_phone_template_no_plus` - Without country code

#### Template Application Tests
- ✅ `test_apply_template_wifi` - Apply WiFi template
- ✅ `test_apply_template_vcard` - Apply vCard template
- ✅ `test_apply_template_sms` - Apply SMS template
- ✅ `test_apply_template_email` - Apply email template
- ✅ `test_apply_template_phone` - Apply phone template
- ✅ `test_apply_template_invalid` - Invalid template error

## Integration Tests

Integration tests are located in `tests/integration_tests.rs` and test the complete CLI application.

### Basic Functionality
- ✅ `test_basic_qr_generation` - Generate QR code with output file
- ✅ `test_qr_generation_with_url` - Generate QR with URL data
- ✅ `test_default_output_filename` - Default qr_code.png output

### Customization Tests
- ✅ `test_custom_size` - Custom box size
- ✅ `test_custom_border` - Custom border size
- ✅ `test_custom_colors` - Named colors
- ✅ `test_hex_colors` - Hex color codes
- ✅ `test_error_correction_levels` - All L/M/Q/H levels

### Output Modes
- ✅ `test_terminal_output` - Terminal ASCII display
- ✅ `test_terminal_and_file_output` - Both terminal and file

### Template Integration Tests
- ✅ `test_wifi_template` - WiFi QR code
- ✅ `test_vcard_template` - vCard contact
- ✅ `test_sms_template` - SMS message
- ✅ `test_email_template` - Email link
- ✅ `test_phone_template` - Phone number

### CLI Tests
- ✅ `test_help_output` - Help message display
- ✅ `test_version_output` - Version display

### Edge Cases
- ✅ `test_invalid_color` - Invalid color error handling
- ✅ `test_empty_data` - Empty string input
- ✅ `test_long_data` - Very long data (500 chars)
- ✅ `test_special_characters` - Special characters

## Test Coverage

### Core Functionality Coverage
- ✅ QR code generation (all error levels)
- ✅ Color parsing (named and hex)
- ✅ Image creation and saving
- ✅ Terminal display
- ✅ Error handling

### Template System Coverage
- ✅ WiFi QR codes
- ✅ vCard contacts
- ✅ SMS messages
- ✅ Email links
- ✅ Phone numbers
- ✅ Template error handling

### CLI Coverage
- ✅ Argument parsing
- ✅ Help and version display
- ✅ Output file handling
- ✅ Error messages
- ✅ Default behaviors

### Edge Cases Coverage
- ✅ Empty input
- ✅ Very long input
- ✅ Special characters
- ✅ Unicode support
- ✅ Invalid parameters

## Comparison with Python Tests

The Rust test suite provides equivalent coverage to the Python version:

| Test Category | Python | Rust |
|---------------|--------|------|
| Core QR Generation | ✅ | ✅ |
| Error Correction | ✅ | ✅ |
| Color Parsing | ✅ | ✅ |
| Template WiFi | ✅ | ✅ |
| Template vCard | ✅ | ✅ |
| Template SMS | ✅ | ✅ |
| Template Email | ✅ | ✅ |
| Template Phone | ✅ | ✅ |
| CLI Integration | ✅ | ✅ |
| Edge Cases | ✅ | ✅ |

## Performance

Test execution times:

- **Unit Tests**: ~0.01s (47 tests)
- **Integration Tests**: ~1.3s (21 tests)
- **Total**: ~1.3s (68 tests)

The Rust tests run significantly faster than the Python equivalents due to:
- Compiled binary (no interpreter startup)
- Efficient memory management
- Native execution

## Continuous Integration

To run tests in CI/CD pipelines:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Run tests
cargo test --release

# Generate test report
cargo test --release -- --format json > test-results.json
```

## Adding New Tests

### Adding Unit Tests

Add test functions in the same file as the code being tested:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        assert_eq!(my_function("input"), "expected");
    }
}
```

### Adding Integration Tests

Add test functions in `tests/integration_tests.rs`:

```rust
#[test]
fn test_my_feature() {
    let output = run_qrgen(&["arg1", "arg2"]);
    assert!(output.status.success());
}
```

## Test Best Practices

1. **Fast Tests**: Keep unit tests fast (< 1ms each)
2. **Isolated**: Each test should be independent
3. **Clear Names**: Test names should describe what they test
4. **Edge Cases**: Test boundary conditions and error cases
5. **Assertions**: Use appropriate assertion macros
6. **Cleanup**: Integration tests use temp directories for automatic cleanup

## Known Limitations

- Interactive template tests (stdin input) are not automated
- Logo embedding tests require valid image files
- Some integration tests depend on cargo build artifacts

## Future Test Improvements

- [ ] Add property-based testing with `proptest`
- [ ] Add benchmark tests with `criterion`
- [ ] Increase code coverage to 95%+
- [ ] Add mutation testing
- [ ] Add visual QR code validation tests
