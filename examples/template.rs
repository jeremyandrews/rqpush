use rqpush::Notification;

fn main() {
    // Create a notification by setting the app name, a notification title, and a short message.
    let mut notification = Notification::init("Example", "An example", "This is an example notification.");

    // Create a custom template, generating something like this: 
    //    "[Example] This is an example notification. (3)"
    notification.set_short_text_template("[{{app}}]: {{notification}} ({{integer}})".to_string());
    notification.add_value("integer".to_string(), 3.to_string());

    // Send the notification: in this example we send it to port 8000 on localhost, with a priority
    // of 55. We do not set a TTL nor a shared key.
    match notification.send("http://localhost:8000", 55, 0, None) {
        Ok(r) => println!("Success: {:?}", r),
        Err(e) => println!("Failure: {:?}", e),
    }
}