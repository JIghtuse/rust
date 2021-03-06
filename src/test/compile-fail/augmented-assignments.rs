// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(augmented_assignments)]

use std::ops::AddAssign;

struct Int(i32);

impl AddAssign for Int {
    fn add_assign(&mut self, _: Int) {
        unimplemented!()
    }
}

fn main() {
    let mut x = Int(1);
    x   //~ error: use of moved value: `x`
    +=
    x;  //~ note: `x` moved here because it has type `Int`, which is non-copyable

    let y = Int(2);
    y   //~ error: cannot borrow immutable local variable `y` as mutable
    +=
    Int(1);
}
