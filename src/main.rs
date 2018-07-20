extern crate hyper;
extern crate hyper_tls;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

// #[macro_use]
// extern crate serde_derive;

use hyper::{Client, Request, Method, Uri};
use hyper::header::{HeaderValue, AUTHORIZATION};
use hyper::rt::{self, Future, Stream};
use hyper_tls::HttpsConnector;

use std::io::{self, Write};

// use std::net::{TcpStream, TcpListener};
use std::env;

use serde_json::{Value, Error};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() > 1 {
    //     let command = &args[1];
    // }

    // Check if the .env file is there or even readable.
    // If so, we'll check for certain keys and pull them in if they exist.
    // We need to make sure the program fails if any of these elements don't exist.
    dotenv::dotenv().expect("Failed to read .env file");
    let mailchimp_url = env::var("MAILCHIMP_URL").expect("Mailchimp Server Url not found in config");
    let mailchimp_api_key = env::var("MAILCHIMP_API_KEY").expect("Mailchimp API Key not found in config");
    let mailchimp_username = env::var("MAILCHIMP_USERNAME").expect("Mailchimp Username not found in config");

    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder()
        .build::<_, hyper::Body>(https);
    let mut request = Request::default();
    let uri: Uri = mailchimp_url.parse().unwrap();

    *request.method_mut() = Method::GET;
    *request.uri_mut() = uri.clone();
    request.headers_mut().insert("content-type", HeaderValue::from_str("application/json").unwrap());
    request.headers_mut().insert(
       AUTHORIZATION,
       HeaderValue::from_str("sample").unwrap()
    );

    println!("Requesting....");
    let post = client.request(request).and_then(|res| {
        println!("Response: {}", res.status());
        println!("Headers: {:#?}", res.headers());

        res.into_body()
            // Body is a stream, so as each chunk arrives...
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
    rt::run(post);

    // Finish off our request by fetching all of the body.
    // let unwrapped_body = response.body().concat2();
    // let body = core.run(unwrapped_body);
    // let body_string = String::from_utf8_lossy(&body);
    // let account: Account = serde_json::from_str(&body_string)?;
    // println!("{}", v["account_name"]);
}

// #[derive(Serialize, Deserialize)]
// struct Account {
//     account_name: String
// }

