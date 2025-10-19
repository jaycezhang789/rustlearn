pub fn pick_longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

pub fn longest_word<'a>(sentence: &'a str) -> Option<&'a str> {
    let mut longest: Option<&'a str> = None;
    for word in sentence.split_whitespace() {
        match longest {
            Some(current) if current.len() >= word.len() => {}
            _ => longest = Some(word),
        }
    }
    longest
}

#[allow(dead_code)]
pub struct Article<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

impl<'a> Article<'a> {
    pub fn new(title: &'a str, body: &'a str) -> Self {
        Self { title, body }
    }

    pub fn teaser(&self) -> &'a str {
        const LIMIT: usize = 30;
        if self.title.len() <= LIMIT {
            self.title
        } else {
            // 找到第 LIMIT 个字符的边界，避免 UTF-8 截断。
            let mut end = self.title.len();
            let mut count = 0;
            for (idx, _) in self.title.char_indices() {
                if count == LIMIT {
                    end = idx;
                    break;
                }
                count += 1;
            }
            if count < LIMIT {
                self.title
            } else {
                &self.title[..end]
            }
        }
    }

    pub fn body_length(&self) -> usize {
        self.body.len()
    }
}

pub fn find_match<'a, F>(items: &'a [&'a str], predicate: F) -> Option<&'a str>
where
    F: Fn(&str) -> bool,
{
    for &item in items {
        if predicate(item) {
            return Some(item);
        }
    }
    None
}

pub fn shortest_word<'a>(sentence: &'a str) -> Option<&'a str> {
    let mut shortest: Option<&'a str> = None;
    for word in sentence.split_whitespace() {
        match shortest {
            Some(current) if current.len() <= word.len() => {}
            _ => shortest = Some(word),
        }
    }
    shortest
}

pub fn longest_with_note<'a>(left: &'a str, right: &'a str, note: &str) -> &'a str {
    println!("{note}");
    pick_longest(left, right)
}
