use super::commandline::Commandline;
use crate::pts_loader::dataset::DataSet;
use std::io::Write;

#[derive(Clone)]
pub struct Repl;

macro_rules! strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

impl Repl {
    fn print_help() {
        println!(":h | :help --> print help");
        println!(":q | :quit --> quit");
        println!(":l | :list --> list loaded files");
        println!(":f <FILE>| :load <FILE> --> try loading new file");
    }

    pub fn parse<'a>(
        cmd: &'a Commandline,
        user_io: String,
        files: &mut Vec<DataSet>,
    ) -> Result<String, ()> {
        let help: Vec<String> = strings![":help", ":h", "-h", "-help"];
        let quit: Vec<String> = strings![":quit", ":q", "-q", "-quit"];
        let load: Vec<String> = strings![":load", ":f", "-f ", "-load"];
        let list: Vec<String> = strings![":list", ":l", "-l", "-list"];
        let all: Vec<String> = strings![":all", ":a", "-a", "-all"];
        let user_io_clean = user_io
            .replace("\r", "")
            .replace("\n", "")
            .trim_end()
            .to_string();
        let words = user_io_clean
            .split(' ')
            .map(|x| String::from(x))
            .collect::<Vec<String>>();

        if help.contains(&user_io_clean) {
            Repl::print_help();
            Ok("help".to_string())
        } else if list.contains(&user_io_clean) {
            for file in files {
                println!("{}", file.get_filename());
            }
            Ok("listed files".to_string())
        } else if words.len() > 1 {
            match words.iter().position(|x| load.contains(x)) {
                Some(index) => {
                    if words.len() > index + 1 {
                        let new_file = &words[index + 1];
                        if !files
                            .iter()
                            .map(|x| String::from(x.get_filename()))
                            .collect::<Vec<String>>()
                            .contains(&new_file)
                        {
                            match DataSet::init(&new_file) {
                                Err(e) => println!("{}", e),
                                Ok(data) => {
                                    println!("Loaded file: {}", new_file);
                                    files.push(data);
                                }
                            }
                        } else {
                            println!("file {} is allready loaded ", new_file);
                        }
                    }
                }
                None => (),
            }
            match words.iter().position(|x| all.contains(x)) {
                Some(index) => {
                    if words.len() > index + 1 {
                        match words[index + 1].parse::<i32>() {
                            Ok(file_index) => {
                                let _index: usize = file_index.try_into().unwrap();
                                if files.len() > _index {
                                    files[_index].print_si_errors(cmd.verbose(), cmd.utc());
                                    files[_index].print_special_events(cmd.verbose(), cmd.utc());
                                } else {
                                    println!("invalid index {}", file_index);
                                }
                            }
                            Err(e) => println!("{}", e),
                        }
                    }
                }
                None => (),
            }
            Ok("".to_string())
        } else if quit.contains(&user_io_clean) {
            Err(())
        } else {
            Ok(format!("{}", user_io_clean))
        }
    }

    fn run_repl(cmd: &Commandline, files: &mut Vec<DataSet>) {
        use std::io;
        let mut user_io = String::new();
        loop {
            io::stdout()
                .write_all(b"pts-repl> ")
                .expect("Failed to write line");
            io::stdout().flush().expect("flush failed!");
            io::stdin()
                .read_line(&mut user_io)
                .expect("Failed to read line");

            match Repl::parse(&cmd, user_io, files) {
                Ok(..) => (),
                Err(..) => break,
            }
            user_io = "".to_string();
        }
    }

    pub fn start_without_data(cmd: &Commandline) {
        let mut files = Vec::new();
        Repl::run_repl(&cmd, &mut files);
    }

    pub fn start(cmd: &Commandline, _dataset: &DataSet) {
        let dataset = _dataset.clone();
        let mut files = vec![dataset];
        Repl::run_repl(&cmd, &mut files);
    }
}
