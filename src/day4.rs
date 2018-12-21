use chrono::{NaiveDateTime, Timelike};
use std::collections::HashMap;
use std::ops::Range;

pub struct Record {
    id: u32,
    time_intervals: Vec<Range<u32>>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Record> {
    // Record example
    // [1518-11-01 00:00] Guard #10 begins shift
    // [1518-11-01 00:05] falls asleep
    // [1518-11-01 00:25] wakes up

    let mut sorted_raw_records = input
        .lines()
        .map(|line| {
            let mut parts = line.split(']');
            let date_time =
                NaiveDateTime::parse_from_str(&parts.next().unwrap()[1..], "%F %R").unwrap();
            (date_time, parts.next().unwrap().trim())
        })
        .collect::<Vec<(NaiveDateTime, &str)>>();
    sorted_raw_records.sort_by(|a, b| a.0.cmp(&b.0));

    let mut records = Vec::new();
    sorted_raw_records
        .iter()
        .for_each(|(date_time, raw_record)| {
            if raw_record.ends_with("begins shift") {
                let id = raw_record.split(' ').nth(1).unwrap()[1..].parse().unwrap();
                records.push(Record {
                    id,
                    time_intervals: Vec::new(),
                });
            } else if *raw_record == "falls asleep" {
                records
                    .last_mut()
                    .unwrap()
                    .time_intervals
                    .push(date_time.minute()..60);
            } else if *raw_record == "wakes up" {
                records
                    .last_mut()
                    .unwrap()
                    .time_intervals
                    .last_mut()
                    .unwrap()
                    .end = date_time.minute();
            }
        });

    records
}

pub fn get_sleep_times_by_guard(records: &[Record]) -> HashMap<u32, [u32; 60]> {
    let mut sleep_times_by_guard = HashMap::<u32, [u32; 60]>::new();

    for record in records {
        sleep_times_by_guard
            .entry(record.id)
            .and_modify(|sleep_times| {
                for interval in &record.time_intervals {
                    for i in interval.clone() {
                        sleep_times[i as usize] += 1;
                    }
                }
            })
            .or_insert_with(|| {
                let mut sleep_times = [0; 60];
                for interval in &record.time_intervals {
                    for i in interval.clone() {
                        sleep_times[i as usize] = 1;
                    }
                }
                sleep_times
            });
    }

    sleep_times_by_guard
}

#[aoc(day4, part1)]
pub fn solve_part1(records: &[Record]) -> u32 {
    let sleep_times_by_guard = get_sleep_times_by_guard(records);
    let guard_with_max_minutes = sleep_times_by_guard
        .iter()
        .max_by_key::<u32, _>(|(_, times)| times.iter().sum())
        .unwrap()
        .0;
    let sleep_in_most_minute = sleep_times_by_guard[guard_with_max_minutes]
        .iter()
        .enumerate()
        .max_by_key(|x| x.1)
        .unwrap()
        .0 as u32;

    guard_with_max_minutes * sleep_in_most_minute
}

#[aoc(day4, part2)]
pub fn solve_part2(records: &[Record]) -> u32 {
    let sleep_times_by_guard = get_sleep_times_by_guard(records);
    let (guard, times, _) = sleep_times_by_guard
        .iter()
        .map(|(guard, times)| {
            let minute_and_times = times.iter().enumerate().max_by_key(|x| x.1).unwrap();
            (guard, minute_and_times.0 as u32, minute_and_times.1)
        })
        .max_by_key(|x| x.2)
        .unwrap();
    guard * times
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(&input_generator(
                r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"
            )),
            240
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(&input_generator(
                r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"
            )),
            4455
        );
    }
}
