use serde::Serialize;
use serde_json::Result;
use std::process::exit;
use upower_dbus::UPower;

#[derive(Serialize)]
struct Msg {
    text: String,
    tooltip: &'static str,
    class: &'static str,
    percentage: f64,
}

enum Icon {
    Empty,
    Low,
    Medium,
    High,
    Full,
    Charging,
    Plugged,
}

impl Icon {
    fn from_pct(pct: &f64) -> Self {
        return if pct > &0.0 && pct <= &15.0 {
            Icon::Empty
        } else if pct > &15.0 && pct <= &40.0 {
            Icon::Low
        } else if pct > &40.0 && pct <= &60.0 {
            Icon::Medium
        } else if pct > &60.0 && pct <= &80.0 {
            Icon::High
        } else {
            Icon::Full
        };
    }

    fn icon(&self) -> &'static str {
        match self {
            Icon::Empty => "",
            Icon::Low => "",
            Icon::Medium => "",
            Icon::High => "",
            Icon::Full => "",
            Icon::Charging => "",
            Icon::Plugged => "",
        }
    }
}

enum Class {
    Critical,
    Warning,
    Good,
}

impl Class {
    fn from(pct: &f64) -> Self {
        return if pct > &0.0 && pct <= &10.0 {
            Class::Critical
        } else if pct > &10.0 && pct <= &30.0 {
            Class::Warning
        } else {
            Class::Good
        };
    }

    fn class(&self) -> &'static str {
        match self {
            Class::Critical => "critical",
            Class::Warning => "warning",
            Class::Good => "good",
        }
    }
}

fn main() {
    let upower = match UPower::new(1000) {
        Ok(upower) => upower,
        Err(e) => {
            eprintln!("Failed to get dbus connection: {}", e);
            exit(1);
        }
    };
    let display_device = upower.get_display_device().unwrap().unwrap();
    let pct = upower.get_percentage_of(display_device).unwrap().round();

    let icon = if upower.on_battery().unwrap() {
        Icon::from_pct(&pct)
    } else {
        Icon::Plugged
    }
    .icon();

    let class = Class::from(&pct).class();

    let text = format!("{}% {}", pct, icon);

    let msg = serde_json::to_string(&Msg {
        text,
        tooltip: "",
        class,
        percentage: pct,
    })
    .unwrap();

    println!("{}", msg);
}
