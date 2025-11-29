use anyhow::{Context, Result};
use clap::Parser;
use image::{Rgba, RgbaImage};
use qrcode::EcLevel;
use qrcode::{Color, QrCode};
use std::path::PathBuf;

mod templates;
use templates::apply_template;

#[derive(Parser, Debug)]
#[command(
    name = "qrgen",
    version = "0.1.0",
    author = "Dougie Richardson",
    about = "Generate QR codes from the command line",
    long_about = None
)]
#[command(after_help = "Examples:
  qrgen \"https://example.com\"
  qrgen \"Hello World\" -o qr.png
  qrgen \"https://github.com\" --terminal
  qrgen \"Contact: john@example.com\" -o contact.png --size 15")]
struct Cli {
    /// The data to encode in the QR code (text, URL, etc.)
    data: String,

    /// Output file path (PNG format). Default: qr_code.png
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Size of each box in pixels
    #[arg(short, long, default_value = "10")]
    size: u32,

    /// Border size in boxes
    #[arg(short, long, default_value = "4")]
    border: u32,

    /// Error correction level: L(7%), M(15%), Q(25%), H(30%)
    #[arg(short, long, value_parser = parse_error_correction, default_value = "M")]
    error_correction: EcLevel,

    /// Display QR code in terminal using ASCII characters
    #[arg(short, long)]
    terminal: bool,

    /// Fill color for QR code modules (default: black)
    #[arg(long, default_value = "black")]
    fill_color: String,

    /// Background color for QR code (default: white)
    #[arg(long, default_value = "white")]
    back_color: String,

    /// Path to logo image to embed in center of QR code
    #[arg(long)]
    logo: Option<PathBuf>,

    /// Use a template for specific content types
    #[arg(long, value_parser = ["wifi", "vcard", "sms", "email", "phone"])]
    template: Option<String>,
}

fn parse_error_correction(s: &str) -> Result<EcLevel> {
    match s.to_uppercase().as_str() {
        "L" => Ok(EcLevel::L),
        "M" => Ok(EcLevel::M),
        "Q" => Ok(EcLevel::Q),
        "H" => Ok(EcLevel::H),
        _ => Err(anyhow::anyhow!("Invalid error correction level")),
    }
}

fn parse_color(color_str: &str) -> Result<Rgba<u8>> {
    // Handle hex colors
    if color_str.starts_with('#') {
        let hex = color_str.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16)?;
            let g = u8::from_str_radix(&hex[2..4], 16)?;
            let b = u8::from_str_radix(&hex[4..6], 16)?;
            return Ok(Rgba([r, g, b, 255]));
        }
    }

    // Handle named colors
    match color_str.to_lowercase().as_str() {
        "black" => Ok(Rgba([0, 0, 0, 255])),
        "white" => Ok(Rgba([255, 255, 255, 255])),
        "red" => Ok(Rgba([255, 0, 0, 255])),
        "green" => Ok(Rgba([0, 255, 0, 255])),
        "blue" => Ok(Rgba([0, 0, 255, 255])),
        "yellow" => Ok(Rgba([255, 255, 0, 255])),
        "cyan" => Ok(Rgba([0, 255, 255, 255])),
        "magenta" => Ok(Rgba([255, 0, 255, 255])),
        "darkblue" => Ok(Rgba([0, 0, 139, 255])),
        "darkgreen" => Ok(Rgba([0, 100, 0, 255])),
        "lightgray" => Ok(Rgba([211, 211, 211, 255])),
        "lightyellow" => Ok(Rgba([255, 255, 224, 255])),
        _ => Err(anyhow::anyhow!("Unknown color: {}", color_str)),
    }
}

fn create_qr_code(
    data: &str,
    error_correction: EcLevel,
    size: u32,
    border: u32,
    fill_color: Rgba<u8>,
    back_color: Rgba<u8>,
) -> Result<RgbaImage> {
    let code = QrCode::with_error_correction_level(data, error_correction)
        .context("Failed to generate QR code")?;

    let qr_width = code.width() as u32;
    let img_size = (qr_width + 2 * border) * size;

    let mut img = RgbaImage::from_pixel(img_size, img_size, back_color);

    for y in 0..qr_width {
        for x in 0..qr_width {
            if code[(x as usize, y as usize)] == Color::Dark {
                let px = (x + border) * size;
                let py = (y + border) * size;

                for dy in 0..size {
                    for dx in 0..size {
                        img.put_pixel(px + dx, py + dy, fill_color);
                    }
                }
            }
        }
    }

    Ok(img)
}

