
use reqwest::blocking::Client;

fn main() {
    println!("Hello world");

    let mut client = Client::default();

    let response = 
        client.get("http://google.com")
              .send();

    match response {
        Ok(val) => {
            println!("Web request successful. Response: {val:?}");
        },
        Err(val) => {
            println!("Error: {val}");
        },
    }
}

