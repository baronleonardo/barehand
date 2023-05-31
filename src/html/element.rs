use regex::Regex;

use super::attribute::Attributes;

#[derive(Debug)]
pub struct ElementParts<'a> {
    pub tag: &'a str,
    pub attributes: Attributes<'a>,
    pub value: &'a str,
}

impl<'a> ElementParts<'a>
{
    fn new_empty() -> ElementParts<'a>
    {
        ElementParts {
            tag: "",
            attributes: Attributes::new_empty(),
            value: ""
        }
    }

//     fn new(tag: &'a str, value: &'a str) -> ElementParts<'a>
//     {
//         ElementParts {
//             tag,
//             attributes: Attributes::new_empty(),
//             value
//         }
//     }
}

pub type ElementParent = isize;
pub type ElementChild = usize;

#[derive(Debug)]
pub struct Element<'a> {
    pub data: &'a str,
    pub parts: ElementParts<'a>,
    pub parent: ElementParent,
    pub children: Vec<ElementChild>,
}

impl<'a> Element<'a>
{
    pub const PARENTLESS: isize = -1;

    pub fn new(data: &'a str, parent: ElementParent) -> Element<'a>
    {
        Element {
            data,
            parts: Self::parse(data),
            parent,
            children: Vec::<ElementChild>::new(),
        }
    }

    fn parse(element_data: &'a str) -> ElementParts<'a>
    {
        if !element_data.is_empty()
        {
            let re = Regex::new("^([a-z]+)\\s*([^>]+)?>([^$]+)?").unwrap();
            let re_captures = re.captures_at(element_data, 0).unwrap();

            let tag = re_captures.get(1).unwrap().as_str();
            let value = re_captures.get(3).map(|capture| capture.as_str()).unwrap_or_default();

            let attributes = 
                if let Some(attributes) = re_captures.get(2)
                { 
                    Attributes::new(attributes.as_str())
                }
                else
                {
                    Attributes::new_empty()
                };

            ElementParts { tag, attributes, value }
        }
        else
        {
            ElementParts::new_empty()
        }
    }
}