fn embed_logo(mut qr_img: RgbaImage, logo_path: &PathBuf) -> Result<RgbaImage> {
    let logo = image::open(logo_path)
        .context("Failed to open logo image")?
        .to_rgba8();

    let qr_width = qr_img.width();
    let qr_height = qr_img.height();

    // Logo should be about 1/5 of QR code size
    let logo_size = qr_width.min(qr_height) / 5;

    // Resize logo maintaining aspect ratio
    let resized_logo = image::imageops::resize(
        &logo,
        logo_size,
        logo_size,
        image::imageops::FilterType::Lanczos3,
    );

    // Create white background for logo
    let logo_bg_size = (logo_size as f32 * 1.2) as u32;
    let mut logo_bg = RgbaImage::from_pixel(logo_bg_size, logo_bg_size, Rgba([255, 255, 255, 255]));

    // Paste logo onto white background
    let logo_x = (logo_bg_size - resized_logo.width()) / 2;
    let logo_y = (logo_bg_size - resized_logo.height()) / 2;
    image::imageops::overlay(&mut logo_bg, &resized_logo, logo_x as i64, logo_y as i64);

    // Paste logo background onto QR code
    let qr_logo_x = (qr_width - logo_bg_size) / 2;
    let qr_logo_y = (qr_height - logo_bg_size) / 2;
    image::imageops::overlay(&mut qr_img, &logo_bg, qr_logo_x as i64, qr_logo_y as i64);

    Ok(qr_img)
}

