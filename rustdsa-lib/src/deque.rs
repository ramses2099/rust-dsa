#![allow(dead_code)]
use std::iter::Iterator;

#[derive(Debug)]
pub struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    pub fn new(cap: usize) -> Self {
        Self { cap: cap, data: Vec::with_capacity(cap) }
    }
    //
    pub fn is_empty(&self) -> bool {
        0 == self.len()
    }
    //
    pub fn len(&self) -> usize {
        self.data.len()
    }
    //
    pub fn is_full(&self) -> bool {
        self.len() == self.cap
    }
    //
    pub fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap);
    }
    // use the tail of a vec as the start of the deque
    pub fn add_front(&mut self, val: T) -> Result<(), String> {
        if self.len() == self.cap {
            return Err("No space avaliable".to_string());
        }
        self.data.push(val);

        Ok(())
    }
    //rear = back of the deque
    // the head of the ve is the tail of the deque
    pub fn add_rear(&mut self, val: T) -> Result<(), String> {
        if self.len() == self.cap {
            return Err("No space avaliable".to_string());
        }
        self.data.insert(0, val);

        Ok(())
    }
    // remove data from the queue head
    pub fn remove_front(&mut self) -> Option<T> {
        if self.len() > 0 { Some(self.data.pop().unwrap()) } else { None }
    }
    // remove data from the queue head
    pub fn remove_rear(&mut self) -> Option<T> {
        if self.len() > 0 { Some(self.data.remove(0)) } else { None }
    }
    // Implementation of iteration for the deque
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    //
    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }
    //
    pub fn iter_mut(&mut self) -> IterMut<T>{
        let mut iterator = IterMut{stack: Vec::new()};
        for item in self.data.iter_mut(){
            iterator.stack.push(item);
        }
        iterator
    }
}

// Implementing 3 Iterator
pub struct IntoIter<T>(Deque<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() { Some(self.0.data.remove(0)) } else { None }
    }
}
//
pub struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if 0 != self.stack.len() { Some(self.stack.remove(0)) } else { None }
    }
}
//
pub struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if 0 != self.stack.len() { Some(self.stack.remove(0)) } else { None }
    }
}
