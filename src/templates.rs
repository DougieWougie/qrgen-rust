use anyhow::Result;
use std::io::{self, Write};

pub fn apply_template(template_type: &str, data: &str) -> Result<String> {
    match template_type {
        "wifi" => Ok(wifi_template(data)?),
        "vcard" => Ok(vcard_template(data)?),
        "sms" => Ok(sms_template(data)),
        "email" => Ok(email_template(data)),
        "phone" => Ok(phone_template(data)),
        _ => Err(anyhow::anyhow!("Unknown template type: {}", template_type)),
    }
}

fn wifi_template(data: &str) -> Result<String> {
    let parts: Vec<&str> = data.split(',').collect();

    let (ssid, password, encryption) = if parts.len() == 3 {
        (parts[0].to_string(), parts[1].to_string(), parts[2].to_uppercase())
    } else {
        println!("WiFi QR Code Generator");
        print!("Network SSID: ");
        io::stdout().flush()?;
        let mut ssid = String::new();
        io::stdin().read_line(&mut ssid)?;

        print!("Password: ");
        io::stdout().flush()?;
        let mut password = String::new();
        io::stdin().read_line(&mut password)?;

        print!("Encryption (WPA/WEP/nopass): ");
        io::stdout().flush()?;
        let mut encryption = String::new();
        io::stdin().read_line(&mut encryption)?;
        let encryption = encryption.trim().to_uppercase();
        let encryption = if ["WPA", "WEP", "NOPASS"].contains(&encryption.as_str()) {
            encryption
        } else {
            "WPA".to_string()
        };

        (ssid.trim().to_string(), password.trim().to_string(), encryption)
    };

    Ok(format!("WIFI:T:{};S:{};P:{};;", encryption, ssid, password))
}

fn vcard_template(data: &str) -> Result<String> {
    let parts: Vec<&str> = data.split(',').collect();

    let (name, phone, email, org) = if parts.len() >= 2 {
        let name = parts[0].to_string();
        let phone = parts[1].to_string();
        let email = if parts.len() > 2 { parts[2].to_string() } else { String::new() };
        let org = if parts.len() > 3 { parts[3].to_string() } else { String::new() };
        (name, phone, email, org)
    } else {
        println!("vCard QR Code Generator");
        print!("Full Name: ");
        io::stdout().flush()?;
        let mut name = String::new();
        io::stdin().read_line(&mut name)?;

        print!("Phone: ");
        io::stdout().flush()?;
        let mut phone = String::new();
        io::stdin().read_line(&mut phone)?;

        print!("Email: ");
        io::stdout().flush()?;
        let mut email = String::new();
        io::stdin().read_line(&mut email)?;

        print!("Organization (optional): ");
        io::stdout().flush()?;
        let mut org = String::new();
        io::stdin().read_line(&mut org)?;

        (name.trim().to_string(), phone.trim().to_string(), email.trim().to_string(), org.trim().to_string())
    };

    let mut vcard = format!("BEGIN:VCARD\nVERSION:3.0\nFN:{}\n", name);
    if !phone.is_empty() {
        vcard.push_str(&format!("TEL:{}\n", phone));
    }
    if !email.is_empty() {
        vcard.push_str(&format!("EMAIL:{}\n", email));
    }
    if !org.is_empty() {
        vcard.push_str(&format!("ORG:{}\n", org));
    }
    vcard.push_str("END:VCARD");

    Ok(vcard)
}

fn sms_template(data: &str) -> String {
    let parts: Vec<&str> = data.splitn(2, ',').collect();
    if parts.len() == 2 {
        format!("SMSTO:{}:{}", parts[0], parts[1])
    } else {
        format!("SMSTO:{}:", data)
    }
}

fn email_template(data: &str) -> String {
    let parts: Vec<&str> = data.splitn(3, ',').collect();
    let email = parts.get(0).unwrap_or(&"");
    let subject = parts.get(1).unwrap_or(&"");
    let body = parts.get(2).unwrap_or(&"");

    format!("mailto:{}?subject={}&body={}", email, subject, body)
}

