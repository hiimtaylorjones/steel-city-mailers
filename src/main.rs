extern crate hyper;
extern crate hyper_tls;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate base64;

use hyper::{Client, Request, Method, Uri};
use hyper::header::{HeaderValue, AUTHORIZATION};
use hyper::rt::{self, Future, Stream};
use hyper_tls::HttpsConnector;

use std::io::{self, Write};
use std::env;

use base64::{encode};

fn main() {

    // Check if the .env file is there or even readable.
    // If so, we'll check for certain keys and pull them in if they exist.
    // We need to make sure the program fails if any of these elements don't exist.
    dotenv::dotenv().expect("Failed to read .env file");
    let mailchimp_url = env::var("MAILCHIMP_URL").expect("Mailchimp Server Url not found in config");
    let mailchimp_api_key = env::var("MAILCHIMP_API_KEY").expect("Mailchimp API Key not found in config");
    let mailchimp_username = env::var("MAILCHIMP_USERNAME").expect("Mailchimp Username not found in config");

    // Creating and configuring out HTTPS connector.
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    // Creating the request and setting its attributes
    let mut request = Request::default();
    // Casting our ENV mailchimp url as a Uri object
    let uri: Uri = mailchimp_url.parse().unwrap();

    // Merging the username and api key from our ENV together like:
    //  username:api_key
    // 
    // We then encode this combination string via base64 so that we can 
    // insert it into our Authorization header.
    let credential_string = [mailchimp_username, mailchimp_api_key].join(":");
    let encoded_credentials = encode(&credential_string);
    let full_credentials = format!("Basic {}", encoded_credentials);

    // Since our request is a mutex, we set our attributes like this.
    *request.method_mut() = Method::GET;
    *request.uri_mut() = uri.clone();
    request.headers_mut().insert("content-type", HeaderValue::from_str("application/json").unwrap());
    request.headers_mut().insert(
       AUTHORIZATION,
       HeaderValue::from_str(&full_credentials).unwrap()
    );

    println!("Requesting....");
    let post = client.request(request).and_then(|res| {
        println!("Response: {}", res.status());
        println!("Headers: {:#?}", res.headers());

        res.into_body()
            // Our response body comes in the form of a stream.
            // Because of this, we want to print out each part as it 
            // comes into view. 
            .for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map_err(|e| {
                        panic!("example expects stdout is open, error={}", e)
                    })
            })
    })
    .map(|_| {
        println!("\n\nDone.");
    })
    .map_err(|err| {
        println!("Error: {}", err);
    });

    // The request is executed here via rt::run. 
    rt::run(post);
}

