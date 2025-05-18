use scraper::Html;

use crate::error::PaperError;
use crate::html_resource::HTMLResource;
use crate::model::{Account, Notification, NotificationType};
use crate::scrapers::TextProvider;
use reqwest::Client;

use scraper::Selector;

pub struct PublicHamburgAccountScraper {}

// GET https://www.buecherhallen.de/kontostand.html
impl PublicHamburgAccountScraper {
    pub(crate) async fn scrape(&self, client: &Client) -> Result<Account, PaperError> {
        let resource = HTMLResource {
            client: client.to_owned(),
            url: "https://www.buecherhallen.de/kundendaten.html".to_string(),
        };
        let html = resource.load().await?;
        let document = scraper::Html::parse_document(html.as_str());

        self.scrape_document(document).await
    }

    async fn scrape_document(&self, document: Html) -> Result<Account, PaperError> {
        let mut account = Account::new();

        let info_notifications_selector = Selector::parse(r#"div[class="box box-info"]"#)
            .expect("CSS accessors must always be correct");
        let mut infos: Vec<Notification> = document
            .select(&info_notifications_selector)
            .filter_map(|info| {
                return Some(Notification {
                    message: HTMLResource::get_text_from_element(info),
                    notification_type: NotificationType::Info,
                });
            })
            .collect();

        account.notifications.append(&mut infos);

        let error_notifications_selector = Selector::parse(r#"div[class="box box-error"]"#)
            .expect("CSS accessors must always be correct");
        let mut infos: Vec<Notification> = document
            .select(&error_notifications_selector)
            .filter_map(|error| {
                let error_messages_selector =
                    Selector::parse(r#"p"#).expect("CSS accessors must always be correct");
                match error.select(&error_messages_selector).last() {
                    Some(paragraph) => {
                        return Some(Notification {
                            message: HTMLResource::get_text_from_element(paragraph),
                            notification_type: NotificationType::Error,
                        })
                    }
                    None => return None,
                }
            })
            .collect();

        account.notifications.append(&mut infos);

        if let Some(name) = document.get_text(r#".userprops-list>[id=name]>.userprops-value"#) {
            account.name = name;
        }

        if let Some(account_id) =
            document.get_text(r#".userprops-list>[id=kundennummer]>.userprops-value"#)
        {
            account.account_id = account_id;
        }

        // address
        if let Some(address) = document.get_text(r#".userprops-list>[id=adresse]>.userprops-value"#)
        {
            account.address = address;
        }

        if let Some(email) = document.get_text(r#".userprops-list>[id=e-mail]>.userprops-value"#) {
            account.email = email;
        }

        if let Some(phone) = document.get_text(r#".userprops-list>[id=telefon]>.userprops-value"#) {
            account.phone = phone;
        }

        // all charge service items
        let charges_selector =
            Selector::parse(r#"li[class="userprops-item userprops-charges-item"]"#)
                .expect("CSS accessors must always be correct");

        for charge in document.select(&charges_selector) {
            let label_selector = Selector::parse(r#".userprops-label"#)
                .expect("CSS accessors must always be correct");
            let label_ref = charge
                .select(&label_selector)
                .next()
                .ok_or_else(|| PaperError::ParseErrorAccountInfoBalance)?;
            let label = HTMLResource::get_text_from_element(label_ref);

            let value_selector = Selector::parse(r#".userprops-value"#)
                .expect("CSS accessors must always be correct");
            let value_ref = charge
                .select(&value_selector)
                .next()
                .ok_or_else(|| PaperError::ParseErrorAccountInfoBalance)?;
            let value = HTMLResource::get_text_from_element(value_ref);

            account.charge_info.insert(label, value);
        }

        return Ok(account);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::model::NotificationType;

    use super::PublicHamburgAccountScraper;

    #[tokio::test]
    async fn it_scrapes_notifications() {
        let html =
            fs::read_to_string("src/fixtures/hamburg_public/account/balance_multiple_owed.html")
                .expect("Something went wrong reading the file");
        let document = scraper::Html::parse_document(html.as_str());
        let account = PublicHamburgAccountScraper {}
            .scrape_document(document)
            .await
            .unwrap();

        assert_eq!(account.notifications.len(), 2);
        assert_eq!(
            account.notifications[0].notification_type,
            NotificationType::Error
        );
        assert_eq!(
            account.notifications[1].notification_type,
            NotificationType::Error
        );
    }

    #[tokio::test]
    async fn it_scrapes_notifications_when_info() {
        let html =
            fs::read_to_string("src/fixtures/hamburg_public/account/loans_notifications_info.html")
                .expect("Something went wrong reading the file");
        let document = scraper::Html::parse_document(html.as_str());
        let account = PublicHamburgAccountScraper {}
            .scrape_document(document)
            .await
            .unwrap();

        assert_eq!(account.notifications.len(), 2);

        assert_eq!(
            account.notifications[0].notification_type,
            NotificationType::Info
        );
        assert_eq!(
            account.notifications[0].message,
            "Bitte beachten Sie die Anpassung unserer Servicegebühren zum 01.04.2024."
        );

        assert_eq!(
            account.notifications[1].notification_type,
            NotificationType::Error
        );
        assert_eq!(account.notifications[1].message, "Ihr Kundenkonto ist derzeit gesperrt. Bei der Sperre kann es sich z.B. um abgelaufene oder ausstehende Gebühren handeln. Das Verlängern und Vorbestellen von Medien und die Nutzung der eBuecherhalle ist leider nicht möglich. Wenden Sie sich bitte an das Bibliothekspersonal.");
    }
}
