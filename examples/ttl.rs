use rqpush::Notification;

fn main() {
    // Create a notification by setting the app name, a notification title, and a short message.
    let mut notification = Notification::init("Example", "An example", "This is an example notification.");

    // Send the notification: in this example we send it to port 8000 on localhost, with a priority
    // of 100. The TTL is set to 60, so if it takes more than 60 seconds to deliver the notification
    // it will be quietly dropped.
    match notification.send("http://localhost:8000", 100, 60, None) {
        Ok(r) => println!("Success: {:?}", r),
        Err(e) => println!("Failure: {:?}", e),
    }
}
