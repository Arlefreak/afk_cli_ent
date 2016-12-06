use hyper::*;
use std::io::Read;

fn test (){
    let client = Client::new();
    let mut res = client.get("http://api.arlefreak.com/about/entry/7/").send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();

    println!("test");
}

pub fn api(){
    println!("called `afk::api()`");
}
