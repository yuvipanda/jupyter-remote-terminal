// TODO
// - [  ] Get notebook URL from commandline
// - [  ] Get token from commandline / env var
// - [  ] Send actually useful JSON struct to server
// - [  ] Find way to send and receive messages together?! Does this need async?
// - [  ] Echo stdin / stdout properly
#[macro_use] extern crate hyper;

use hyper::header::Headers;
use websocket::url::Url;
use websocket::{ClientBuilder, Message};

// Notebook expects the value of Authorization header to be 'token {token}',
// while the most common form is actually 'Bearer {token}'. websocket's default
// Authorization / Token structs use that format, which doesn't work for us.
// Maybe this is the easiest way to add this header? Hopefully there is something
// easier? But then again, this shit is super statically typed so string keys are
// not acceptable?
header! { (AuthorizationToken, "Authorization") => [String] }



fn main() {
    let url = Url::parse("ws://127.0.0.1:8888/terminals/websocket/1").unwrap();
    let token = "cdcb9149de5c2521cbbcf990501172c19b6f221eb29de24c".to_string();

    let mut headers = Headers::new();
    headers.set(AuthorizationToken(format!("token {}", token)));

    let mut client = ClientBuilder::from_url(&url)
        .custom_headers(&headers)
        .connect_insecure()
        .unwrap();

    match client.send_message(&Message::text("hello")) {
        Ok(_) => {}
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}
