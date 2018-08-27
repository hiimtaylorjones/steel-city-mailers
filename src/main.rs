extern crate hyper;
extern crate hyper_tls;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate base64;

use hyper::{Client, Body};

use hyper::rt::{self, Future, Stream};
use hyper_tls::HttpsConnector;

use std::io::{self, Write};

use base64::{encode};

mod authorization;
mod mailchimp_config;

fn main() {

    // Check if the .env file is there or even readable.
    // If so, we'll check for certain keys and pull them in if they exist.
    // We need to make sure the program fails if any of these elements don't exist.
    dotenv::dotenv().expect("Failed to read .env file");
    let mailchimp_url = mailchimp_config::find_mailchimp_url();
    let mailchimp_api_key = mailchimp_config::find_mailchimp_api_key();
    let mailchimp_username = mailchimp_config::find_mailchimp_username();

    // Merging the username and api key from our ENV together like:
    //  username:api_key
    // 
    // We then encode this combination string via base64 so that we can 
    // insert it into our Authorization header.
    let credential_string = [mailchimp_username, mailchimp_api_key].join(":");
    let encoded_credentials = encode(&credential_string);
    let full_credentials = format!("Basic {}", encoded_credentials);

    // Creating and configuring out HTTPS connector.
    
    let client = generate_client();
    let request = authorization::set_request_authorization(&full_credentials, &mailchimp_url);

    println!("Requesting....");
    let get = client.request(request).and_then(|res| {
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
    rt::run(get);
}

fn generate_client() -> Client<HttpsConnector<hyper::client::HttpConnector>> {
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder()
        .build::<_, Body>(https);
    return client;
}