use barehand::html::Html;

// #[derive(Debug)]
// struct Tag<'a> {
//     data: &'a str,
//     parent: usize,
//     children: Vec<usize>,
// }

fn main() {
    let html = std::fs::read_to_string("src/index.html").unwrap();
    let html = Html::new(&html);
    html.print_tree();

    // let (head, body) = prepare(&html);
    // println!("{:?}", head);
    // println!("{:?}", body);
    // analyis(&head, &body);
}

// fn prepare(html: &String) -> (Vec<&str>, Vec<&str>)
// {
//     let head_start = html.find("<head>").unwrap() + "<head>".len();
//     let head_end = head_start + html.get(head_start..).unwrap().find("</head>").unwrap() - 1;
//     let body_start = head_end + html.get(head_end..).unwrap().find("<body>").unwrap() + "<body>".len();
//     let body_end = body_start + html.get(body_start..).unwrap().find("</body>").unwrap() - 1;

//     let head: Vec<&str> = html[head_start..=head_end]
//         .split('<')
//         .map(|line| line.trim())
//         .skip_while(|line| line.is_empty())
//         .collect();

//     let body: Vec<&str> = html[body_start..=body_end]
//         .split('<')
//         .map(|line| line.trim())
//         .skip_while(|line| line.is_empty())
//         .collect();

//     return (head, body)
// }

// fn analyis(head: &Vec<&str>, body: &Vec<&str>)
// {
//     let &title = head
//         .iter()
//         .find(|&&line| line.starts_with("title>"))
//         .unwrap();

//     let title = title.get("title>".len()..).unwrap();
//     println!("-- title: {}", title);

//     let root = Tag {
//         data: "body>",
//         parent: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
//         children: vec![],
//     };


//     let mut tree = vec![root];
//     let mut stack = vec![0];

//     for line in body
//     {
//         let cur_tag: Tag;

//         if !line.starts_with("img")
//         {
//             if !line.starts_with("/")
//             {
//                 cur_tag = Tag {
//                     data: line,
//                     parent: *stack.last().unwrap(),
//                     children: vec![],
//                 };

//                 let tree_len = tree.len();

//                 tree.push(cur_tag);
                
//                 tree[*stack.last().unwrap()].children.push(tree_len);

//                 stack.push(tree_len);
//             }
//             else
//             {
//                 stack.pop();
//             }
//         }
//         else
//         {
//             cur_tag = Tag {
//                 data: line,
//                 parent: *stack.last().unwrap(),
//                 children: vec![],
//             };

//             let tree_len = tree.len();

//             tree.push(cur_tag);

//             tree[*stack.last().unwrap()].children.push(tree_len);

//             stack.push(tree_len);
//         }
//     }

//     print_tree(tree.first().unwrap(), 0, &tree);
// }

// fn print_tree<'a>(root: &Tag<'a>, spaces: usize, tree: &Vec<Tag<'a>>)
// {
//     println!("{}{}", " ".repeat(spaces), root.clone().data);

//     for child in &root.children
//     {
//         print_tree(&tree[*child], spaces + 1, tree);
//     }
// }