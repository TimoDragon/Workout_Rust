pub struct Password {
    pass: Vec<char>,
}

impl Password {
    pub fn new(pass: &Vec<char>) -> Self {
        let mut p: Vec<char> = Vec::new();

        for i in 0..pass.iter().count() {
            p.push(pass[i]);
        }

        Self { pass: p }
    }

    pub fn is_strong(&self) -> bool {
        check_if_strong(&self.pass)
    }

    pub fn is_strong_custom(&self, pass: &Vec<char>) -> bool {
        check_if_strong(&pass)
    }

    pub fn change_password(&mut self, old_password: &Vec<char>, new_password: &Vec<char>) -> bool {
        if !is_same(&self.pass, &old_password) || !check_if_strong(&new_password) {
            return false;
        }

        self.pass.clear();
        for i in 0..new_password.iter().count() {
            self.pass.push(new_password[i]);
        }
        true
    }
}

fn check_if_strong(pass: &Vec<char>) -> bool {
    let mut upper: bool = false;
    let mut lower: bool = false;
    let mut special: bool = false;
    let mut number: bool = false;

    for i in 0..pass.len() {
        if pass[i] >= 'a' && pass[i] <= 'z' {
            lower = true;
        }
        else if pass[i] >= 'A' && pass[i] <= 'Z' {
            upper = true;
        }
        else if pass[i] >= '0' && pass[i] <= '9' {
            number = true;
        }
        else if pass[i] >= '!' && pass[i] <= '.' {
            special = true;
        }
    }

    upper && lower && special && number
}

fn is_same(first: &Vec<char>, second: &Vec<char>) -> bool {
    for x in 0..first.iter().count() {
        if first[x] != second[x] {
            return false;
        }
    }

    true
}