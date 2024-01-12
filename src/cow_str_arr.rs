use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq)]
pub struct CowStrArray<'a> {
    words: Cow<'a, [&'a str]>,
}

impl<'a> From<&'a [&'a str]> for CowStrArray<'a> {
    fn from(s: &'a [&'a str]) -> CowStrArray<'a> {
        Self {
            words: Cow::Borrowed(s),
        }
    }
}
impl<'a> From<&'a Vec<&'a str>> for CowStrArray<'a> {
    fn from(s: &'a Vec<&'a str>) -> CowStrArray<'a> {
        Self {
            words: Cow::Borrowed(s),
        }
    }
}
impl<'a> From<Vec<&'a str>> for CowStrArray<'a> {
    fn from(s: Vec<&'a str>) -> CowStrArray<'a> {
        Self {
            words: Cow::Owned(s),
        }
    }
}
impl<'a> From<&'a [String]> for CowStrArray<'a> {
    fn from(s: &'a [String]) -> CowStrArray<'a> {
        let words: Vec<&str> = s.iter().map(|e| e.as_str()).collect();
        Self {
            words: Cow::Owned(words),
        }
    }
}
impl<'a> From<&'a Vec<String>> for CowStrArray<'a> {
    fn from(s: &'a Vec<String>) -> CowStrArray<'a> {
        let words: Vec<&str> = s.iter().map(|e| e.as_str()).collect();
        Self {
            words: Cow::Owned(words),
        }
    }
}
/*
impl<'a> From<Vec<String>> for CowStrArray<'a> {
    fn from(s: Vec<String>) -> CowStrArray<'a> {
        let words: Vec<&str> = s.iter().map(|e| e.as_str()).collect();
        Self {
            words: Cow::Owned(s.iter()),
        }
    }
}
*/
/*
impl<'a, E: AsRef<str>, T: AsRef<[E]>> From<T> for CowStrArray<'a> {
    fn from(value: T) -> Self {
        Self {
            words: value.as_ref().into(),
        }
    }
}
*/
