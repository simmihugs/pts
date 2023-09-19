use super::commandline::Commandline;

pub struct Repl {
    a: String,
}

impl Repl {
    pub fn start(cmd: &Commandline) {
        println!("filename: {}", cmd.filename());
        println!("verbose: {}\tutc: {}", cmd.verbose(), cmd.utc());
        println!("show sierrors: {}", cmd.sierror());
        println!("show special events: {}", cmd.ps_event());
        println!("show all errors: {}", cmd.all());
        println!("grep for illegal events: {}", cmd.look_for_illegalevents());
    }
}
