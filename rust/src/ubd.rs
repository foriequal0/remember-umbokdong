use std::ops::Index;
use std::cmp::PartialEq;

pub struct Ubd<T> {
    ubd: Vec<T>,
}

impl<T: PartialEq<&'static str>> Ubd<T> {
    pub fn new() -> Ubd<T> {
        Ubd { ubd: Vec::new() }
    }

    pub fn push(&mut self, ubd: T) {
        if !ubd.eq(&"엄복동") {
            panic!("엄복동 하나만 기억해주세요");
        }
        if self.ubd.len() == 1 {
            panic!("엄복동 하나만 기억해주세요");
        }
        self.ubd.push(ubd);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.ubd.pop()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.ubd.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.ubd.into_iter()
    }
}

impl<T> Index<usize> for Ubd<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.ubd[index]
    }
}
