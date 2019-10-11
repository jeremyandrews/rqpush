#[macro_use] extern crate serde_json;

use serde_json::Value;
use handlebars::Handlebars;

mod template;

#[derive(Debug)]
pub struct Notification {
    app: String,
    url: Option<String>,
    tagline: Option<String>,
    category: Option<String>,
    lang: String,
    title: String,
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
    // Create a notification with the minumum required number of fields: 
    //  - `app` is the app name
    //  - `title` is short text for the notification (ie, an email subject)
    //  - `short_text` is longer text for the notification (ie, an email body)
    pub fn init(app: &str, title: &str, short_text: &str) -> Notification {
        let default_values = match serde_json::from_str(template::DEFAULT_MAPPING) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("error in init(): {}", e);
                json!(null)
            }
        };
        Notification{
            app: app.to_string(),
            url: None,
            tagline: None,
            category: None,
            lang: default_values["lang"].to_string(),
            title: title.to_string(),
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

    pub fn set_app(&mut self, app: &str) -> &Notification {
        self.app = app.to_string();
        self.values["app"] = json!(&self.app);
        self
    }

    pub fn set_url(&mut self, url: &str) -> &Notification {
        self.url = Some(url.to_string());
        self.values["url"] = json!(&self.url);
        self
    }

    pub fn set_tagline(&mut self, tagline: &str) -> &Notification {
        self.tagline = Some(tagline.to_string());
        self.values["tagline"] = json!(&self.tagline);
        self
    }

    pub fn set_category(&mut self, category: &str) -> &Notification {
        self.category = Some(category.to_string());
        self.values["category"] = json!(&self.category);
        self
    }

    pub fn set_lang(&mut self, lang: &str) -> &Notification {
        self.lang = lang.to_string();
        self.values["lang"] = json!(&self.lang);
        self
    }

    pub fn set_title(&mut self, title: &str) -> &Notification {
        self.title = title.to_string();
        self.values["title"] = json!(&self.title);
        self
    }

    pub fn set_short_text(&mut self, short_text: &str) -> &Notification {
        self.short_text = short_text.to_string();
        self
    }

    pub fn set_short_text_template(&mut self, template: String) -> &Notification {
        self.short_text_template = Some(template.to_string());
        self
    }

    pub fn set_short_html(&mut self, short_html: &str) -> &Notification {
        self.short_html = Some(short_html.to_string());
        self
    }

    pub fn set_long_text(&mut self, long_text: &str) -> &Notification {
        self.long_text = Some(long_text.to_string());
        self
    }

    pub fn set_long_text_template(&mut self, template: String) -> &Notification {
        self.long_text_template = Some(template.to_string());
        self
    }

    pub fn set_long_html(&mut self, long_html: &str) -> &Notification {
        self.long_html = Some(long_html.to_string());
        self
    }

    pub fn set_short_html_template(&mut self, template: String) -> &Notification {
        self.short_html_template = Some(template.to_string());
        self
    }

    pub fn set_long_html_template(&mut self, template: String) -> &Notification {
        self.long_html_template = Some(template.to_string());
        self
    }

    pub fn add_value(&mut self, key: String, value: String) -> &Notification {
        self.values[key] = json!(value);
        self
    }

    pub fn send(&mut self, ttl: u32, priority: u8) -> bool {
        // Be sure {{app}}, {{url}}, {{tagline}} and {{category}} are defined
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

        // Process title (which may include {{variables}})
        outbound_notification.title = process_template(
            self.title.clone(),
            template::DEFAULT_TEXT_TEMPLATE.to_string(),
            &mut self.values);
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
            &mut self.values);

        // If custom html isn't provided, use the text version, then process
        outbound_notification.short_html = match &self.short_html {
            Some(sh) => sh.to_string(),
            None => self.short_text.clone(),
        };
        self.short_html_template = match &self.short_html_template {
            Some(sht) => Some(sht.to_string()),
            None => Some(template::DEFAULT_HTML_TEMPLATE.to_string()),
        };
        outbound_notification.short_html = process_template(outbound_notification.short_html.clone(), self.short_html_template.clone().unwrap(), &mut self.values);

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
            &mut self.values);

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
            &mut self.values);
        
        eprintln!("outbound_notification: ({:?})", outbound_notification);

        true
    }
}

#[derive(Debug, Default)]
pub struct OutboundNotification {
    app: String,
    url: String,
    tagline: String,
    category: String,
    lang: String,
    title: String,
    short_text: String,
    short_html: String,
    long_text: String,
    long_html: String,
}

fn process_template(notification: String, template: String, values: &mut Value) -> String {
    values["notification"] = json!(notification);
    let handlebars = Handlebars::new();
    match handlebars.render_template(&template, &values) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("error in process_html_template: {}", e);
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
