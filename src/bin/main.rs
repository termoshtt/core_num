#![no_std]
#![feature(lang_items)]

extern crate core_num;
use core_num::FloatIntrinsics;

fn test_float() {
    1.0_f64.sin();
}

fn main() {
    test_float();
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() -> ! { loop {} }
