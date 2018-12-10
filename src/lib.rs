#![feature(vec_resize_with)]
#![feature(vec_resize_default)]

extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
extern crate bytecount;
extern crate cgmath;
extern crate chrono;
extern crate regex;

pub mod day1;
pub mod day10;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vec2<T> {
    x: T,
    y: T,
}

aoc_lib! {year = 2018}
