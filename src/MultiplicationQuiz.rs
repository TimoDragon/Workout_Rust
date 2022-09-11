use std::ops::Add;
use rand::prelude::*;

pub struct MultiplicationQuiz {
    first_num: i32,
    second_num: i32,
}

impl MultiplicationQuiz {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        Self {first_num: rng.gen_range(1..20), second_num: rng.gen_range(1..20)}
    }

    pub fn get_exercise(&self) -> String {
        String::from(&self.first_num.to_string().add(" * ").add(&self.second_num.to_string()).add(" = ?"))
    }

    pub fn get_result(&self) -> i32 {
        self.first_num * self.second_num
    }
}