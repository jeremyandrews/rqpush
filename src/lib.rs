//! RQPush is a library that assists with generating and sending
//! notifications to RQueue:
//! https://github.com/jeremyandrews/rqueue

#![deny(missing_docs)]

/// Use json!() macro for creating JSON key-value pairs.
#[macro_use]
extern crate serde_json;
/// Use #[derive(Serialize)] for converting struct to JSON key-value pairs.
#[macro_use]
extern crate serde_derive;

use std::result::Result;

use handlebars::Handlebars;
use log::{debug, error, trace};
use reqwest::{Error, Response};
use serde_json::Value;
use sha2::{Digest, Sha256};

mod template;
#[cfg(test)]
mod tests;

#[derive(Debug)]
/// An object used to generate notifications.
pub struct Notification {
    /// Name of application generating notification.
    pub app: String,
    /// Optional URL offering more information about application and/or notification.
    pub url: Option<String>,
    /// Optional tagline describing application.
    pub tagline: Option<String>,
    /// Optional categorization for notification.
    pub category: Option<String>,
    /// Language the notification is in.
    pub lang: String,
    /// Title of the notification.
    pub title: String,
    title_template: Option<String>,
    short_text: String,
    short_text_template: Option<String>,
    short_html: Option<String>,
    short_html_template: Option<String>,
    long_text: Option<String>,
    long_text_template: Option<String>,
    long_html: Option<String>,
    long_html_template: Option<String>,
    values: Value,
}

impl Notification {
    /// Initializes a notification with the minimum number of required fields:
    ///  - `app` is the app name
    ///  - `title` is short text for the notification (ie, an email subject)
    ///  - `short_text` is longer text for the notification (ie, an email body)
    pub fn init(app: &str, title: &str, short_text: &str) -> Notification {
        trace!("rqpush init: app({}) title({}) short_text({})", &app, &title, &short_text);
        let default_values = match serde_json::from_str(template::DEFAULT_MAPPING) {
            Ok(v) => v,
            Err(e) => {
                error!("error in init(): {}", e);
                json!(null)
            }
        };
        Notification {
            app: app.to_string(),
            url: None,
            tagline: None,
            category: None,
            lang: default_values["lang"].to_string(),
            title: title.to_string(),
            title_template: None,
            short_text: short_text.to_string(),
            short_text_template: None,
            short_html: None,
            short_html_template: None,
            long_text: None,
            long_text_template: None,
            long_html: None,
            long_html_template: None,
            values: default_values,
        }
    }

    /// Update the notification object, setting the notification app name.
    pub fn set_app(&mut self, app: &str) -> &Notification {
        trace!("rqpush set_app: app({})", &app);
        self.app = app.to_string();
        self.values["app"] = json!(&self.app);
        self
    }

    /// Update the notification object, setting the notification url.
    pub fn set_url(&mut self, url: &str) -> &Notification {
        trace!("rqpush set_url: url({})", &url);
        self.url = Some(url.to_string());
        self.values["url"] = json!(&self.url);
        self
    }

    /// Update the notification object, setting the notification tagline.
    pub fn set_tagline(&mut self, tagline: &str) -> &Notification {
        trace!("rqpush set_tagline: tagline({})", &tagline);
        self.tagline = Some(tagline.to_string());
        self.values["tagline"] = json!(&self.tagline);
        self
    }

    /// Update the notification object, setting the notification category.
    pub fn set_category(&mut self, category: &str) -> &Notification {
        trace!("rqpush set_category: category({})", &category);
        self.category = Some(category.to_string());
        self.values["category"] = json!(&self.category);
        self
    }

    /// Update the notification object, setting the notification language.
    pub fn set_lang(&mut self, lang: &str) -> &Notification {
        trace!("rqpush set_lang: lang({})", &lang);
        self.lang = lang.to_string();
        self.values["lang"] = json!(&self.lang);
        self
    }

    /// Update the notification object, setting the notification title.
    pub fn set_title(&mut self, title: &str) -> &Notification {
        trace!("rqpush set_title: title({})", &title);
        self.title = title.to_string();
        self.values["title"] = json!(&self.title);
        self
    }

    /// Update the notification object, setting the title_template (otherwise will
    /// default to template::DEFAULT_TITLE_TEMPLATE).
    pub fn set_title_template(&mut self, template: String) -> &Notification {
        trace!("rqpush set_title_template: title_template({})", &template);
        self.title_template = Some(template.to_string());
        self
    }

    /// Update the notification object, setting the short_text.
    pub fn set_short_text(&mut self, short_text: &str) -> &Notification {
        trace!("rqpush set_short_text: short_text({})", &short_text);
        self.short_text = short_text.to_string();
        self
    }

    /// Update the notification object, setting the short_text_template (otherwise will
    /// default to template::DEFAULT_TEXT_TEMPLATE).
    pub fn set_short_text_template(&mut self, template: String) -> &Notification {
        trace!("rqpush set_text_template: text_template({})", &template);
        self.short_text_template = Some(template.to_string());
        self
    }

