pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn traits() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("George R. R. Martin"),
        headline: String::from("George R. R. Martin confirms he’s 3/4 done with ‘Winds of Winter’"),
        content: String::from(
            r#"""Game of Thrones fans rejoice, George R. R. Martin has confirmed that the next instalment in the series is well on the way.
Speaking in a live stream with Random House, Martin spoke about the latest series of House of the Dragon and the long-awaited Winds of Winter.
“It’s a big, big book,” said Martin. “It’s a challenging book. It’s probably going to be a larger book than any of the previous volumes in the series.
“The Dance of Dragons and Storm of Swords were the two largest books in the series. They were both about 1500 manuscript pages. I think this one is gonna be longer than that, by the time I’ve finished it.”
        """#,
        ),
        location: String::from(
            "https://thebrag.com/george-r-r-martin-confirms-hes-3-4-done-with-winds-of-winter/",
        ),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", article.summarize());
}
