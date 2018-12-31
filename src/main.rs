use std::process::Command;
extern crate notify_rust;
use notify_rust::Notification;

fn color(percent: i32) -> &'static str {
    if percent <= 20 {
        return "#FF0000";
    } else if percent >= 80 {
        return "#00FF00";
    }
    else {
        return "#FFFF00";
    }
}

fn symbol(status: &str) -> &'static str {
    if status == "Charging" {
        return "";
    } else {
        return "⚡";
    }
}

fn main() {
    let cap = Command::new("cat")
        .arg("/sys/class/power_supply/BAT0/capacity")
        .output()
        .expect("failed to execute process");

    let stat = Command::new("cat")
        .arg("/sys/class/power_supply/BAT0/status")
        .output()
        .expect("failed to execute process");

    let capacity = String::from_utf8(cap.stdout)
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let status: String = String::from_utf8(stat.stdout).unwrap();

    if capacity == 10 {
        Notification::new()
            .summary("LOW BATTERY!!!")
            .body("PLUG IN YOUR COMPUTER")
            .show()
            .unwrap();
    }
    print!(
        "%{{c}}%{{F{}}} {} %{{F-}}{}%",
        color(capacity),
        symbol(status.trim()),
        capacity
    );
}
