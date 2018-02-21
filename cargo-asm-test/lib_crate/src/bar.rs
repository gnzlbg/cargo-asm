pub const N: usize = 3;

pub fn max_array(x: &mut[f64; 65536], y: &[f64; 65536]) {
    for i in 0..65536 {
        x[i] = if y[i] > x[i] { y[i] } else { x[i] };
    }
}

#[inline(never)]
pub fn generic_add<T: ::std::ops::Add<T,Output=T>>(x: T, y: T) -> T { x + y }
pub fn generic_add2<T: ::std::ops::Add<T,Output=T>>(x: T, y: T) -> T { x + y }
pub fn generic_mul2<T: ::std::ops::Mul<T,Output=T>>(x: T, y: T) -> T { x * y }
#[inline(never)]
pub fn generic_mul<T: ::std::ops::Mul<T,Output=T>>(x: T, y: T) -> T { x * y }


pub fn add(x: usize, y: usize) -> usize {
    let z = generic_add(x, y);
    let b = x + z;
    let q = 2 * b / z;
    let f = 13 * x + 15 * b + 3 * y + 200 % q;
    f
}

pub fn simple_add(x: usize, y: usize) -> usize { x + y }

pub fn simple_add2(x: usize, y: usize) -> usize {
    x + y
}


pub fn add_and_mul(x: usize, y: usize) -> usize {
    let w = generic_add2(x, y);
    let z = generic_mul2(x, y);
    w + z
}

pub fn double_and_sort(x: &mut [usize]) {
    for x in x.iter_mut() {
        *x *= 2;
    }
    x.sort_unstable();
}

