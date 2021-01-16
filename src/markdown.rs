use std::{cmp::Ordering, collections::VecDeque};

use pulldown_cmark::{CowStr, Event, LinkType, Parser, Tag};


/// InterlinkParser is an adapter for Parser to implement interlinks via `[[id]]` notation
///
/// The adapter looks for patterns of '[', '[', text, ']', ']' which is how this notation 
/// is currently parsed, and converts it to collapsed link events
struct InterlinkParser<'a> {
    parser: Parser<'a>,
    store: VecDeque<Event<'a>>,
}

impl <'a> InterlinkParser<'a> {
    pub fn new(parser: Parser<'a>) -> Self {
        InterlinkParser{parser, store: VecDeque::new()}
    }

    fn token_ok(&self, t: &CowStr) -> bool {
        match self.store.len() {
            n if n<2 => "[".cmp(&t) == Ordering::Equal,
            n if n==2 => true,
            n if ((n>2) && (n<5)) => "]".cmp(&t) == Ordering::Equal,
            _ => false,
        }
    }
}

impl <'a>  Iterator for InterlinkParser<'a> {
    type Item = Event<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.store.is_empty() {
            return self.store.pop_back()
        }

        loop {
            if self.store.len()==5 {
                if let Some(Event::Text(t)) = self.store.remove(2) {
                    self.store.clear();
                    let url: CowStr = format!("internal:{}", t).into();
                    self.store.push_front(Event::Text(t));
                    self.store.push_front(Event::End(Tag::Link(LinkType::Collapsed, "".into(), "".into())));
                    break Some(Event::Start(Tag::Link(LinkType::Collapsed, url, "".into())));
                } else {
                    panic!("Internal error");
                }
            }
            let n = self.parser.next();
            match &n {
                Some(Event::Text(t)) if self.token_ok(&t) => self.store.push_front(n.unwrap()),
                _ => break n
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pulldown_cmark::{html};
    #[test]
    fn it_works() {
        let s = "This is markdown with [[interlink]]!";
        let parser = InterlinkParser::new(Parser::new(s));
        let mut res = String::new();
        html::push_html(&mut res, parser);
        assert_eq!(res, "<p>This is markdown with <a href=\"internal:interlink\">interlink</a>!</p>\n");
    }
}