//! # Minreq
//! Simple, minimal-dependency HTTP client.
//! The library has a very minimal API, so you'll probably know
//! everything you need to after reading a few examples.
//!
//! # Examples
//!
//! ## Get
//! ```no_run
//! // This is a simple example of sending a GET request and
//! // printing out the response.
//! if let Ok(response) = minreq::get("http://httpbin.org/ip").send() {
//!     println!("{}", response.body);
//! }
//! ```
//!
//! ## Body
//! ```no_run
//! // To include a body, add .with_body("") before .send().
//! if let Ok(response) = minreq::post("http://httpbin.org/post")
//!     .with_body("Pong!")
//!     .send()
//! {
//!     println!("{}", response.body);
//! }
//! ```
//!
//! ## Headers
//! ```no_run
//! // To add a header, add .with_header("Key", "Value") before .send().
//! if let Ok(response) = minreq::get("http://httpbin.org/headers")
//!     .with_header("Accept", "text/plain")
//!     .with_header("Something", "Interesting")
//!     .send()
//! {
//!     println!("{}", response.body);
//! }
//! ```
//!
//! ## Timeouts
//! ```no_run
//! // To avoid timing out, or limit the request's response time even more,
//! // use .with_timeout(n) before .send(). The given value is in seconds.
//! // NOTE: The default timeout is 5 seconds.
//! if let Ok(response) = minreq::post("http://httpbin.org/delay/6")
//!     .with_timeout(10)
//!     .send()
//! {
//!     println!("{}", response.body);
//! }
//! ```
//!
//! # Timeouts
//! The timeout of the created request is 5 seconds by default. You
//! can change this in two ways:
//! - Use this function (`create_request`) and call
//!   [`with_timeout`](struct.Request.html#method.with_timeout)
//!   on it to set the timeout per-request like so:
//!   ```
//!   minreq::get("/").with_timeout(8).send();
//!   ```
//! - Set the environment variable `MINREQ_TIMEOUT` to the desired
//!   amount of seconds until timeout. Ie. if you have a program called
//!   `foo` that uses minreq, and you want all the requests made by that
//!   program to timeout in 8 seconds, you launch the program like so:
//!   ```text,ignore
//!   $ MINREQ_TIMEOUT=8 ./foo
//!   ```
//!   Or add the following somewhere before the requests in the code.
//!   ```
//!   use std::env;
//!
//!   env::set_var("MINREQ_TIMEOUT", "8");
//!   ```

#![deny(missing_docs)]

#[cfg(test)]
extern crate tiny_http;

mod requests;
mod http;
mod connection;
#[cfg(test)]
mod tests;

pub use requests::*;
pub use http::*;
