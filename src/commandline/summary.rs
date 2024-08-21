use colored::Colorize;

use crate::commandline::commandline::Commandline;
pub struct Summary {
    pub time_errors: i64,
    pub id_errors: i64,
    pub logo_errors: i64,
    pub special_event_errors: i64,
    pub va_errors: i64,
    pub si_errors: i64,
    pub text_error: i64,
    pub length_error: i64,
    pub si_length_error: i64,
    pub commercial_error: i64,
    pub trailer_balls_error: i64,
}

impl Summary {
    pub fn new() -> Self {
        Summary {
            time_errors: 0,
            id_errors: 0,
            logo_errors: 0,
            special_event_errors: 0,
            va_errors: 0,
            si_errors: 0,
            text_error: 0,
            length_error: 0,
            si_length_error: 0,
            commercial_error: 0,
            trailer_balls_error: 0,
        }
    }

    pub fn print(&self, cmd: &Commandline) {
        if cmd.all() || cmd.ps_event() || cmd.vaerrors() || cmd.sierrors() {
            println!("{}", "Error Summary:");
        }

        if cmd.all() || cmd.ps_event() {
            println!(
                "{:3} time errors",
                if self.time_errors > 0 {
                    format!("{}", self.time_errors).red()
                } else {
                    format!("{}", self.time_errors).green()
                }
            );
            println!(
                "{:3} id errors",
                if self.id_errors > 0 {
                    format!("{}", self.id_errors).red()
                } else {
                    format!("{}", self.id_errors).green()
                }
            );
            println!(
                "{:3} logo errors",
                if self.logo_errors > 0 {
                    format!("{}", self.logo_errors).red()
                } else {
                    format!("{}", self.logo_errors).green()
                }
            );
            println!(
                "{:3} special event errors",
                if self.special_event_errors == 0 {
                    format!("{}", 0).green()
                } else {
                    format!("{}", self.special_event_errors).red()
                }
            );

            println!(
                "{:3} length errors",
                if self.length_error == 0 {
                    format!("{}", 0).green()
                } else {
                    format!("{}", self.length_error).red()
                }
            );

            println!(
                "{:3} si length errors",
                if self.si_length_error == 0 {
                    format!("{}", 0).green()
                } else {
                    format!("{}", self.si_length_error).red()
                }
            );

            println!(
                "{:3} commercial length errors",
                if self.commercial_error == 0 {
                    format!("{}", 0).green()
                } else {
                    format!("{}", self.commercial_error).red()
                }
            );
        }

        if cmd.all() || cmd.vaerrors() {
            println!(
                "{:3} vaerrors",
                if self.va_errors == 0 {
                    format!("{}", 0).green()
                } else {
                    format!("{}", self.va_errors).red()
                }
            );
        }

        if cmd.all() || cmd.sierrors() {
            println!(
                "{:3} sierrors",
                if self.si_errors == 0 {
                    format!("{}", self.si_errors).green()
                } else {
                    format!("{}", self.si_errors).red()
                }
            );
        }

        if cmd.all() || cmd.missing_texts() {
            println!(
                "{:3} missing_texts",
                if self.text_error == 0 {
                    format!("{}", self.text_error).green()
                } else {
                    format!("{}", self.text_error).red()
                }
            );
        }

        if cmd.all() {
            println!(
                "{:3} trailer balls mixup",
                if self.trailer_balls_error == 0 {
                    format!("{}", self.trailer_balls_error).green()
                } else {
                    format!("{}", self.trailer_balls_error).red()
                }
            );
        }
    }
}
