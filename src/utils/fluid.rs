use csv::ReaderBuilder;
use std::collections::HashMap;

fn load_database(path: String) -> Vec<HashMap<String, String>> {
    let mut vec = Vec::new();
    let rdr = ReaderBuilder::new()
        .delimiter(b';')
        //.from_path("../robby/uhd_fluid_database.csv");
        .from_path(path);

    match rdr {
        Err(..) => (),
        Ok(mut rdr2) => {
            for result in rdr2.deserialize() {
                match result {
                    Err(..) => (),
                    Ok(result) => {
                        let record: HashMap<String, String> = result;
                        vec.push(record);
                    }
                }
            }
        }
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
        self.database = load_database(path);
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        if *&self.database.len() >= 3 {
            for e in &self.database[0..3] {
                println!("{:?}", e);
            }
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
