use reqwest::Client;

use super::request;
use super::response;

#[derive(Clone)]
pub struct MailService {
    client: Client,
    mail_service_url: String,
}

impl MailService {
    pub async fn new(mail_service_url: &str) -> Self {
        MailService {
            mail_service_url: String::from(mail_service_url),
            client: Client::new(),
        }
    }

    pub async fn mail(
        &self,
        destination: String,
        title: String,
        content: String,
    ) -> Result<response::Mail, response::MailError> {
        let mnr = request::MailNew {
            destination,
            title,
            content,
        };

        self.client
            .post(format!("{}/mail/new", self.mail_service_url))
            .json(&mnr)
            .send()
            .await
            .map_err(|_| response::MailError::Unknown)?
            .json()
            .await
            .map_err(|_| response::MailError::Unknown)?
    }
}
