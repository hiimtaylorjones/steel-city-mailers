use std::env;

pub fn find_mailchimp_url() -> String {
   return env::var("MAILCHIMP_URL").expect("Mailchimp Server Url not found in config");
}

pub fn find_mailchimp_api_key() -> String {
    return env::var("MAILCHIMP_API_KEY").expect("Mailchimp API Key not found in config");
}

pub fn find_mailchimp_username() -> String {
    return env::var("MAILCHIMP_USERNAME").expect("Mailchimp Username not found in config");
}