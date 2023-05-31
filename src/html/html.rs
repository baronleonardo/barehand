use super::Element;
use super::is_singleton_tag;

pub struct Html<'a> {
    title: &'a str,
    tree: Vec<Element<'a>>
}

impl<'a> Html <'a>
{
    pub fn new(html: &'a String) -> Html
    {
        let prepared_html = Self::prepare(html);
        let html = Self::build_tree(&prepared_html);

        html
    }

    fn prepare(html: &String) -> Vec<&str>
    {
        let res: Vec<&str> = html
            .split('<')
            .map(|line| line.trim())
            .skip_while(|line| line.is_empty() || line.starts_with("!"))
            .collect();

        return res
    }

    fn build_tree(html_splitted: &Vec<&'a str>) -> Html<'a>
    {
        let &title = html_splitted
            .iter()
            .find(|&&line| line.starts_with("title>"))
            .unwrap();

        let title = title.get("title>".len()..).unwrap_or_default();

        let root = Element::new(
            "",
            Element::PARENTLESS
        );

        let mut tree = vec![root];
        let mut stack = vec![0];

        for line in html_splitted
        {
            let cur_tag: Element;

            if !is_singleton_tag(line)
            {
                if !line.starts_with("/")
                {
                    cur_tag = Element::new(line, *stack.last().unwrap());

                    let tree_len = tree.len();

                    tree.push(cur_tag);
                    
                    tree[*stack.last().unwrap() as usize].children.push(tree_len);

                    stack.push(tree_len as isize);
                }
                else
                {
                    stack.pop();
                }
            }
            else
            {
                cur_tag = Element::new(line, *stack.last().unwrap());

                let tree_len = tree.len();

                tree.push(cur_tag);

                tree[*stack.last().unwrap() as usize].children.push(tree_len);
            }
        }

        Html { tree, title }
    }

    pub fn print_tree(&self)
    {
        Self::print_tree_rec(self.tree.first().unwrap(), 0, &self.tree);
    }

    fn print_tree_rec(root: &Element<'a>, spaces: usize, tree: &Vec<Element<'a>>)
    {
        println!("{}{}", "|".repeat(spaces), root.clone().data);

        for child in &root.children
        {
            Self::print_tree_rec(&tree[*child], spaces + 1, tree);
        }
    }
}