use csv::Reader;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::File;

fn read_csv(path: &String) -> Option<Reader<File>> {
    let mut result = None;
    for delimiter in vec![b';', b',', b'\t'] {
        let rdr = ReaderBuilder::new().delimiter(delimiter).from_path(path);
        match rdr {
            Err(err) => {
                println!("{}: {}", "Error occured", err);
                break;
            }
            Ok(mut rdr) => {
                let headers = rdr.headers().ok()?;
                if headers.get(0)? == "Title" {
                    result = Some(rdr);
                    break;
                }
            }
        }
    }

    result
}

fn load_database(path: &String) -> Vec<HashMap<String, String>> {
    let mut vec = Vec::new();
    let rdr = read_csv(&path);

    match rdr {
        Some(mut rdr) => {
            for result in rdr.deserialize() {
                match result {
                    Err(..) => (),
                    Ok(result) => {
                        let record: HashMap<String, String> = result;
                        vec.push(record);
                    }
                }
            }
        }
        _ => (),
    }

    vec
}

pub struct Fluid {
    database: Vec<HashMap<String, String>>,
}

impl Fluid {
    pub fn init() -> Self {
        Fluid { database: vec![] }
    }

    pub fn load(&mut self, path: String) {
        self.database = load_database(&path);
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        if *&self.database.len() >= 3 {
            for e in &self.database[0..3] {
                println!("{:?}", e);
            }
        } else {
            println!("Error: Database has length {}", *&self.database.len());
        }
    }

    #[allow(dead_code)]
    pub fn query(&self, id: &str) -> Option<String> {
        for entry in &self.database {
            if entry["ContentId"] == id {
                return Some(format!("{}", entry["Filename"]));
            }
        }
        None
    }
}
