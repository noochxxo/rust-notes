extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::fs::OpenOptions;

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
  pub list: Vec<String>,
  pub paragraph: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
  pub _id: u16,
  pub author: String,
  pub title: String,
  pub date: String,
  pub list: bool,
  pub body: Body,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notes {
  notes: Vec<Note>
}

pub fn notes_from_file(f: &String) -> Notes {
    let file = File::open(f).unwrap();
    let deserialized_json: Notes = serde_json::from_reader(file).unwrap();
    deserialized_json
}
