use std::borrow::{Cow, Borrow};

use regex::Regex;

use super::Block;

pub struct Css<'a>
{
    // tree: Vec<Rule<'a>>,
    blocks: Vec<Block<'a>>,
}

impl<'a> Css<'a>
{
    pub fn new(css: &'a String) -> Css<'a>
    {
        let prepared_css = Self::prepare(css);
        let css = Self::build_blocks(&prepared_css);

        css
    }

    fn prepare(css: &str) -> Vec<&str>
    {
        // let re = Regex::new(r"\/\*[^\*]+\*\/").unwrap();
        // let removed_comments_css = re.split(css);
        let blocks_raw: Vec<&str> = css
            .split('}')
            .map(|line| line.trim())
            .skip_while(|line| line.is_empty())
            .collect();

        blocks_raw
    }

    fn build_blocks(prepared_css: &Vec<&'a str>) -> Css<'a>
    {
        let mut blocks = Vec::<Block<'a>>::new();

        for line in prepared_css
        {
            if !line.is_empty()
            {
                let block = Block::new(line);
                println!("{:?}", block);
                blocks.push(block);
            }
        }

        Css { blocks }
    }
}

#[cfg(test)]
mod tests
{
    use super::Css;

    #[test]
    fn test_css()
    {
        let css_raw = std::fs::read_to_string("src/style.css").unwrap();
        let Css = Css::new(&css_raw);
    }
}