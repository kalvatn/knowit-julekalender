use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use reqwest::blocking::Client;
use reqwest::Error;

pub fn download_input(url: &str) -> Result<String, Error> {
  println!("downloading input from {}", url);
  let client = Client::new();
  let response = client.get(url).send()?;
  let text = response.text()?;
  Ok(text)
}

pub fn read_input(url: &str, filename: &str) -> String {
  let filepath = format!("inputs/{}", filename);
  let path = Path::new(&filepath);
  let file = match path.exists() {
    true => File::open(path),
    false => {
      if url.is_empty() {
        panic!("empty url");
      }
      let response = download_input(&url);
      let mut out = File::create(&path).expect("failed to create file");
      io::copy(&mut response.unwrap().as_bytes(), &mut out).expect("failed to copy contents");
      File::open(&path)
    }
  };
  let mut contents = String::new();

  file
    .unwrap()
    .read_to_string(&mut contents)
    .expect("unable to read contents");
  return contents;
}
