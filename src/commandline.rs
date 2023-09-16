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
    sierror: bool,

    #[arg(short, long, default_value_t = false)]
    logoerror: bool,

    #[arg(short, long, default_value_t = String::from("DEFAULT"))]
    illegalevents: String,
}

pub struct Commandline {
    args: Args,
    options: CommandlineOptions,
}

pub enum ProgrammMode {
    SingleFile(Commandline),
    Repl,
    StopRightHere,
}

pub enum CommandlineOptions {
    SiErrors,
    VaEventLogoErrors,
    GrepIllegalEvents(Vec<String>),
}

impl CommandlineOptions {
    fn new(args: &Args) -> Self {
        if args.sierror {
            CommandlineOptions::SiErrors
        } else if args.logoerror {
            CommandlineOptions::VaEventLogoErrors
        } else if args.illegalevents != "DEFAULT" {
            CommandlineOptions::GrepIllegalEvents(
                args.illegalevents
                    .split(';')
                    .into_iter()
                    .map(|x| String::from(x))
                    .collect(),
            )
        } else {
            //default
            CommandlineOptions::SiErrors
        }
    }
}

impl Commandline {
    pub fn parse() -> ProgrammMode {
        let args: Args = Args::parse();
        let options = CommandlineOptions::new(&args);

        if args.repl {
            ProgrammMode::Repl
        } else {
            if args.filename == "DEFAULT" {
                ProgrammMode::StopRightHere
            } else {
                ProgrammMode::SingleFile(Commandline { args, options })
            }
        }
    }

    pub fn options(&self) -> &CommandlineOptions {
        &self.options
    }

    pub fn verbose(&self) -> bool {
        self.args.verbose
    }

    pub fn print_help() {
        let mut cmd = Args::command();
        let _ = cmd.print_help();
    }

    pub fn filename(&self) -> &String {
        &self.args.filename
    }
}
