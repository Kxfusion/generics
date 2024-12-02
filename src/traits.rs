use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

pub fn notifyStrict<T: Summary + Debug>(item: &T, item2: &T) {
    println!("{}", item.summarize());
    println!("{}", item.summarize());
}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Debug,
    U: Clone + Debug,
{
    5
}