    /// Update the notification object, setting the short_html. This is used when sending
    /// email notifications -- if not set, will be the same as short_text.
    pub fn set_short_html(&mut self, short_html: &str) -> &Notification {
        self.short_html = Some(short_html.to_string());
        trace!("rqpush set_short_html: short_html({})", &short_html);
        self
    }

    /// Update the notification object, setting the long_text -- if not set, will be the
    /// same as short_text.
    pub fn set_long_text(&mut self, long_text: &str) -> &Notification {
        trace!("rqpush set_long_text: long_text({})", &long_text);
        self.long_text = Some(long_text.to_string());
        self
    }

    /// Update the notification object, setting the long_text_template (otherwise will
    /// default to template::DEFAULT_TEXT_TEMPLATE).
    pub fn set_long_text_template(&mut self, template: String) -> &Notification {
        trace!("rqpush set_long_text_template: long_text_template({})", &template);
        self.long_text_template = Some(template.to_string());
        self
    }

    /// Update the notification object, setting the short_html. This is used when sending
    /// email notifications -- if not set, will be the same as short_html.
    pub fn set_long_html(&mut self, long_html: &str) -> &Notification {
        trace!("rqpush set_long_html: long_html({})", &long_html);
        self.long_html = Some(long_html.to_string());
        self
    }

    /// Update the notification object, setting the short_html_template (otherwise will
    /// default to template::DEFAULT_HTML_TEMPLATE).
    pub fn set_short_html_template(&mut self, template: String) -> &Notification {
        trace!("rqpush set_short_html_template: short_html_template({})", &template);
        self.short_html_template = Some(template.to_string());
        self
    }

    /// Update the notification object, setting the long_html_template (otherwise will
    /// default to template::DEFAULT_HTML_TEMPLATE).
    pub fn set_long_html_template(&mut self, template: String) -> &Notification {
        trace!("rqpush set_long_html_template: long_html_template({})", &template);
        self.long_html_template = Some(template.to_string());
        self
    }

    /// Update the notification object, adding a handlebars key->value pair,
    /// for example: {{key}} -> "value" will cause anywhere {{key}} is written
    /// to be replaced with "value".
    pub fn add_value(&mut self, key: String, value: String) -> &Notification {
        trace!("rqpush add_value: key({}) value({})", &key, &value);
        self.values[key] = json!(value);
        self
    }

    /// Update the notification object, adding a handlebars key->value pair,
    /// where the "value" is a serde_json encoded Value. This allows passing in
    /// structured data, necessary for lists.
    pub fn add_serde_json_value(&mut self, key: String, value: Value) -> &Notification {
        trace!("rqpush add_serde_json_value: key({}) value({})", &key, &value);
        self.values[key] = value;
        self
    }

