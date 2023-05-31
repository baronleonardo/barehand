// This is 2d array
// rows -> represent from 'a' to 'z'
// cols -> represent singlton tags
pub const SINGLTON_TAGS: &'static [&'static [&'static str]] = &[
    &["area"],
    &["base", "br"],
    &["col", "command"],
    &[],
    &["embed"],
    &[],
    &[],
    &["hr"],
    &["img", "input"],
    &[],
    &["keygen"],
    &["link"],
    &["meta"],
    &[],
    &[],
    &["param"],
    &[],
    &[],
    &["source"],
    &["track"],
    &[],
    &[],
    &["wbr"],
    &[],
    &[],
    &[]
];

pub fn is_singleton_tag(line: &str) -> bool
{
    let first_letter = line.chars().nth(0).unwrap() as usize;
        if first_letter as usize >= 'a' as usize && first_letter as usize <= 'z' as usize
        {
            for elm in SINGLTON_TAGS[first_letter as usize - 'a' as usize]
            {
                if line.starts_with(elm)
                {
                    return true;
                }
            }
        }

        false
}