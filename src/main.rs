mod Health;
mod RemoteControl;
mod MultiplicationQuiz;
mod Password;

fn main() {
    password();
}

fn remote_control() {
    let mut r = RemoteControl::RemoteControl {
        current_channel: 0,
        channels: vec![String::from(""); 31],
    };
    r.save_channel_name(String::from("ARD"));

    for i in 0..31 {
        println!("{:?}", r.get_channel_name());
        r.up();
    }
}

fn bmi() {
    let h = Health::Health {};

    println!("{}", h.bmi_string(63.0, 1.75));
}

fn multiplication_quiz() {
    let m = MultiplicationQuiz::MultiplicationQuiz::new();
    println!("{}", m.get_exercise());
    println!("{}", m.get_result())
}

fn password() {
    let mut pwd_first: Vec<char> = "PassWD15!!".chars().collect();
    let pwd_weak: Vec<char> = "1234567890".chars().collect();
    let pwd_strong: Vec<char> = "NewPWD16!!".chars().collect();

    let mut p = Password::Password::new(&pwd_first);
    println!("{}", p.change_password(&pwd_first, &pwd_weak));
    pwd_first[0] = 'p';
    println!("{}", p.change_password(&pwd_first, &pwd_strong));
    pwd_first[0] = 'P';
    println!("{}", p.change_password(&pwd_first, &pwd_strong));
}