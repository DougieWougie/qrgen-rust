use std::process::Command;
use tempfile::TempDir;

/// Helper function to run the qrgen binary with arguments
fn run_qrgen(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to execute qrgen")
}

#[test]
fn test_basic_qr_generation() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test_qr.png");

    let output = run_qrgen(&["test data", "-o", output_path.to_str().unwrap()]);

    assert!(output.status.success());
    assert!(output_path.exists());
    assert!(output_path.metadata().unwrap().len() > 0);
}

#[test]
fn test_qr_generation_with_url() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("url_qr.png");

    let output = run_qrgen(&["https://example.com", "-o", output_path.to_str().unwrap()]);

    assert!(output.status.success());
    assert!(output_path.exists());
}

#[test]
fn test_custom_size() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("large_qr.png");

    let output = run_qrgen(&["test", "-o", output_path.to_str().unwrap(), "--size", "20"]);

    assert!(output.status.success());
    assert!(output_path.exists());

    // Larger size should produce larger file
    let large_size = output_path.metadata().unwrap().len();

    let output_path_small = temp_dir.path().join("small_qr.png");
    let _ = run_qrgen(&[
        "test",
        "-o",
        output_path_small.to_str().unwrap(),
        "--size",
        "5",
    ]);

    let small_size = output_path_small.metadata().unwrap().len();
    assert!(large_size > small_size);
}

#[test]
fn test_custom_border() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("border_qr.png");

    let output = run_qrgen(&[
        "test",
        "-o",
        output_path.to_str().unwrap(),
        "--border",
        "10",
    ]);

    assert!(output.status.success());
    assert!(output_path.exists());
}

#[test]
fn test_error_correction_levels() {
    let temp_dir = TempDir::new().unwrap();

    for level in ["L", "M", "Q", "H"] {
        let output_path = temp_dir.path().join(format!("qr_{}.png", level));
        let output = run_qrgen(&[
            "test",
            "-o",
            output_path.to_str().unwrap(),
            "--error-correction",
            level,
        ]);

        assert!(output.status.success(), "Failed for error level {}", level);
        assert!(output_path.exists());
    }
}

#[test]
fn test_custom_colors() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("colored_qr.png");

    let output = run_qrgen(&[
        "test",
        "-o",
        output_path.to_str().unwrap(),
        "--fill-color",
        "blue",
        "--back-color",
        "yellow",
    ]);

    assert!(output.status.success());
    assert!(output_path.exists());
}

#[test]
fn test_hex_colors() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("hex_qr.png");

    let output = run_qrgen(&[
        "test",
        "-o",
        output_path.to_str().unwrap(),
        "--fill-color",
        "#FF5733",
        "--back-color",
        "#C70039",
    ]);

    assert!(output.status.success());
    assert!(output_path.exists());
}

#[test]
fn test_terminal_output() {
    let output = run_qrgen(&["test", "--terminal"]);

    assert!(output.status.success());
    // Terminal output should contain box drawing characters
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("█") || stdout.contains("▄") || stdout.contains("▀"));
}

#[test]
fn test_wifi_template() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("wifi_qr.png");

    let output = run_qrgen(&[
        "MyNetwork,password123,WPA",
        "--template",
        "wifi",
        "-o",
        output_path.to_str().unwrap(),
    ]);

    assert!(output.status.success());
    assert!(output_path.exists());
}

#[test]
fn test_vcard_template() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("vcard_qr.png");

    let output = run_qrgen(&[
        "John Doe,+1234567890,john@example.com,Acme Corp",
        "--template",
        "vcard",
        "-o",
        output_path.to_str().unwrap(),
    ]);

    assert!(output.status.success());
    assert!(output_path.exists());
}

#[test]
fn test_sms_template() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("sms_qr.png");

    let output = run_qrgen(&[
        "1234567890,Hello!",
        "--template",
        "sms",
        "-o",
        output_path.to_str().unwrap(),
    ]);

    assert!(output.status.success());
    assert!(output_path.exists());
}

#[test]
fn test_email_template() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("email_qr.png");

    let output = run_qrgen(&[
        "test@example.com,Subject,Body",
        "--template",
        "email",
        "-o",
        output_path.to_str().unwrap(),
    ]);

    assert!(output.status.success());
    assert!(output_path.exists());
}

#[test]
fn test_phone_template() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("phone_qr.png");

    let output = run_qrgen(&[
        "1234567890",
        "--template",
        "phone",
        "-o",
        output_path.to_str().unwrap(),
    ]);

    assert!(output.status.success());
    assert!(output_path.exists());
}

#[test]
fn test_help_output() {
    let output = run_qrgen(&["--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Generate QR codes"));
    assert!(stdout.contains("--output"));
    assert!(stdout.contains("--size"));
    assert!(stdout.contains("--terminal"));
}

#[test]
fn test_version_output() {
    let output = run_qrgen(&["--version"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.1.0"));
}

#[test]
fn test_invalid_color() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("invalid_qr.png");

    let output = run_qrgen(&[
        "test",
        "-o",
        output_path.to_str().unwrap(),
        "--fill-color",
        "notacolor",
    ]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown color") || stderr.contains("Invalid"));
}

#[test]
fn test_empty_data() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("empty_qr.png");

    let output = run_qrgen(&["", "-o", output_path.to_str().unwrap()]);

    assert!(output.status.success());
    assert!(output_path.exists());
}

#[test]
fn test_long_data() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("long_qr.png");
    let long_text = "A".repeat(500);

    let output = run_qrgen(&[&long_text, "-o", output_path.to_str().unwrap()]);

    assert!(output.status.success());
    assert!(output_path.exists());
}

#[test]
fn test_special_characters() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("special_qr.png");

    let output = run_qrgen(&["Hello! @#$%^&*()", "-o", output_path.to_str().unwrap()]);

    assert!(output.status.success());
    assert!(output_path.exists());
}

#[test]
fn test_terminal_and_file_output() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("both_qr.png");

    let output = run_qrgen(&["test", "-o", output_path.to_str().unwrap(), "--terminal"]);

    assert!(output.status.success());
    assert!(output_path.exists());

    // Should have terminal output too
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("█") || stdout.len() > 50);
}

#[test]
fn test_default_output_filename() {
    let temp_dir = TempDir::new().unwrap();
    let default_output = temp_dir.path().join("qr_code.png");

    // Run without -o flag, which should create qr_code.png
    let output = Command::new(env!("CARGO_BIN_EXE_qrgen"))
        .arg("test data")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute qrgen");

    assert!(output.status.success());
    assert!(default_output.exists());
}
