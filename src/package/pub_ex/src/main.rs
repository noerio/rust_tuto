#![allow(unused_imports)]
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

// FILL in the blank in two ways
// DON'T add new code line
use std::collections::*;
use std::collections::{HashMap, BTreeMap, HashSet};

fn ex2() {
    let _c1:HashMap<&str, i32> = HashMap::new();
    let mut c2 = BTreeMap::new();
    c2.insert(1, "a");
    let _c3: HashSet<i32> = HashSet::new();
}
