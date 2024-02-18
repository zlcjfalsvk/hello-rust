mod libs;
mod traits;

use crate::libs::{NewsArticle, Tweet};
use crate::traits::Summary;
use std::fmt::{Debug, Display};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // 공백의 impl 이지만 trait 에 기본 구현을 제공함으로써 함수 작동
    // 위의 tweet 는 impl 을 정의 했기 때문에 기본 구현에 영향받지 않음
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    let tweet = returns_summarizable();
    tweet.summarize();

    let pair = Pair::new(1, 2);
    pair.cmp_display();

    // 타입이 특정 트레이트를 구현하는 경우에만 해당 타입에 트레이트를 구현
    let s = 3.to_string();
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// trait 바운드 기법
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify(item: &(impl Summary + Display)) {}

// trait 바운드 여러개 지정
// notify에서 item의 summarize 메서드뿐만 아니라 출력 포맷팅까지 사용하고 싶다고 가정했을 때,
// notify의 정의를 할때 item이 Display, Summary를 모두 구현해야 함
// pub fn notify<T: Summary + Display>(item: &T) {}

// trait 바운드에 조건 걸기
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

// trait 를 구현하는 타입 반환
// 클로저 및 반복자의 컨텍스트에서 굉장히 유용
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
