use super::Properties;

/// @TODO: handle '@'

#[derive(Debug)]
pub struct Block<'a>
{
    pub selectors: Vec<&'a str>,
    pub properties: Properties<'a>,
}

impl<'a> Block<'a>
{
    pub fn new(block_raw: &'a str) -> Block<'a>
    {
        let block_splitted: Vec<&str> = block_raw
            .split('{')
            .collect();

        let selectors: Vec<&str> = block_splitted[0]
            .trim()
            .split(',')
            .collect();

        let properties =
            if let Some(block_raw) = block_splitted.get(1)
            {
                Properties::new(block_raw)
            }
            else
            {
                Properties::new_empty()
            };

        Block {
            selectors, properties
        }
    }
}