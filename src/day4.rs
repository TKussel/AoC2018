use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Timestamp {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32
}

impl std::ops::Sub for Timestamp {
    type Output = Timestamp;

    fn sub(self, other: Timestamp) -> Timestamp {
        let mut year = self.year - other.year;
        let mut m = self.month - other.month;
        let mut month = if m < 0 { m +=12; year -= 1; m } else {m};
        let mut d = self.day - other.day;
        let mut day = if d < 0 { d += 31; month -= 1; d } else {d};
        let mut h = self.hour - other.hour;
        let mut hour = if h < 0 { h += 24; day -= 1; h } else {h};
        let mut mi = self.minute - other.minute;
        let minute = if  mi < 0 { mi += 60; hour -= 1; mi } else {mi};
        return Timestamp{year, month, day, hour, minute};
    }
}
#[derive(Debug, PartialEq,Clone)]
enum Event {
    Wakeup,
    Sleep,
    Shift(i32)
}

#[derive(Debug, PartialEq)]
pub struct Log {
    timestamp: Timestamp,
    event: Event
}

#[derive(Debug)]
struct Guard {
    id: i32,
    logs: Vec<Log>
}

impl std::cmp::PartialEq for Guard {
    fn eq(&self, other: &Guard) -> bool {
    self.id == other.id
    }
}
impl std::cmp::Eq for Guard{}

impl std::hash::Hash for Guard{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
     self.id.hash(state)
    }
}

impl Guard {
    fn new(id: i32) -> Guard {
        Guard{id, logs: Vec::new() }
    }
    fn asleep_min(&self) -> i32 {
        let mut it = self.logs.iter().peekable();
        let mut sleeptime = 0;
        while let Some(log) = it.next() {
            if log.event == Event::Sleep {
                let wakeup = it.peek().expect("Line after sleep is not wakeup!");
                sleeptime += wakeup.timestamp.minute - log.timestamp.minute;
            }
        }
        if self.logs.is_empty() {return -5}
        return sleeptime;
    }

    fn sleep_histogram(&self) -> HashMap<i32, i32> {
        let mut sleephist: HashMap<i32, i32> = HashMap::new();
        let mut it = self.logs.iter().peekable();
        while let Some(log) = it.next() {
            if log.event == Event::Sleep {
                let wakeup = it.peek().expect("Line after sleep is not wakeup!");
                let mut i = log.timestamp.minute.clone();
                while i < wakeup.timestamp.minute {
                    *sleephist.entry(i).or_insert(0) += 1;
                    i += 1;
                }
            }
        }
        return sleephist
    }

    fn most_slept_min(&self) -> (i32,i32) {
     let hist = self.sleep_histogram();
     let mut it = hist.iter();
     let mut maximum: (i32,i32) = (-1,-1);
     while let Some((min,val)) = it.next() {
        if val > &maximum.1 {
            maximum = (min.clone(),val.clone());}
     }
     return maximum
    }
}


#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Log> {
    let mut log: Vec<Log> = Vec::new();
    input.lines().for_each(|l|{
        let date: Vec<i32> = l.get(1..11).unwrap().split('-').map(|v|v.parse().unwrap()).collect();
        let time: Vec<i32> = l.get(12..17).unwrap().split(':').map(|v|v.parse().unwrap()).collect();
        let t = Timestamp{year: date[0], month: date[1], day: date[2], hour: time[0], minute: time[1]};
        let e = l.split_at(19).1;
        let event = if e.starts_with("falls") {Event::Sleep} 
                    else if e.starts_with("wakes") {Event::Wakeup} 
                    else if e.starts_with("Guard") {
                        let idstart = e.find('#').unwrap() + 1;
                        let mut line = e.to_string();
                        let ev: String= line.drain(idstart..).collect();
                        let id: Vec<&str> = ev.split_whitespace().collect();
                        Event::Shift(id[0].parse().unwrap()) } 
                    else {panic!("Invalid Log entry: {}", e)};
        log.push(Log{timestamp: t, event: event});
    });

    log.sort_by(|a, b| match a.timestamp.year.cmp(&b.timestamp.year){
                                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                                std::cmp::Ordering::Equal => match a.timestamp.month.cmp(&b.timestamp.month) {
                                    std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                                    std::cmp::Ordering::Equal => match a.timestamp.day.cmp(&b.timestamp.day) {
                                        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                                        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                                        std::cmp::Ordering::Equal => match a.timestamp.hour.cmp(&b.timestamp.hour) {
                                            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                                            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                                            std::cmp::Ordering::Equal => match a.timestamp.minute.cmp(&b.timestamp.minute) {
                                                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                                                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                                                std::cmp::Ordering::Equal => std::cmp::Ordering::Equal
                                            }
                                        }
                                    }
                                }
    });
    log
}

fn parse_log(log: &Vec<Log>) -> HashMap<i32, Guard> {
    let mut it = log.iter();
    let mut guards: HashMap<i32, Guard> = HashMap::new();
    let mut last_id: i32 = -1;
    while let Some(line) = it.next() {
        match line.event {
            Event::Shift(id) => {
                last_id = id;
                if !guards.contains_key(&id) {
                    guards.insert(id, Guard::new(id));
                    };
                },
            _ => {
                guards.entry(last_id).and_modify(|e| e.logs.push(Log{timestamp: line.timestamp.clone(), event: line.event.clone()}));}
        }
    }
    return guards
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<Log>) -> i32 {
    let guards = parse_log(input);
    let mut it = guards.iter();
    let mut optimum: (i32, i32) = (-1,-1);
    while let Some((id, guard)) = it.next() {
        if guard.asleep_min() > optimum.1 {
            optimum = (id.clone(), guard.asleep_min().clone());
        }
    }
    let minguard = guards.get(&optimum.0).unwrap();
    return minguard.id * minguard.most_slept_min().1;
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Vec<Log>) -> i32 {
    let guards = parse_log(input);
    let mut optimum: (i32, i32, i32) = (-1,-1,-1);
        for g in guards {
            let loc_max = g.1.most_slept_min();
            if loc_max.1 > optimum.2 {
                optimum = (g.0, loc_max.0, loc_max.1);
            }
        }
    return optimum.0 * optimum.1
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn example1() {
        let input = String::from("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2");
        assert_eq!(solve_part1(&input_generator(&input)), 4);
    }
}
