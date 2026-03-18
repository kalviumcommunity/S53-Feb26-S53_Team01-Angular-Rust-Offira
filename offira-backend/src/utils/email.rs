use reqwest::Client;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct Sender {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct To {
    email: String,
}

#[derive(Serialize)]
struct EmailRequest {
    sender: Sender,
    to: Vec<To>,
    subject: String,

    #[serde(rename = "htmlContent")]
    html_content: String,
}

pub async fn send_email(to_email: &str, subject: &str, html_content: &str) -> Result<(), String> {
    let api_key = env::var("BREVO_API_KEY").map_err(|_| "Missing API key")?;
    let sender_email = env::var("SENDER_EMAIL").map_err(|_| "Missing sender email")?;
    let sender_name = env::var("SENDER_NAME").map_err(|_| "Missing sender name")?;

    let client = Client::new();

    let body = EmailRequest {
        sender: Sender {
            name: sender_name,
            email: sender_email,
        },
        to: vec![To {
            email: to_email.to_string(),
        }],
        subject: subject.to_string(),
        html_content: html_content.to_string(),
    };

    let res = client
        .post("https://api.brevo.com/v3/smtp/email")
        .header("api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|_| "Failed to call Brevo")?;

    if !res.status().is_success() {
        let err = res.text().await.unwrap_or_default();
        return Err(format!("Brevo error: {}", err));
    }

    Ok(())
}

pub fn build_invite_email(invite_link: &str) -> String {
    format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        </head>

        <body style="margin:0; padding:0; background-color:#f4f6f8; font-family:Arial, sans-serif;">

        <table width="100%" cellpadding="0" cellspacing="0" style="background-color:#f4f6f8; padding:20px 0;">
        <tr>
        <td align="center">

        <table width="600" cellpadding="0" cellspacing="0" style="background:#ffffff; border-radius:10px; overflow:hidden; box-shadow:0 4px 12px rgba(0,0,0,0.05);">

        <!-- HEADER -->
        <tr>
        <td style="background:linear-gradient(135deg,#4CAF50,#2e7d32); padding:20px; text-align:center;">
        <h1 style="color:white; margin:0;">You're Invited 🎉</h1>
        </td>
        </tr>

        <!-- BODY -->
        <tr>
        <td style="padding:30px; color:#333333;">

        <p style="font-size:16px;">Hello,</p>

        <p style="font-size:15px; line-height:1.6;">
        You’ve been invited to join our platform. Click the button below to accept your invitation and set up your account.
        </p>

        <!-- BUTTON -->
        <div style="text-align:center; margin:30px 0;">
        <a href="{}"
        style="
        background:linear-gradient(135deg,#4CAF50,#2e7d32);
        color:white;
        padding:14px 24px;
        font-size:16px;
        text-decoration:none;
        border-radius:6px;
        display:inline-block;
        font-weight:bold;
        ">
        Accept Invitation
        </a>
        </div>

        <p style="font-size:14px; color:#555;">
        If the button doesn’t work, copy and paste this link into your browser:
        </p>

        <p style="font-size:13px; word-break:break-all; color:#2e7d32;">
        {}
        </p>

        <p style="font-size:13px; color:#999; margin-top:25px;">
        This invite will expire in 24 hours for security reasons.
        </p>

        </td>
        </tr>

        <!-- FOOTER -->
        <tr>
        <td style="background:#f1f1f1; text-align:center; padding:15px; font-size:12px; color:#888;">
        © 2026 YourApp. All rights reserved.
        </td>
        </tr>

        </table>

        </td>
        </tr>
        </table>

        </body>
        </html>
        "#,
        invite_link, invite_link
    )
}
