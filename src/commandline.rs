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

    #[arg(short, long, default_value_t = true)]
    sierror: bool,

    #[arg(short, long, default_value_t = false)]
    logoerror: bool,

    #[arg(short, long, default_value_t = String::from("DEFAULT"))]
    illegalevents: String,
}

// impl Args {
//     pub fn repl(&self) -> bool {
//         self.repl
//     }
// }

pub struct Commandline {
    args: Args,
    //options: CommandlineOptions,
}

// pub enum ProgrammMode {
//     SingleFile(Commandline),
//     Repl,
//     StopRightHere,
// }

// pub enum CommandlineOptions {
//     SiErrors,
//     VaEventLogoErrors,
//     GrepIllegalEvents(Vec<String>),
// }

// impl CommandlineOptions {
//     fn new(args: &Args) -> Self {
//         if args.sierror {
//             CommandlineOptions::SiErrors
//         } else if args.logoerror {
//             CommandlineOptions::VaEventLogoErrors
//         } else if args.illegalevents != "DEFAULT" {
//             CommandlineOptions::GrepIllegalEvents(
//                 args.illegalevents
//                     .split(';')
//                     .into_iter()
//                     .map(|x| String::from(x))
//                     .collect(),
//             )
//         } else {
//             //default
//             CommandlineOptions::SiErrors
//         }
//     }
// }

impl Commandline {
    pub fn parse() -> Self {
        Self {
            args: Args::parse(),
        }
    }

    pub fn verbose(&self) -> bool {
        self.args.verbose
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
    
    // pub fn illegalevents(&self) -> &String {
    //     &self.args.illegalevents
    // }

    pub fn print_help() {
        let mut cmd = Args::command();
        let _ = cmd.print_help();
    }

    pub fn filename(&self) -> &String {
        &self.args.filename
    }
}
