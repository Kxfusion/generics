use lifetimes::life;
use traits::{Summary, Tweet, NewsArticle};

mod traits;
mod lifetimes;

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p2: Point<f32> = Point { x: 3.0, y: 2.0};
    println!("{}", p2.distance_from_origin());

    let tweet = Tweet {
        username: String::from("name"),
        content: String::from("Stuff"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("article {}", article.summarize());

    println!("1 new tweet: {}", tweet.summarize());

    life();
}
