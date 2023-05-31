use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
pub struct Attributes<'a>
{
    pub data: HashMap<&'a str, Option<&'a str>>,
}

impl<'a> Attributes<'a>
{
    pub fn new(attributes_raw: &'a str) -> Attributes<'a>
    {
        Attributes { data: Self::parse(attributes_raw) }
    }

    pub fn new_empty() -> Attributes<'a>
    {
        Attributes { data: HashMap::<&'a str, Option<&'a str>>::new() }
    }

    fn parse(attributes_raw: &'a str) -> HashMap<&'a str, Option<&'a str>>
    {
        // group 1: key
        // group 2 or 3: value
        // group 4: key without value
        let re = Regex::new("\\s?(?:([a-z-_]+)=(?:\"([^\"]+)\"|'([^']+)'))|([a-z-_]+)").unwrap();
        let re_captures = re.captures_iter(attributes_raw);
        let mut attributes_map = HashMap::<&'a str, Option<&'a str>>::new();
        for capture in re_captures
        {
            if let Some(data) = capture.get(1)
            {
                let key = data.as_str();
                let value = capture.get(2).unwrap_or_else(|| capture.get(3).unwrap()).as_str();
                attributes_map.insert(key, Some(value));
            }
            else
            {
                let key = capture.get(4).unwrap().as_str();
                attributes_map.insert(key, None);
            }
        }
        attributes_map
    }
}