fn print_terminal(data: &str, error_correction: EcLevel) -> Result<()> {
    let code = QrCode::with_error_correction_level(data, error_correction)
        .context("Failed to generate QR code for terminal display")?;

    // Use Unicode block elements for better terminal display
    let qr_string = code
        .render::<char>()
        .quiet_zone(true)
        .module_dimensions(2, 1)
        .build();

    println!("{}", qr_string);
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Apply template if specified
    let data = if let Some(template) = &cli.template {
        apply_template(template, &cli.data)?
    } else {
        cli.data.clone()
    };

    // Determine output path
    let output_path = if !cli.terminal && cli.output.is_none() {
        Some(PathBuf::from("qr_code.png"))
    } else {
        cli.output.clone()
    };

    // Parse colors
    let fill_color = parse_color(&cli.fill_color)
        .with_context(|| format!("Invalid fill color: {}", cli.fill_color))?;
    let back_color = parse_color(&cli.back_color)
        .with_context(|| format!("Invalid back color: {}", cli.back_color))?;

    // Display in terminal if requested
    if cli.terminal {
        print_terminal(&data, cli.error_correction)?;
    }

    // Save to file if output path provided
    if let Some(output) = output_path {
        let mut img = create_qr_code(
            &data,
            cli.error_correction,
            cli.size,
            cli.border,
            fill_color,
            back_color,
        )?;

        // Embed logo if provided
        if let Some(logo_path) = &cli.logo {
            img = embed_logo(img, logo_path)?;
        }

        img.save(&output)
            .with_context(|| format!("Failed to save QR code to {:?}", output))?;
        println!("QR code saved to: {}", output.display());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_correction_l() {
        let result = parse_error_correction("L").unwrap();
        assert_eq!(result, EcLevel::L);
    }

    #[test]
    fn test_parse_error_correction_m() {
        let result = parse_error_correction("M").unwrap();
        assert_eq!(result, EcLevel::M);
    }

    #[test]
    fn test_parse_error_correction_q() {
        let result = parse_error_correction("Q").unwrap();
        assert_eq!(result, EcLevel::Q);
    }

    #[test]
    fn test_parse_error_correction_h() {
        let result = parse_error_correction("H").unwrap();
        assert_eq!(result, EcLevel::H);
    }

    #[test]
    fn test_parse_error_correction_lowercase() {
        let result = parse_error_correction("l").unwrap();
        assert_eq!(result, EcLevel::L);
    }

    #[test]
    fn test_parse_error_correction_invalid() {
        let result = parse_error_correction("X");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_color_black() {
        let color = parse_color("black").unwrap();
        assert_eq!(color, Rgba([0, 0, 0, 255]));
    }

    #[test]
    fn test_parse_color_white() {
        let color = parse_color("white").unwrap();
        assert_eq!(color, Rgba([255, 255, 255, 255]));
    }

    #[test]
    fn test_parse_color_red() {
        let color = parse_color("red").unwrap();
        assert_eq!(color, Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_parse_color_blue() {
        let color = parse_color("blue").unwrap();
        assert_eq!(color, Rgba([0, 0, 255, 255]));
    }

    #[test]
    fn test_parse_color_hex_lowercase() {
        let color = parse_color("#ff5733").unwrap();
        assert_eq!(color, Rgba([255, 87, 51, 255]));
    }

    #[test]
    fn test_parse_color_hex_uppercase() {
        let color = parse_color("#FF5733").unwrap();
        assert_eq!(color, Rgba([255, 87, 51, 255]));
    }

    #[test]
    fn test_parse_color_hex_mixed_case() {
        let color = parse_color("#FfA500").unwrap();
        assert_eq!(color, Rgba([255, 165, 0, 255]));
    }

    #[test]
    fn test_parse_color_darkblue() {
        let color = parse_color("darkblue").unwrap();
        assert_eq!(color, Rgba([0, 0, 139, 255]));
    }

    #[test]
    fn test_parse_color_invalid() {
        let result = parse_color("notacolor");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_color_invalid_hex() {
        let result = parse_color("#GGGGGG");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_color_invalid_hex_length() {
        let result = parse_color("#FFF");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_qr_code_basic() {
        let img = create_qr_code(
            "test",
            EcLevel::M,
            10,
            4,
            Rgba([0, 0, 0, 255]),
            Rgba([255, 255, 255, 255]),
        );
        assert!(img.is_ok());
        let img = img.unwrap();
        assert!(img.width() > 0);
        assert!(img.height() > 0);
    }

    #[test]
    fn test_create_qr_code_url() {
        let img = create_qr_code(
            "https://example.com",
            EcLevel::M,
            10,
            4,
            Rgba([0, 0, 0, 255]),
            Rgba([255, 255, 255, 255]),
        );
        assert!(img.is_ok());
    }

    #[test]
    fn test_create_qr_code_custom_size() {
        let img = create_qr_code(
            "test",
            EcLevel::M,
            20,
            4,
            Rgba([0, 0, 0, 255]),
            Rgba([255, 255, 255, 255]),
        )
        .unwrap();

        // Larger size should produce larger image
        let img_small = create_qr_code(
            "test",
            EcLevel::M,
            10,
            4,
            Rgba([0, 0, 0, 255]),
            Rgba([255, 255, 255, 255]),
        )
        .unwrap();

        assert!(img.width() > img_small.width());
    }

    #[test]
    fn test_create_qr_code_custom_border() {
        let img = create_qr_code(
            "test",
            EcLevel::M,
            10,
            10,
            Rgba([0, 0, 0, 255]),
            Rgba([255, 255, 255, 255]),
        )
        .unwrap();

        let img_small_border = create_qr_code(
            "test",
            EcLevel::M,
            10,
            4,
            Rgba([0, 0, 0, 255]),
            Rgba([255, 255, 255, 255]),
        )
        .unwrap();

        assert!(img.width() > img_small_border.width());
    }

    #[test]
    fn test_create_qr_code_all_error_levels() {
        for level in [EcLevel::L, EcLevel::M, EcLevel::Q, EcLevel::H] {
            let img = create_qr_code(
                "test",
                level,
                10,
                4,
                Rgba([0, 0, 0, 255]),
                Rgba([255, 255, 255, 255]),
            );
            assert!(img.is_ok());
        }
    }

    #[test]
    fn test_create_qr_code_custom_colors() {
        let img = create_qr_code(
            "test",
            EcLevel::M,
            10,
            4,
            Rgba([0, 0, 255, 255]),   // Blue
            Rgba([255, 255, 0, 255]), // Yellow
        );
        assert!(img.is_ok());
    }

    #[test]
    fn test_create_qr_code_empty_string() {
        let img = create_qr_code(
            "",
            EcLevel::M,
            10,
            4,
            Rgba([0, 0, 0, 255]),
            Rgba([255, 255, 255, 255]),
        );
        assert!(img.is_ok());
    }

    #[test]
    fn test_create_qr_code_long_text() {
        let long_text = "A".repeat(1000);
        let img = create_qr_code(
            &long_text,
            EcLevel::M,
            10,
            4,
            Rgba([0, 0, 0, 255]),
            Rgba([255, 255, 255, 255]),
        );
        assert!(img.is_ok());
    }

    #[test]
    fn test_create_qr_code_special_characters() {
        let img = create_qr_code(
            "Hello! @#$%^&*() 你好 🎉",
            EcLevel::M,
            10,
            4,
            Rgba([0, 0, 0, 255]),
            Rgba([255, 255, 255, 255]),
        );
        assert!(img.is_ok());
    }

    #[test]
    fn test_print_terminal() {
        // Just verify it doesn't panic or error
        let result = print_terminal("test", EcLevel::M);
        assert!(result.is_ok());
    }
}
