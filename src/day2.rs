use std::collections::HashMap;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<String>) -> u32 {
    let mut histogram: Vec<HashMap<String, i32>> = Vec::new();
    for l in input {
        let mut freq: HashMap<String, i32> = HashMap::new();
        for c in l.chars() {
            *freq.entry(c.to_string()).or_insert(0) += 1;
        }
        histogram.push(freq);
    }
    let mut twoers : u32 = 0;
    let mut threes : u32 = 0;
    for label in histogram {
        let mut ins_two: bool = false;
        let mut ins_three: bool = false;
        for freq in label.values() {
            if !ins_two && freq.clone() == 2 {
                twoers += 1;
                ins_two = true;
            }
            if !ins_three && freq.clone() == 3 {
                threes += 1;
                ins_three = true;
            }
        }
    }
    return twoers * threes
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<String>) -> String {
    let mut iter1 = input.iter();
    while let Some(label) = iter1.next() {
        let mut iter2 = iter1.clone();
        iter2.next();
        while let Some(next) = iter2.next() {
            let mut differences = 0;
            let mut candidate: String = String::new();
            for (c1, c2) in label.chars().zip(next.chars()) {
                if c1 != c2 {
                    differences += 1;
                } else {
                    candidate.push(c1);
                }
            }
            if differences == 1 {
                return candidate;
            }
        }
    }
    panic!("No candidate found");
}


#[cfg(test)]
mod tests {

    use super::*;

#[test]
    fn example1() {
    let input: String = String::from("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab");
     assert_eq!(solve_part1(&input_generator(&input)),12);
    }
}
