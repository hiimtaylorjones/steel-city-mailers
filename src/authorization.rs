extern crate hyper;

use hyper::header::{HeaderValue, AUTHORIZATION};
use hyper::{Request, Method, Uri};

pub fn set_request_authorization(full_credentials : &str, mailchimp_url : &str) -> Request<hyper::Body> {
    // Creating the request and setting its attributes
    let mut request = Request::default();
    // Casting our ENV mailchimp url as a Uri object
    let uri: Uri = mailchimp_url.parse().unwrap();

    // Since our request is a mutex, we set our attributes like this.
    *request.method_mut() = Method::GET;
    *request.uri_mut() = uri.clone();
    request.headers_mut().insert("content-type", HeaderValue::from_str("application/json").unwrap());
    request.headers_mut().insert(
       AUTHORIZATION,
       HeaderValue::from_str(&full_credentials).unwrap()
    );
    return request;
}