pub struct StrSplit<'a, D> {
    remainder: Option<&'a str>,
    delimiter: D,
}

impl<'a, D> StrSplit<'a, D> {
    pub fn new(haystack: &'a str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

// 因为没用到'b这个生命周期, 所以可以忽略
// impl<'a, 'b> Iterator for StrSplit<'a, 'b> {
// impl<'a> Iterator for StrSplit<'a, '_> {

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'a, D> Iterator for StrSplit<'a, D>
where
    D: Delimiter,
{
    type Item = &'a str;

    // Example:
    // remainder = "a b c d"
    // next() -> return a and then remainder = "b c d"
    // next() -> return b and then remainder = "c d"
    // next() -> return c and then remainder = "d"
    // next() -> return d and then remainder = ""
    // next() -> return None
    fn next(&mut self) -> Option<Self::Item> {
        let Some(ref mut remainder /* &mut &'a str */) = self.remainder /* Option<&'a str> */ else {
            return None;
        };
        let Some((start, end)) = self.delimiter.find_next(remainder) else {
            return self.remainder.take();
        };

        let until_delimiter = &remainder[..start];
        *remainder = &remainder[end..];
        Some(until_delimiter)
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8())) // char必须为utf8长度(有别国的文字)
    }
}

#[allow(dead_code)]
fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c).next().unwrap()
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell")
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn empty_tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
