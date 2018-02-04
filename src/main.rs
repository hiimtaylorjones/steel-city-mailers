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

    // Create a custom "connector" for Hyper which will route connections
    // through the `TlsConnector` we create here after routing them through
    // `HttpConnector` first.
    let tls_connector = TlsConnector::builder().unwrap().build().unwrap();
    let mut connector = HttpsConnector {
        tls: Arc::new(tls_connector),
        http: HttpConnector::new(2, &core.handle()),
    };
    connector.http.enforce_http(false);
    let client = Client::configure()
                    .connector(connector)
                    .build(&core.handle());

    dotenv::dotenv().expect("Failed to read .env file");
    let mailchimp_url = env::var("MAILCHIMP_URL").expect("Mailchimp Server Url not found in config");
    let mailchimp_api_key = env::var("MAILCHIMP_API_KEY").expect("Mailchimp API Key not found in config");
    let mailchimp_username = env::var("MAILCHIMP_USERNAME").expect("Mailchimp Username not found in config");

    let uri = mailchimp_url.parse().unwrap();

    let mut req = Request::new(Method::Get, uri);
    req.headers_mut().set(
       Authorization(
           Basic {
               username: mailchimp_username.to_owned(),
               password: Some(mailchimp_api_key).to_owned()
           }
       )
    );
    for header in req.headers().iter() {
        print!("{}", header);
    }
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
