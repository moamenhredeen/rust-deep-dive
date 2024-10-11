//! Adapted from https://github.com/ferrous-systems/teaching-material/blob/main/assignments/serde-lifetimes.adoc


use serde::{Deserialize, Serialize};

/// pretend that we call an API and get a JSON String back
fn fetch_data() -> String {
    String::from(
        r#"
            {
                "id": 1,
                "title": "Hello, Rust"
            }
        "#,
    )
}
 
#[derive(Debug, Serialize, Deserialize)]
struct BlogPost {
    id: u32,
    title: String,
}

fn main() -> anyhow::Result<()> {
    let post: BlogPost = {
        let data = fetch_data();
        match serde_json::from_str(data.as_str()) {
            Ok(value) => value,
            Err(err) => panic!("{}", err),
        }
    };

    println!("deserialized = {:?}", post);

    let post_json = match serde_json::to_string(&post) {
        Ok(value) => value,
        Err(err) => panic!("{}", err) 
    };
    println!("serialized = {:?}", post_json);

    Ok(())
}
