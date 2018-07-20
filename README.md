# Steel City Mailers

## What is this?

Awhile ago, I found myself wondering how and effective mailing service would be implemented in 
Rust. I found myself curious to mess with the Mailchimp API as well. This gave rise to Steel City 
Mailers. Named after Birmingham, AL or Pittsburgh, PA (depending on which one you like more).

Its basic functionality right now is to offer and easy and performant way to interact with Mailchimp's API. 

## How Does It Work?

To run the project, you'll need to have an .env that looks a bit like this:

```
MAILCHIMP_URL="https://us14.api.mailchimp.com/[mailchimp_api_version]/"
MAILCHIMP_API_KEY="[mailchimp_api_key]"
MAILCHIMP_USERNAME="[mailchimp_api_username]"
```

Then run `cargo build` and you should be on your way! 

