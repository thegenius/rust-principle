mod cow_str_arr;
use cow_str_arr::CowStrArray;
use std::{borrow::Cow, marker::PhantomData};

pub fn compare_cow_str<'a>(a: impl Into<Cow<'a, str>>, b: impl Into<Cow<'a, str>>) -> bool {
    a.into() == b.into()
}

pub fn compare_as_ref_str(a: impl AsRef<str>, b: impl AsRef<str>) -> bool {
    a.as_ref() == b.as_ref()
}

pub fn compare_into_str(a: impl Into<String>, b: impl Into<String>) -> bool {
    a.into() == b.into()
}

/*
pub fn compare_into_str_arr<S, T>(a: &[S], b: &[T]) -> bool
where
    S: Into<String>,
    T: Into<String>,
{
    let cnt = a.len();
    if cnt != b.len() {
        return false;
    }
    for i in 0..cnt {
        // cannot move out of type `[S]`
        let a_element: String = a[i].into();
        let b_element: String = b[i].into();
        if a_element != b_element {
            return false;
        }
    }
    return true;
}
*/

pub fn compare_as_ref_str_arr<S, T>(a: &[S], b: &[T]) -> bool
where
    S: AsRef<str>,
    T: AsRef<str>,
{
    let cnt = a.len();
    if cnt != b.len() {
        return false;
    }
    for i in 0..cnt {
        let a_element = a[i].as_ref();
        let b_element = b[i].as_ref();
        if a_element != b_element {
            return false;
        }
    }
    true
}

#[derive(Debug, PartialEq, Eq)]
pub struct Content<'a> {
    data: Cow<'a, str>,
}

impl<'a> Content<'a> {
    pub fn new(data: impl Into<Cow<'a, str>>) -> Self {
        Self { data: data.into() }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Article<E, T: AsRef<[E]> + AsMut<[E]>> {
    words: T,
    _phantom: PhantomData<E>,
}

impl<E, T: AsRef<[E]> + AsMut<[E]>> Article<E, T> {
    pub fn new(words: T) -> Self {
        Self {
            words,
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Poem<'a, E: Into<Cow<'a, str>>, T: AsRef<[E]> + AsMut<[E]>> {
    words: T,
    _phantom: PhantomData<&'a E>,
}

impl<'a, E: Into<Cow<'a, str>>, T: AsRef<[E]> + AsMut<[E]>> Poem<'a, E, T> {
    pub fn new(words: T) -> Self {
        Self {
            words,
            _phantom: PhantomData,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Epic<'a, T: AsRef<[Cow<'a, str>]> + AsMut<[Cow<'a, str>]>> {
    words: T,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, T: AsRef<[Cow<'a, str>]> + AsMut<[Cow<'a, str>]>> Epic<'a, T> {
    pub fn new(words: T) -> Self {
        Self {
            words,
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Legendary<'a> {
    words: CowStrArray<'a>,
}

impl<'a> Legendary<'a> {
    pub fn from_str_arr(words: &'a [&'a str]) -> Self {
        Self {
            words: words.into(),
        }
    }
    pub fn from_string_arr(words: &'a [String]) -> Self {
        Self {
            words: words.into(),
        }
    }
}

fn main() {
    let result = compare_into_str("hello", "hello".to_string());
    assert!(result);

    let result = compare_as_ref_str("hello", "hello".to_string());
    assert!(result);

    let result = compare_cow_str("hello", "hello".to_string());
    assert!(result);

    let a_arr = vec!["a", "b"];
    let b_arr = vec!["a".to_string(), "b".to_string()];
    let result = compare_as_ref_str_arr(&a_arr, &b_arr);
    assert!(result);

    let a_arr = ["a", "b"];
    let b_arr = vec!["a".to_string(), "b".to_string()];
    let result = compare_as_ref_str_arr(&a_arr, &b_arr);
    assert!(result);

    let content_a = Content::new("hello");
    let content_b = Content::new("hello".to_string());
    assert_eq!(content_a, content_b);

    let article_a = Article::new(["a", "b", "c"]);
    let article_b = Article::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    // article_a and article_b is two different type, so you cannot compare them!
    //assert_eq!(article_a, article_b);

    let poem_a = Poem::new(["a", "b", "c"]);
    let poem_b = Poem::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    // poem_a and poem_b is also tow different type!
    //assert_eq!(poem_a, poem_b);

    let epic_words: Vec<Cow<'_, str>> = ["a", "b", "c"].into_iter().map(Cow::from).collect();
    let epic_a = Epic::new(epic_words);
    let epic_words = ["a".to_string(), "b".to_string(), "c".to_string()]
        .into_iter()
        .map(Cow::from)
        .collect();
    let epic_b = Epic::new(epic_words);
    assert_eq!(epic_a, epic_b);

    let lengendary_a = Legendary::from_str_arr(&["a"]);
    let vec_string = vec!["a".to_string()];
    let lengendary_b = Legendary::from_string_arr(&vec_string);
    assert_eq!(lengendary_a, lengendary_b);

    println!("Hello, world!");
}
