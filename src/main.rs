use serde::{Deserialize, Serialize};

/// Represents a paragraph with a name.
#[derive(Serialize, Deserialize)]
struct Paragraph {
    /// The name of the paragraph.
    name: String,
}

/// Represents an article with paragraphs and an author.
#[derive(Serialize, Deserialize)]
struct Article {
    /// The title of the article.
    article: String,
    /// The author of the article.
    author: String,
    /// List of paragraphs in the article.
    paragraph: Vec<Paragraph>,
}

fn main() {
    // JSON data representing an article with paragraphs
    let json = r#"{
        "article": "working with json data in rust",
        "author": "chinedu",
        "paragraph": [
            {
                "name": "lorem10"
            },
            {
                "name": "lorem20"
            },
            {
                "name": "lorem30"
            }
        ]
    }"#;

    // Deserialize the JSON data into an `Article` struct using the `read_json_type` function
    let parsed: Article = read_json_type(json);

    // Print the name of the second paragraph
    println!("The name of the second paragraph is: {}", parsed.paragraph[1].name);
}

/// Deserialize a JSON string into an `Article` struct.
///
/// # Arguments
///
/// * `json` - A JSON string representing an article with paragraphs and an author.
///
/// # Returns
///
/// Returns an `Article` struct containing the deserialized data.

fn read_json_type(json: &str) -> Article {  
    let parsed: Article = serde_json::from_str(json).unwrap();
    parsed
}



