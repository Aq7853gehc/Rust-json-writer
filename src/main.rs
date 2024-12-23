
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
struct Paragraph{
    name : String
}
#[derive(Deserialize,Serialize)]
struct Article{
    article: String,
    author : String,
    paragraph:Vec<Paragraph>
}

fn main() {
    let article = Article{
        article:String::from("How to write json code using rust"),
        author:String::from("Aquib"),
        paragraph:vec![Paragraph{
            name:String::from("Start paragraph")
        },
        Paragraph{
            name:String::from("Body of paragraph")
        },
        Paragraph{
            name:String::from("Ending of paragraph")
        }
        ]
    };

    let parsed = serde_json::to_string(&article).unwrap();
    println!("{}",parsed);

}