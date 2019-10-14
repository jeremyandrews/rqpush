use crate::{Notification, generate_sha256, process_template};
use serde_json::Value;

#[test]
fn test_process_template() {
    // Build a simple handlebars template.
    let notification = "This is an example.".to_string();
    let template = "{{foo}}: {{notification}}".to_string();
    let mut values: Value = json!({
        "foo": "bar",
    });

    // Process template and confirm {{foo}} and {{notification}} are properly replaced.
    let processed_template = process_template(notification, template, &mut values);
    assert_eq!(processed_template, "bar: This is an example.".to_string());
}

#[test]
fn test_sha256() {
    // Generate unsalted sha256 of "foo"
    assert_eq!(generate_sha256("foo", None), "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae");
    // Generate salted sha256 of "foo", using the salt "bar"
    assert_eq!(generate_sha256("foo", Some("bar")), "c3ab8ff13720e8ad9047dd39466b3c8974e592c2fa383d4a3960714caef0c4f2");
}

#[test]
fn test_notification() {
    let mut notification = Notification::init("example", "An example", "This is an example.");
    // Values manually set:
    assert_eq!(notification.app, "example");
    assert_eq!(notification.title, "An example");
    assert_eq!(notification.short_text, "This is an example.");
    // Automatically set:
    assert_eq!(notification.lang, "\"en\"");
    // Values not automatically set:
    assert_eq!(notification.url, None);
    assert_eq!(notification.tagline, None);
    assert_eq!(notification.category, None);
    assert_eq!(notification.short_text_template, None);
    assert_eq!(notification.short_html, None);
    assert_eq!(notification.short_html_template, None);
    assert_eq!(notification.long_text, None);
    assert_eq!(notification.long_text_template, None);
    assert_eq!(notification.long_html, None);
    assert_eq!(notification.long_html_template, None);
    assert_eq!(notification.values, json!({
        "lang": "en",
    }));

    notification.set_category("example");
    // Category value set:
    assert_eq!(notification.category, Some("example".to_string()));
    // Other values didn't change:
    assert_eq!(notification.app, "example");
    assert_eq!(notification.title, "An example");
    assert_eq!(notification.short_text, "This is an example.");
    assert_eq!(notification.tagline, None);
}