extern crate futures;
extern crate hyper;
extern crate native_tls;
extern crate tokio_core;
extern crate tokio_service;
extern crate tokio_tls;
extern crate dotenv;

use std::io;
use std::sync::Arc;

use futures::future::{err, Future};
use futures::stream::Stream;

use hyper::client::HttpConnector;
use hyper::{Client, Request, Method, Uri};
use hyper::header::{Connection, Headers, Authorization, Basic};

use native_tls::TlsConnector;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_service::Service;
use tokio_tls::{TlsConnectorExt, TlsStream};
use std::env;

fn main() {
    let mut core = Core::new().unwrap();

    // Creates a TLS connector for us to better communicate with Mailchimp 
    // in a safe and efficient way. 
    let tls_connector = TlsConnector::builder().unwrap().build().unwrap();
    // We're also generating a HTTPS connector for the job as well.
    // Notice how we're wrapping the TLS connector we just created in order 
    // to insert it into the HTTPS connector 
    let mut connector = HttpsConnector {
        tls: Arc::new(tls_connector),
        http: HttpConnector::new(2, &core.handle()),
    };
    // We're not enforcing http at the moment, but this is where would
    // determine that enforcement.
    connector.http.enforce_http(false);
    let client = Client::configure()
                    .connector(connector)
                    .build(&core.handle());

    // Check if the .env file is there or even readable.
    // If so, we'll check for certain keys and pull them in if they exist.
    // We need to make sure the program fails if any of these elements don't exist.
    dotenv::dotenv().expect("Failed to read .env file");
    let mailchimp_url = env::var("MAILCHIMP_URL").expect("Mailchimp Server Url not found in config");
    let mailchimp_api_key = env::var("MAILCHIMP_API_KEY").expect("Mailchimp API Key not found in config");
    let mailchimp_username = env::var("MAILCHIMP_USERNAME").expect("Mailchimp Username not found in config");

    // Prep the mailchimp url for work.
    let uri = mailchimp_url.parse().unwrap();
    let mut req = Request::new(Method::Get, uri);
    
    // Set our headers for the request. We're using a basic authorization, but 
    // I've found it easy to set whatever you want in here as long as it works 
    // in an API client like Paw or Postman.
    req.headers_mut().set(
       Authorization(
           Basic {
               username: mailchimp_username.to_owned(),
               password: Some(mailchimp_api_key).to_owned()
           }
       )
    );

    // Debugging for headers.
    for header in req.headers().iter() {
        print!("{}", header);
    }

    // Execute our request and print out the result for debugging purposes.
    let response = core.run(client.request(req)).unwrap();
    println!("{} {}", response.version(), response.status());
    for header in response.headers().iter() {
        print!("{}", header);
    }

    // Finish off our request by fetching all of the body.
    let body = core.run(response.body().concat2()).unwrap();
    println!("{}", String::from_utf8_lossy(&body));
}

struct HttpsConnector {
    tls: Arc<TlsConnector>,
    http: HttpConnector,
}

impl Service for HttpsConnector {
    type Request = Uri;
    type Response = TlsStream<TcpStream>;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = io::Error>>;

    fn call(&self, uri: Uri) -> Self::Future {
        if uri.scheme() != Some("https") {
            return err(io::Error::new(io::ErrorKind::Other,
                                      "only works with https")).boxed()
        }

        let host = match uri.host() {
            Some(s) => s.to_string(),
            None =>  {
                return err(io::Error::new(io::ErrorKind::Other,
                                          "missing host")).boxed()
            }
        };

        let tls_cx = self.tls.clone();
        Box::new(self.http.call(uri).and_then(move |tcp| {
            tls_cx.connect_async(&host, tcp)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        }))
    }
}
