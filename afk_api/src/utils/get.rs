use hyper;
use std::io::Read;
use hyper::Client;

// TODO: Implement get_single<T> & get_collection<T>
// https://github.com/QuietMisdreavus/twitter-rs/blob/master/src/user/fun.rs#L108
// https://github.com/QuietMisdreavus/twitter-rs/blob/master/src/common/response.rs#L380

pub fn content(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = try!(client.get(url).send());
    let mut buf = String::new();
    try!(response.read_to_string(&mut buf));
    Ok(buf)
}