    /// Compiles and sends the notification. Any missing fields are automatically
    /// filled out, a sha256 is calculated (salted with an optional shared secret),
    /// then the notification is sent using Reqwest.
    pub fn send(
        &mut self,
        server: &str,
        priority: u8,
        ttl: u32,
        shared_secret: Option<&str>,
    ) -> Result<Response, Error> {
        trace!("rqpush send: server({}) priority({}) ttl({}) shared_secret({:?})", &server, priority, ttl, &shared_secret);
        // Provide field mappings, ie {{app}} and {{category}}
        if !&self.values["app"].is_null() {
            self.values["app"] = json!(&self.app);
        }
        if !&self.values["url"].is_null() {
            self.values["url"] = json!(&self.url);
        }
        if !&self.values["category"].is_null() {
            self.values["category"] = json!(&self.category);
        }

        // Create the final outbound notification object
        let mut outbound_notification = OutboundNotification::default();
        outbound_notification.app = self.app.clone();
        outbound_notification.lang = self.lang.clone();
        outbound_notification.priority = priority;
        outbound_notification.ttl = ttl;

        // Process title (which may include {{variables}})
        self.title_template = match &self.title_template {
            Some(tt) => Some(tt.to_string()),
            None => Some(template::DEFAULT_TITLE_TEMPLATE.to_string()),
        };
        outbound_notification.title = process_template(
            self.title.clone(),
            self.title_template.clone().unwrap(),
            &mut self.values,
        );
        if !&self.values["title"].is_null() {
            self.values["title"] = json!(&outbound_notification.title);
        }

        // If url isn't set manually, set to empty string
        outbound_notification.url = match &self.url {
            Some(u) => u.to_string(),
            None => "".to_string(),
        };

        // If category isn't set manually, set to empty string
        outbound_notification.category = match &self.category {
            Some(c) => c.to_string(),
            None => "".to_string(),
        };

        // If tagline isn't set manually, set it to app name
        outbound_notification.tagline = match &self.tagline {
            Some(t) => t.to_string(),
            None => self.app.clone(),
        };
        if !&self.values["tagline"].is_null() {
            self.values["tagline"] = json!(&outbound_notification.tagline);
        }

        // Process short_text (which may include {{variables}})
        self.short_text_template = match &self.short_text_template {
            Some(stt) => Some(stt.to_string()),
            None => Some(template::DEFAULT_TEXT_TEMPLATE.to_string()),
        };
        outbound_notification.short_text = process_template(
            self.short_text.clone(),
            self.short_text_template.clone().unwrap(),
            &mut self.values,
        );

        // If custom html isn't provided, use the text version, then process
        outbound_notification.short_html = match &self.short_html {
            Some(sh) => sh.to_string(),
            None => self.short_text.clone(),
        };
        self.short_html_template = match &self.short_html_template {
            Some(sht) => Some(sht.to_string()),
            None => Some(template::DEFAULT_HTML_TEMPLATE.to_string()),
        };
        outbound_notification.short_html = process_template(
            outbound_notification.short_html.clone(),
            self.short_html_template.clone().unwrap(),
            &mut self.values,
        );

        // If custom long text isn't provided, use the short text version
        self.long_text = match &self.long_text {
            Some(lt) => Some(lt.to_string()),
            None => Some(self.short_text.clone()),
        };
        self.long_text_template = match &self.long_text_template {
            Some(ltt) => Some(ltt.to_string()),
            None => Some(template::DEFAULT_TEXT_TEMPLATE.to_string()),
        };
        outbound_notification.long_text = process_template(
            outbound_notification.long_text.clone(),
            self.long_text_template.clone().unwrap(),
            &mut self.values,
        );

        // If custom html isn't provided, use the text version
        outbound_notification.long_html = match &self.long_html {
            Some(lh) => lh.to_string(),
            None => self.long_text.clone().unwrap(),
        };
        self.long_html_template = match &self.long_html_template {
            Some(lht) => Some(lht.to_string()),
            None => Some(template::DEFAULT_HTML_TEMPLATE.to_string()),
        };
        outbound_notification.long_html = process_template(
            outbound_notification.long_html.clone(),
            self.long_html_template.clone().unwrap(),
            &mut self.values,
        );

        let contents = json!(outbound_notification).to_string();
        let sha256 = Some(generate_sha256(&contents, shared_secret));
        debug!("rqpush sending message '{}' with priority of {}, sha256 of {:?} and ttl of {} to {}...", &outbound_notification.title, priority, &sha256, ttl, &server);

        let message = Message {
            sha256: sha256,
            contents: contents,
            priority: Some(priority),
            ttl: Some(outbound_notification.ttl),
        };
        trace!("rqpush send: message({:?})", &message);

        let client = reqwest::Client::new();
        client.post(server).json(&message).send()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
/// The final outbound notification object that is derived from
/// the internal Notification object. This is delivered inside
/// Message.contents.
pub struct OutboundNotification {
    /// The name of the application generating the notification.
    pub app: String,
    /// Optional URL of the application project.
    pub url: String,
    /// Optional tagline describing the application.
    pub tagline: String,
    /// Optional category used to route or filter notifications.
    pub category: String,
    /// Two-letter language code of notification, defaults to "en".
    pub lang: String,
    /// Required title of notification, for example used as an email subject.
    pub title: String,
    /// Required text body of notification.
    pub short_text: String,
    /// Optional HTML-version of body of notification.
    pub short_html: String,
    /// Optional extended text body of notification.
    pub long_text: String,
    /// Optional HTML-version of extended body of notification.
    pub long_html: String,
    /// Optional lifetime in seconds of notficiation.
    pub ttl: u32,
    /// Optional priority of notification, from 0-255, higher value is delivered faster.
    pub priority: u8,
}

#[derive(Debug, Default, Serialize, Deserialize)]
/// The final outbound message that is sent, where "contents" is the OutboundNotification
/// derived from the internal Notification.
pub struct Message {
    /// SHA256 hash of the "contents" String (optionally salted).
    pub sha256: Option<String>,
    /// Contains OutboundNotification struct.
    pub contents: String,
    /// Value from 0-255, higher number is higher priority.
    pub priority: Option<u8>,
    /// How long the notification is valid, in seconds.
    pub ttl: Option<u32>,
}

/// Generate a sha256 of a string, including an optional shared_secret as salt.
pub fn generate_sha256(text: &str, shared_secret: Option<&str>) -> String {
    trace!("rqpush generate_sha256: text({}) shared_secret({:?})", &text, &shared_secret);
    let mut hasher = Sha256::new();
    hasher.input(text.as_bytes());
    let salt = match shared_secret {
        Some(s) => s,
        None => "",
    };
    if salt != "" {
        hasher.input(salt.as_bytes());
    }
    let sha = format!("{:x}", hasher.result());
    trace!("rqpush generate_sha256: sha256({})", &sha);
    sha
}

/// Invokes handlebars to convert {{tokens}} to their values.
fn process_template(notification: String, template: String, values: &mut Value) -> String {
    trace!("rqpush process_template: notification({}) template({}) values({:?})", &notification, &template, &values);
    values["notification"] = json!(notification);
    let handlebars = Handlebars::new();
    match handlebars.render_template(&template, &values) {
        Ok(h) => h,
        Err(e) => {
            error!("error in process_html_template: {}", e);
            "".to_string()
        }
    }
}
