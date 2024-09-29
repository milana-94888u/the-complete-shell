use std::cmp;

#[derive(Clone, Debug)]
struct ShellIntRange {
    start: i64,
    end: i64,
    step: i64,
    alignment: usize,
}

#[derive(Clone, Debug)]
struct ShellCharRange {
    start: u8,
    end: u8,
    step: i64,
}

#[derive(Clone, Debug)]
pub enum ShellRange {
    Int(ShellIntRange),
    Char(ShellCharRange),
}

impl ShellRange {
    pub fn get_int_create_func() -> fn(&Vec<u8>, &Vec<u8>, &Vec<u8>) -> Option<ShellRange> {
        ShellIntRange::create_validated
    }

    pub fn get_char_create_func() -> fn(&Vec<u8>, &Vec<u8>, &Vec<u8>) -> Option<ShellRange> {
        ShellCharRange::create_validated
    }

    pub fn expand(&self) -> Vec<Vec<u8>> {
        match self {
            ShellRange::Int(range) => range.expand(),
            ShellRange::Char(range) => range.expand(),
        }
    }
}

impl ShellIntRange {
    fn count_int_align(num: &Vec<u8>) -> usize {
        let mut result = 0usize;
        let mut iter = num.into_iter();
        match iter.next() {
            Some(b'-') => {},
            Some(b'0') => {result = 1},
            Some(_) => return 0usize,
            None => unreachable!(),
        };
        while let Some(b'0') = iter.next() {
            result += 1;
        }
        if result > 0 {
            num.len()
        } else {
            0usize
        }
    }

    fn create_validated(start: &Vec<u8>, end: &Vec<u8>, step: &Vec<u8>) -> Option<ShellRange> {
        let alignment = cmp::max(Self::count_int_align(start), Self::count_int_align(end));
        let start = String::from_utf8(start.clone()).unwrap().parse::<i64>().unwrap();
        let end = String::from_utf8(end.clone()).unwrap().parse::<i64>().unwrap();
        let step = String::from_utf8(step.clone()).unwrap().parse::<i64>().unwrap();
        let step = match step.checked_abs() {
            Some(value) => value,
            None => return None,
        };
        if (end - start) / step + 1 > 2147483645i64 {
            None
        } else {
            Some(ShellRange::Int(
                Self {start, end, step, alignment}
            ))
        }
    }

    fn expand(&self) -> Vec<Vec<u8>> {
        let mut result = vec![];
        let range = if self.start < self.end {
            self.start..=self.end
        } else {
            self.end..=self.start
        };
        for i in range.step_by(self.step as usize) {
            result.push(format!("{:01$}", i, self.alignment).as_bytes().to_vec())
        }
        result
    }
}

impl ShellCharRange {
    fn create_validated(start: &Vec<u8>, end: &Vec<u8>, step: &Vec<u8>) -> Option<ShellRange> {
        let step = String::from_utf8(step.clone()).unwrap().parse::<i64>().unwrap();
        Some(ShellRange::Char(
            Self {start: *start.first().unwrap(), end: *end.first().unwrap(), step}
        ))
    }

    fn expand(&self) -> Vec<Vec<u8>> {
        let mut result = vec![];
        let range = if self.start < self.end {
            self.start..=self.end
        } else {
            self.end..=self.start
        };
        for i in range.step_by(self.step as usize) {
            result.push(vec![i]);
        }
        result
    }
}
