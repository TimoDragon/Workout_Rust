mod Health;
mod RemoteControl;

fn main() {
    remote_control();
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