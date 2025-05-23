use crate::utils::take::Take;

use chrono::{DateTime, Utc};

pub fn line(n: usize) -> String {
    "-".repeat(n)
}

pub fn print_line(n: usize) {
    println!("|{}|", line(n));
}

pub fn print_head() {
    println!(
        "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
        "day".to_string().take(11),
        "title".to_string().take(30),
        "filename".to_string().take(50),
        "programid".to_string().take(15),
        "start".to_string().take(23),
        "end".to_string().take(23),
        "duration".to_string().take(12),
        "tcin".to_string().take(12),
        "tcout".to_string().take(12),
        "contentid".to_string().take(20),
        "logo".to_string().take(16),
    );
}

pub fn print_line_cross() {
    println!(
        "|{}+{}+{}+{}+{}+{}+{}+{}+{}+{}+{}|",
        line(13),
        line(32),
        line(52),
        line(17),
        line(25),
        line(25),
        line(14),
        line(14),
        line(14),
        line(22),
        line(18),
    );
}

#[allow(dead_code)]
pub fn print_day(day: Option<DateTime<Utc>>) {
    println!(
        "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
        day.map(|d| d.format("%A").to_string().take(11))
            .unwrap_or_default(),
        "".to_string().take(30),
        "".to_string().take(50),
        "".to_string().take(15),
        "".to_string().take(23),
        "".to_string().take(23),
        "".to_string().take(12),
        "".to_string().take(12),
        "".to_string().take(12),
        "".to_string().take(20),
        "".to_string().take(16),
    );
}

pub fn missing_text_header() {
    let len = 122;
    println!("|{}|", "-".repeat(len as usize));
    println!(
        "| {} | {} | {} | {} |",
        String::from("title").take(50),
        String::from("progarmid").take(15),
        String::from("start").take(23),
        String::from("end").take(23)
    );
    println!("|{}|", "-".repeat(len as usize));
}

#[allow(dead_code)]
pub fn print_header_short() {
    println!(
        "|{}+{}+{}+{}+{}+{}|",
        line(32),
        line(17),
        line(25),
        line(25),
        line(14),
        line(22)
    );
    println!(
        "| {} | {} | {} | {} | {} | {} |",
        "title".to_string().take(30),
        "programid".to_string().take(15),
        "start".to_string().take(23),
        "end".to_string().take(23),
        "duration".to_string().take(12),
        "contentid".to_string().take(20)
    );
    println!(
        "|{}+{}+{}+{}+{}+{}|",
        line(32),
        line(17),
        line(25),
        line(25),
        line(14),
        line(22)
    );
}
