use std::{collections::HashMap};

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Vec<&'buf str>>
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Vec<&str>> {
        self.data.get(key)
    }
}

// query example
// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for kv in s.split('&') {
            if let Some(i) = kv.find('=') {
                let key = &kv[..i];
                let val = &kv[i + 1..];

                data.entry(key)
                .and_modify(|vec: &mut Vec<&'buf str>| vec.push(val))
                .or_insert(vec![val]);
            }
        }

        QueryString {data}
    }
}
