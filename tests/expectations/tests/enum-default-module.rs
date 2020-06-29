#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub mod Foo {
    pub type Type = u32;
    pub const Bar: Type = 0;
    pub const Qux: Type = 1;
}
pub mod Neg {
    pub type Type = i32;
    pub const MinusOne: Type = -1;
    pub const One: Type = 1;
}