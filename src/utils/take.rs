pub trait Take {
    fn take(&mut self, length: usize) -> String;
    fn take_slice(&self, start: usize, end: usize) -> Option<&str>;
    fn contains_special_characters(&self, length: usize) -> bool ; }

   

impl Take for String {
    fn take_slice(&self, start: usize, end: usize) -> Option<&str> {
        let s: &str = self.as_str();

        if end <= s.len() {
            let mut iter = s
                .char_indices()
                .map(|(pos, _)| pos)
                .chain(Some(s.len()))
                .skip(start)
                .peekable();
            let start_pos = *iter.peek()?;

            for _ in start..end {
                iter.next();
            }
            Some(&s[start_pos..*iter.peek()?])
        } else {
            Some(&s)
        }
    }

        fn contains_special_characters(&self, length: usize) -> bool {
        match self.take_slice(0, length) {
            None => false,
            Some(s) => s.contains("ä")
                || s.contains("ü")
                || s.contains("ö")
                || s.contains("Ä")
                || s.contains("Ü")
                || s.contains("Ö")
        }
    }

    fn take(&mut self, length: usize) -> String {
        let mut len = length;
        if self.contains_special_characters(length) {
            len = length + self.len() - self.chars().count();
        }
        match self.take_slice(0, length) {
            Some(string) => {
                let mut res: String = String::from(string);
                if res.len() < len {
                    res += &std::iter::repeat(" ")
                        .take(len - res.len())
                        .collect::<String>();
                }
                return res;
            }
            None => String::from("TAKEERROR"),
        }
    }

    /*
        fn take(&mut self, length: usize) -> String {
            let len = length + self.len() - self.chars().count();
            // if len != length {
            //     println!("{:?} len: {}", self, self.len());
            //     println!("length: {} length meassured: {}", len, length);
            // }

            if self.len() < len {
                self.to_string() + &" ".repeat(len - self.len())
            } else if self.len() > len {
                match self.take_slice(0, len) {
                    Some(s) => s.to_string(),
                    None => "TAKEERROR".to_string(),
                }
            } else {
                self.to_string()
            }

            // match self.take_slice(0, length) {
            //     Some(string) => {
            //         let mut res: String = String::from(string);
            //         if res.len() < len {
            //             res += &std::iter::repeat(" ")
            //                 .take(len - res.len())
            //                 .collect::<String>();
            //         }
            //         return res;
            //     }
            //     None => {
            //         //println!("{:?}", self);
            //         //String::from("TAKEERROR")
            //         self.to_string()
            //     }
            // }
    }
        */
}
