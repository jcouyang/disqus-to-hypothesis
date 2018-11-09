#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate serde_json;
use serde_xml_rs::deserialize;
use std::env;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Thread {
    dsqid: String,
    author: Author,
    link: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ThreadId {
    dsqid: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    message: String,
    author: Author,
    thread: ThreadId
}

#[derive(Serialize, Deserialize, Debug)]
struct Disqus {
    post: Vec<Post>,
    thread: Vec<Thread>
}
#[derive(Serialize, Deserialize, Debug)]
struct Target { 
    source: String
}
#[derive(Serialize, Deserialize, Debug)]
struct Permission {
    read: Vec<String>
}
#[derive(Serialize, Deserialize, Debug)]
struct Annotation {
    uri: String,
    target: Vec<Target>,
    group: String,
    permissions: Permission,
    text: String,
    tags: Vec<String>
}

struct Config {
    pub h_url: String,
    pub filename: String,
    pub token: String
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, std::env::VarError>{
        if args.len() < 1 {
            return Err(std::env::VarError::NotPresent)
        }
        let token = env::var("H_TOKEN")?;
        let h_url = match env::var("H_URL") {
            Ok(url) => url,
            _ => String::from("https://hypothes.is")
        };
        let filename = args[1].clone();
        Ok(Config { h_url, filename, token})
    }
}
fn composeAnnotation(disqus: &Disqus) -> Vec<Annotation> {
    disqus.post.iter()
        .map(|ref p|{
            let link = &disqus.thread.iter().find(|&t| t.dsqid == p.thread.dsqid).unwrap().link;
            Annotation {
                    uri: link.clone(),
                    target: vec![Target{source: link.clone()}],
                    group: String::from("__world__"),
                    permissions: Permission{read: vec![String::from("group:__world__")]},
                    text: p.message.clone(),
                    tags: vec![format!("from:{}", p.author.name)]
            }
        }).collect::<Vec<Annotation>>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args).unwrap();
    let data: Disqus = deserialize(
        fs::read_to_string(config.filename)
            .unwrap()
            .replace("dsq:id", "dsqid")
            .as_bytes()
    ).unwrap();
    let client = reqwest::Client::new();
    let hurl = config.h_url + "/api/annotations";
    for anno in composeAnnotation(&data).iter() {
        println!("{:?}",client.post(hurl.as_str())
                .body(serde_json::to_string(anno).unwrap())
                .send());

    }
}
