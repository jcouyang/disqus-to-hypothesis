#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

use serde_xml_rs::deserialize;
use std::fs::File;
use std::fs;
use std::io::BufReader;
use std::env;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Thread {
    link: String
}


#[derive(Serialize, Deserialize, Debug)]
struct Post {
    message: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Disqus {
    post: Vec<Post>,
    thread: Vec<Thread>
}


fn main() {
    let args:Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).expect("file not found");

    let data: Disqus = serde_xml_rs::deserialize(file).unwrap();
    println!("In file {}", data.post.first().unwrap().message);
}
