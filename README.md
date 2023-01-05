# Rust-Notes-App

Ugly ass notes app

Reads a json file from the server. displays it in the browser.

TODO: write to the json file on the server

### Note
The user chooses between a list or a paragraph.

If the list is chosesn then the paragraph is an empty String.
If the paragraph is chosen then the list is an empty vector

### JSON structure
```rust
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

```