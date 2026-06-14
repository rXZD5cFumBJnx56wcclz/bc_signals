#![allow(non_camel_case_types)]

use std::cell::RefCell;

use bc_utils::other::roll_slice1;
use bc_utils_lg::types::maps::MAP;

pub type BF_SIGNALS<'a> = RefCell<Vec<MAP<&'a str, Vec<Vec<f64>>>>>;

pub trait BfSignalsExt<'a> {
    fn new<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (&'a str, Vec<Vec<f64>>)>;
    fn insert(&self, index_: usize, key: &'a str, value: Vec<Vec<f64>>);
    fn roll_and_replace(&self, shift: i32, index_: usize, key: &'a str, value: Vec<f64>);
}

impl<'a> BfSignalsExt<'a> for BF_SIGNALS<'a> {
    fn new<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (&'a str, Vec<Vec<f64>>)>,
    {
        RefCell::new(vec![MAP::from_iter(iter)])
    }
    fn insert(&self, index_: usize, key: &'a str, value: Vec<Vec<f64>>) {
        self.borrow_mut()
            .get_mut(index_)
            .unwrap()
            .insert(key, value);
    }
    fn roll_and_replace(&self, shift: i32, index_: usize, key: &'a str, value: Vec<f64>) {
        roll_slice1(
            self.borrow_mut()
                .get_mut(index_)
                .unwrap()
                .get_mut(key)
                .unwrap()
                .as_mut_slice(),
            &shift,
        );
        self.borrow_mut()
            .get_mut(index_)
            .unwrap()
            .entry(key)
            .and_modify(|v| {
                let bind = v.len();
                v[(bind as i32 + shift) as usize] = value;
            });
    }
}
