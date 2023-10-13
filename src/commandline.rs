use clap::{CommandFactory, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("YOU_PICK_A_FILE"))]
    filename: String,

    #[arg(short, long, default_value_t = false)]
    repl: bool,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[arg(short, long, default_value_t = false)]
    ps_event: bool,

    #[arg(short, long, default_value_t = false)]
    utc: bool,

    #[arg(short, long, default_value_t = false)]
    sierror: bool,

    #[arg(short, long, default_value_t = String::from("YOU_PICK_ILLEGAL_EVENTS"))]
    illegalevents: String,

    #[arg(short, long, default_value_t = false)]
    all: bool,

    #[arg(short, long, default_value_t = false)]
    only_errors: bool,

    #[arg(short, long, default_value_t = String::from("YOU_PICK_A_CSV"))]
    csv: String,

    #[arg(short, long, default_value_t = String::from("utf-8"))]
    encoding: String,

    #[arg(long, default_value_t = -1)]
    fps: i64,
}

pub struct Commandline {
    args: Args,
}

impl Commandline {
    pub fn new(verbose: bool, utc: bool, only_errors: bool, fps: i64) -> Self {
        Self {
            args: Args {
                filename: String::from(""),
                repl: false,
                verbose,
                ps_event: false,
                utc,
                sierror: false,
                illegalevents: String::from(""),
                all: false,
                only_errors,
                csv: String::from(""),
                encoding: String::from(""),
                fps,
            },
        }
    }

    pub fn parse() -> Self {
        Self {
            args: Args::parse(),
        }
    }

    pub fn only_errors(&self) -> bool {
        self.args.only_errors
    }

    pub fn ps_event(&self) -> bool {
        self.args.ps_event
    }

    pub fn verbose(&self) -> bool {
        self.args.verbose
    }

    pub fn utc(&self) -> bool {
        self.args.utc
    }

    pub fn repl(&self) -> bool {
        self.args.repl
    }

    pub fn encoding(&self) -> &String {
        &self.args.encoding
    }

    pub fn sierror(&self) -> bool {
        self.args.sierror
    }

    pub fn all(&self) -> bool {
        self.args.all
    }

    pub fn print_help() {
        let mut cmd = Args::command();
        let _ = cmd.print_help();
    }

    pub fn filename(&self) -> &String {
        &self.args.filename
    }

    /*     pub fn fps(&self) -> i64 {
           self.args.fps
       }
    */
    pub fn fps(&self) -> Option<i64> {
        if self.args.fps == -1 {
            None
        } else {
            Some(self.args.fps)
        }
    }

    pub fn csv(&self) -> &String {
        &self.args.csv
    }

    pub fn write_csv(&self) -> bool {
        self.args.csv != "YOU_PICK_A_CSV"
    }

    pub fn look_for_illegalevents(&self) -> bool {
        match self.illegalevents() {
            None => false,
            _ => true,
        }
    }

    pub fn no_option(&self) -> bool {
        !(self.look_for_illegalevents()
            || self.all()
            || self.write_csv()
            || self.ps_event()
            || self.sierror())
    }

    pub fn illegalevents(&self) -> Option<Vec<String>> {
        let illegals = &self.args.illegalevents;
        if illegals == "YOU_PICK_ILLEGAL_EVENTS" {
            None
        } else {
            Some(
                self.args
                    .illegalevents
                    .split(';')
                    .map(|x| String::from(x))
                    .collect::<Vec<String>>()
                    .to_vec(),
            )
        }
    }
}
