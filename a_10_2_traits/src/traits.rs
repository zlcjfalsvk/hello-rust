pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    // fn summarize_author(&self) -> String;
    //
    // fn summarize(&self) -> String {
    //     format!("(Read more from {}...)", self.summarize_author())
    // }
}
