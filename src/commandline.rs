use clap::{CommandFactory, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("DEFAULT"))]
    filename: String,

    #[arg(short, long, default_value_t = false)]
    repl: bool,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[arg(short, long, default_value_t = false)]
    utc: bool,

    #[arg(short, long, default_value_t = true)]
    sierror: bool,

    #[arg(short, long, default_value_t = false)]
    logoerror: bool,

    #[arg(short, long, default_value_t = String::from("DEFAULT"))]
    illegalevents: String,
}

pub struct Commandline {
    args: Args,
}

impl Commandline {
    pub fn parse() -> Self {
        Self {
            args: Args::parse(),
        }
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

    pub fn sierror(&self) -> bool {
        self.args.sierror
    }

    pub fn logoerror(&self) -> bool {
        self.args.logoerror
    }

    pub fn print_help() {
        let mut cmd = Args::command();
        let _ = cmd.print_help();
    }

    pub fn filename(&self) -> &String {
        &self.args.filename
    }

    pub fn illegalevents(&self) -> Vec<String> {
        self.args
            .illegalevents
            .split(';')
            .map(|x| String::from(x))
            .collect::<Vec<String>>()
            .to_vec()
    }
}
