mod front_of_house;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();

    // use 를 사용하여 가져올 수 있음
    hosting::add_to_waitlist();

    // 호밀 (Rye) 토스트를 곁들인 여름철 조식 주문하기
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 먹고 싶은 빵 바꾸기
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 다음 라인의 주석을 해제하면 컴파일되지 않습니다; 식사와 함께
    // 제공되는 계절 과일은 조회나 수정이 허용되지 않습니다
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // 자기 부모 모듈부터 시작되는 상대경로 생성 가능
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}