fn phone_template(data: &str) -> String {
    format!("tel:{}", data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wifi_template_with_data() {
        let result = wifi_template("MyNetwork,password123,WPA").unwrap();
        assert!(result.contains("WIFI:T:WPA"));
        assert!(result.contains("S:MyNetwork"));
        assert!(result.contains("P:password123"));
        assert!(result.ends_with(";;"));
    }

    #[test]
    fn test_wifi_template_wep_encryption() {
        let result = wifi_template("TestNet,pass456,WEP").unwrap();
        assert!(result.contains("WIFI:T:WEP"));
        assert!(result.contains("S:TestNet"));
        assert!(result.contains("P:pass456"));
    }

    #[test]
    fn test_wifi_template_nopass() {
        let result = wifi_template("OpenNet,,NOPASS").unwrap();
        assert!(result.contains("WIFI:T:NOPASS"));
        assert!(result.contains("S:OpenNet"));
    }

    #[test]
    fn test_vcard_template_full_data() {
        let result = vcard_template("John Doe,+1234567890,john@example.com,Acme Corp").unwrap();
        assert!(result.contains("BEGIN:VCARD"));
        assert!(result.contains("VERSION:3.0"));
        assert!(result.contains("FN:John Doe"));
        assert!(result.contains("TEL:+1234567890"));
        assert!(result.contains("EMAIL:john@example.com"));
        assert!(result.contains("ORG:Acme Corp"));
        assert!(result.contains("END:VCARD"));
    }

    #[test]
    fn test_vcard_template_minimal_data() {
        let result = vcard_template("Jane Smith,+9876543210").unwrap();
        assert!(result.contains("BEGIN:VCARD"));
        assert!(result.contains("FN:Jane Smith"));
        assert!(result.contains("TEL:+9876543210"));
        assert!(result.contains("END:VCARD"));
    }

    #[test]
    fn test_vcard_template_no_organization() {
        let result = vcard_template("Bob Jones,+1111111111,bob@example.com").unwrap();
        assert!(result.contains("FN:Bob Jones"));
        assert!(result.contains("TEL:+1111111111"));
        assert!(result.contains("EMAIL:bob@example.com"));
        // Should not have ORG field with content
        assert!(!result.contains("ORG:") || result.contains("ORG:\n"));
    }

    #[test]
    fn test_sms_template_with_message() {
        let result = sms_template("1234567890,Hello there!");
        assert_eq!(result, "SMSTO:1234567890:Hello there!");
    }

    #[test]
    fn test_sms_template_phone_only() {
        let result = sms_template("9876543210");
        assert_eq!(result, "SMSTO:9876543210:");
    }

    #[test]
    fn test_sms_template_with_commas_in_message() {
        let result = sms_template("1234567890,Hello, how are you?");
        assert_eq!(result, "SMSTO:1234567890:Hello, how are you?");
    }

    #[test]
    fn test_email_template_full() {
        let result = email_template("contact@example.com,Subject Line,Email body text");
        assert_eq!(result, "mailto:contact@example.com?subject=Subject Line&body=Email body text");
    }

    #[test]
    fn test_email_template_address_only() {
        let result = email_template("test@example.com");
        assert_eq!(result, "mailto:test@example.com?subject=&body=");
    }

    #[test]
    fn test_email_template_with_subject_no_body() {
        let result = email_template("info@example.com,Important");
        assert_eq!(result, "mailto:info@example.com?subject=Important&body=");
    }

    #[test]
    fn test_phone_template() {
        let result = phone_template("+1234567890");
        assert_eq!(result, "tel:+1234567890");
    }

    #[test]
    fn test_phone_template_no_plus() {
        let result = phone_template("9876543210");
        assert_eq!(result, "tel:9876543210");
    }

    #[test]
    fn test_apply_template_wifi() {
        let result = apply_template("wifi", "MyNet,pass,WPA").unwrap();
        assert!(result.contains("WIFI:T:WPA"));
    }

    #[test]
    fn test_apply_template_vcard() {
        let result = apply_template("vcard", "John,123").unwrap();
        assert!(result.contains("FN:John"));
    }

    #[test]
    fn test_apply_template_sms() {
        let result = apply_template("sms", "123,msg").unwrap();
        assert_eq!(result, "SMSTO:123:msg");
    }

    #[test]
    fn test_apply_template_email() {
        let result = apply_template("email", "test@test.com").unwrap();
        assert!(result.starts_with("mailto:"));
    }

    #[test]
    fn test_apply_template_phone() {
        let result = apply_template("phone", "123").unwrap();
        assert_eq!(result, "tel:123");
    }

    #[test]
    fn test_apply_template_invalid() {
        let result = apply_template("invalid", "test data");
        assert!(result.is_err());
    }
}
