pub trait Take {
    fn take(&mut self, length: usize) -> String;
    fn take_slice(&self, start: usize, end: usize) -> Option<&str>;
}

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

    fn take(&mut self, length: usize) -> String {
        match self.take_slice(0, length) {
            Some(string) => String::from(string),
            None => String::from("TAKEERROR"),
        }
    }
}
