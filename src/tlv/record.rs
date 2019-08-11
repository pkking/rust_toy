use std::fmt::Debug;

#[derive(Debug)]
pub struct item<T:Debug> {
    key: String,
    value: T,
}

#[derive(Debug)]
pub struct record<T:Debug> {
    items: Vec<item<T>>,
}