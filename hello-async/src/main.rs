use trpl::{Either, Html};
use std::env;

async fn page_title(url: &str) -> (&str, Option<String>) {
    // async postfix keyword "await" is lazy, notice await has no `()`
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
                    .select_first("title")
                    .map(|title_element| title_element.inner_html());
    (url, title)
}

fn main() {
    // use collect method to get vec from Args iterator
    let args: Vec<String> = env::args().collect();

    trpl::run(async {
        let title_future_1 = page_title(&args[1]);
        let title_future_2 = page_title(&args[2]);
        // race the two async function calls
        let (url, maybe_title) = 
            match trpl::race(title_future_1, title_future_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };
        
        println!("{url} returned first");
        // try to extract title
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
    
}
