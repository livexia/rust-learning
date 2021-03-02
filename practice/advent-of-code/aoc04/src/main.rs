#[macro_use]
extern crate lazy_static;

use std::{collections::HashMap, usize};
use std::error::Error;
use std::ops::Range;
use std::io::{self, Read, Write};
use std::result;
use std::slice;
use std::str::FromStr;

use regex::Regex;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = result::Result<T, Box<dyn Error>>;

type GuardID = u32;
type EventsByGuard = HashMap<GuardID, Vec<Event>>;
type GuardSleepFrequency = HashMap<GuardID, [u32; 60]>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let mut events: Vec<Event> = vec![];
    for line in input.lines() {
        let event = line.parse().or_else(|err| {
            err!("failed to parse '{:?}': {}", line, err)
        })?;
        events.push(event);
    }
    if events.is_empty() {
        return err!("found no events");
    }

    events.sort_by(|ev1, ev2| ev1.datetime.cmp(&ev2.datetime));
    let mut events_by_guard = EventsByGuard::new();
    let mut cur_guard_id = None;
    for event in events {
        if let EventKind::StartShift { guard_id } = event.kind {
            cur_guard_id = Some(guard_id);
        }
        match cur_guard_id {
            None => return err!("no guard id set for event"),
            Some(id) => {
                events_by_guard.entry(id).or_default().push(event);
            }
        }
    }

    let mut minutes_asleep: GuardSleepFrequency = GuardSleepFrequency::new();
    for (&guard_id, events) in events_by_guard.iter() {
        let mut freq: [u32; 60] = [0; 60];
        for result in MinutesAsleepIter::new(events) {
            for minute in result? {
                freq[minute as usize] += 1;
            }
        }
        minutes_asleep.insert(guard_id, freq);
    }

    part1(&minutes_asleep)?;
    part2(&minutes_asleep)?;
    Ok(())
}

fn part1(minutes_asleep: &GuardSleepFrequency) -> Result<()> {
    let (&sleepiest, _) = minutes_asleep
        .iter()
        .max_by_key(|&(_, ref freqs)| -> u32 {
            freqs.iter().sum()
        }).unwrap();
    let minute = match sleepiest_minute(minutes_asleep, sleepiest) {
        None => return err!("guard {} was never asleep", sleepiest),
        Some(minute) => minute,
    };

    writeln!(io::stdout(), "part 1, product: {}", sleepiest * minute)?;

    Ok(())
}


fn part2(minutes_asleep: &GuardSleepFrequency) -> Result<()> {
    let mut sleepiest_minutes: HashMap<GuardID, (u32, u32)> = HashMap::new();
    for (&guard_id, freqs) in minutes_asleep.iter() {
        let minute = match sleepiest_minute(minutes_asleep, guard_id) {
            None => continue,
            Some(minute) => minute,
        };
        let count = freqs[minute as usize];
        sleepiest_minutes.insert(guard_id, (minute, count));
    }
    if sleepiest_minutes.is_empty() {
        return err!("no guard is slept");
    }

    let (&longest_asleep, &(minute, _)) = sleepiest_minutes
        .iter()
        .max_by_key(|&(_, (_, count))| count)
        .unwrap();

    writeln!(io::stdout(), "part 2, product: {}", longest_asleep * minute)?;

    Ok(())
}

fn sleepiest_minute(
    minutes_asleep: &GuardSleepFrequency, 
    guard_id: GuardID
) -> Option<u32> {
    let (sleepiest_minute, ..) = minutes_asleep[&guard_id]
        .iter()
        .enumerate()
        .max_by_key(|(_, &freq)| -> u32 { freq })
        .expect("Iterator of sleepy minutes should not be empty");
    Some(sleepiest_minute as u32)
}

#[derive(Debug)]
struct MinutesAsleepIter<'a> {
    events: slice::Iter<'a, Event>,
    fell_asleep: Option<u32>,
}

impl<'a> MinutesAsleepIter<'a> {
    fn new(events: &'a [Event]) -> MinutesAsleepIter<'a> {
        MinutesAsleepIter { events: events.iter(), fell_asleep: None }
    }
}

impl <'a> Iterator for MinutesAsleepIter<'a> {
    type Item = Result<Range<u32>>;

    fn next(&mut self) -> Option<Result<Range<u32>>> {
        loop {
            let event = match self.events.next() {
                Some(event) => event,
                None => {
                    if self.fell_asleep.is_some() {
                        return Some(err!("found sleep event without wake up"));
                    }
                    return None;
                }
            };
            match event.kind {
                EventKind::StartShift { .. } => {},
                EventKind::Asleep => {
                    self.fell_asleep = Some(event.datetime.minute);
                },
                EventKind::WakeUp => {
                    let fell_asleep = match self.fell_asleep.take() {
                        Some(minute) => minute,
                        None => {
                            return Some(err!("found wakeup without asleep"));
                        }
                    };
                    if event.datetime.minute < fell_asleep {
                        return Some(err!("found wakeup before asleep"));
                    }
                    return Some(Ok(fell_asleep..event.datetime.minute));
                }
            }
        }
    }
}

#[derive(Debug)]
struct Event {
    datetime: DateTime,
    kind: EventKind,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct DateTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug)]
enum EventKind {
    StartShift { guard_id: GuardID },
    WakeUp,
    Asleep
}

impl FromStr for Event {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Event> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
                \[
                    (?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})
                    \s
                    (?P<hour>[0-9]{2}):(?P<minute>[0-9]{2})
                \]
                \s+
                ((?:Guard\ \#(?P<id>[0-9]+)\ )|(?P<sleep>.+))
            ").unwrap();
        }
        let caps = match RE.captures(s) {
            None => return err!("unrecognized event"),
            Some(caps) => caps,
        };

        let datetime = DateTime {
            year: caps["year"].parse()?,
            month: caps["month"].parse()?,
            day: caps["day"].parse()?,
            hour: caps["hour"].parse()?,
            minute: caps["minute"].parse()?,
        };

        let kind = 
            if let Some(m) = caps.name("id") {
                EventKind::StartShift { guard_id: m.as_str().parse()? }
            } else if &caps["sleep"] == "falls asleep"{
                EventKind::Asleep
            } else if &caps["sleep"] == "wakes up" {
                EventKind::WakeUp
            } else {
                return err!("Could not determine event kind");
            };
        Ok(Event { datetime, kind })
    }
}
