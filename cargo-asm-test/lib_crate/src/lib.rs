pub mod bar;

pub fn moooo(x: i32) -> i32 { x * 2 }

pub fn sum_array(x: &[i32]) -> i32 {
    x.iter().fold(0, |sum, next| sum + *next)
}
