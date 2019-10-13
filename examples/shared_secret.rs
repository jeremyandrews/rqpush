use rqpush::Notification;

fn main() {
    // Create a notification by setting the app name, a notification title, and a short message.
    let mut notification = Notification::init("Example", "An example", "This is an example notification.");

    // Send the notification: in this example we send it to port 8000 on localhost, with a priority
    // of 100. The shared_secret is set to "foo" so the sha255 of our message is salted and will
    // only be accepted by rqueue if also configured with the same shared secret. This prevents
    // invalid notification spam landing in the rqueue.
    match notification.send("http://localhost:8000", 122, 0, Some("foo")) {
        Ok(r) => println!("Success: {:?}", r),
        Err(e) => println!("Failure: {:?}", e),
    }
}