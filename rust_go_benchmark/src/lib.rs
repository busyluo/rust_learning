#![feature(test)]
extern crate test;
use std::collections::HashMap;

fn map_insert() {
    let mut map = HashMap::new();

    for i in 0..1000 {
        map.insert(i, "some values.".to_owned());
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use crate::*;

    #[bench]
    fn map_bench(b: &mut Bencher) {
        b.iter(|| map_insert());
    }
}