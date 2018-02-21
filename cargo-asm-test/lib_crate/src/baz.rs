use super::bar;

pub fn add_and_mul(x: usize, y: usize) -> usize {
    let w = bar::generic_add(x, y);
    let z = bar::generic_mul(x, y);
    w + z
}

pub struct Foo {
    x: usize,
    y: usize,
}

impl Foo {
    pub fn foo_add(&self) -> usize {
        self.x + self.y
    }
}

pub trait Addd {
    fn addd(&self) -> usize;
}

impl Addd for Foo {
    fn addd(&self) -> usize {
        self.foo_add()
    }
}

pub fn barr(x: usize,
            y: usize,
            z: usize) -> usize {
    x + y + z
}

pub fn double(x: &mut usize) {
    *x *= 2;
}
