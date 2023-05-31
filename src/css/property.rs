use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
pub struct Properties<'a>
{
    map: HashMap<&'a str, &'a str>,
}

impl<'a> Properties<'a>
{
    pub fn new(properties_raw: &'a str) -> Properties<'a>
    {
        Properties { map: Self::parse(properties_raw) }
    }

    pub fn new_empty() -> Properties<'a>
    {
        Properties { map: Default::default() }
    }

    fn parse(properties_raw: &'a str) -> HashMap<&'a str, &'a str>
    {
        let mut res = HashMap::<&'a str, &'a str>::new();

        let re = Regex::new(r"([^:\s]+)\s*:\s*([^$;]+)").unwrap();

        let captures = re.captures_iter(properties_raw);
        for capture in captures
        {
            let key = capture.get(1).unwrap();
            let value = capture.get(2).unwrap();

            res.insert(key.as_str(), value.as_str());
        }

        res
    }
}