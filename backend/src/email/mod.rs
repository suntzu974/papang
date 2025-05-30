use anyhow::Result;
use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
}

impl EmailService {
    pub fn new(smtp_username: String, smtp_password: String, from_email: String) -> Self {
        let creds = Credentials::new(smtp_username, smtp_password);

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        Self { mailer, from_email }
    }

    pub async fn send_verification_email(&self, to_email: &str, verification_token: &str) -> Result<()> {
        let verification_url = format!("http://localhost:3001/auth/verify-email?token={}", verification_token);
        
        let email = Message::builder()
            .from(self.from_email.parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject("Vérifiez votre adresse email - Papang")
            .header(ContentType::TEXT_HTML)
            .body(format!(
                r#"
                <html>
                <body>
                    <h2>Bienvenue sur Papang!</h2>
                    <p>Merci de vous être inscrit. Veuillez cliquer sur le lien ci-dessous pour vérifier votre adresse email:</p>
                    <a href="{}" style="background-color: #007bff; color: white; padding: 10px 20px; text-decoration: none; border-radius: 5px;">
                        Vérifier mon email
                    </a>
                    <p>Ou copiez ce lien dans votre navigateur: {}</p>
                    <p>Ce lien expirera dans 24 heures.</p>
                </body>
                </html>
                "#,
                verification_url, verification_url
            ))
            .unwrap();

        self.mailer.send(email).await?;
        Ok(())
    }

    pub async fn send_password_reset_email(
        &self,
        to_email: &str,
        reset_token: &str,
    ) -> Result<(), anyhow::Error> {
        let reset_url = format!("http://localhost:3001/auth/reset-password?token={}", reset_token);
        
        let subject = "Demande de réinitialisation de mot de passe - Papang";
        let body = format!(
            r#"
            <html>
            <body>
                <h2>Réinitialisez votre mot de passe</h2>
                <p>Vous avez demandé une réinitialisation de mot de passe. Cliquez sur le lien ci-dessous pour réinitialiser votre mot de passe:</p>
                <a href="{}" style="background-color: #007bff; color: white; padding: 10px 20px; text-decoration: none; border-radius: 5px;">
                    Réinitialiser mon mot de passe
                </a>
                <p>Ou copiez ce lien dans votre navigateur: {}</p>
                <p>Ce lien expirera dans 1 heure.</p>
                <p>Si vous n'avez pas demandé cette réinitialisation de mot de passe, veuillez ignorer cet email.</p>
            </body>
            </html>
            "#,
            reset_url, reset_url
        );

        let email = Message::builder()
            .from(self.from_email.parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body)
            .unwrap();

        self.mailer.send(email).await?;
        Ok(())
    }
}
