// email_service.rs
use crate::errors::ServiceError;
use crate::models::Invitation;
use lettre::{
    message::header, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};

pub fn send_invitation(invitation: &Invitation) -> Result<(), ServiceError> {
    let creds = Credentials::new(
        std::env::var("SMTP_USERNAME")
            .expect("no username? :(")
            .to_string(),
        std::env::var("SMTP_PASSWORD")
            .expect("no password? :(")
            .to_string(),
    );
    let sending_email =
        std::env::var("SENDING_EMAIL_ADDRESS").expect("SENDING_EMAIL_ADDRESS must be set");
    let mailer = SmtpTransport::relay(&std::env::var("SMTP_SERVER").expect("no server? :("))
        .unwrap()
        .credentials(creds)
        .build();

    // recipient from the invitation email
    let recipient = invitation.email.as_str();

    let email_body = format!(
        "Please click on the link below to complete registration. <br/>
         <a href=\"{}/register/{}\">
         {}/register</a> <br>
         your Invitation expires on <strong>{}</strong>",
        std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string()),
        invitation.email,
        std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string()),
        invitation.expires_at.format("%I:%M %p %A, %-d %B, %C%y")
    );

    // complete the email message with details

    let email = Message::builder()
        .from(format!("Harmonious <{}>", sending_email).parse().unwrap())
        .to(format!("Harmonious user<{}>", recipient).parse().unwrap())
        .subject("Welcome to Harmonious! Please finish registering")
        .header(header::ContentType::TEXT_HTML)
        .body(email_body)
        .expect("failed to build email");

    Ok(match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    })
}
