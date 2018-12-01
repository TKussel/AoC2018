use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l|  l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    let clone = input.clone();
    let it = clone.iter();
    return it.sum()
}

#[aoc(day1,part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
    let mut freq : i32 = 0;
    let mut pastfreq: HashSet<i32> = HashSet::new();
    pastfreq.insert(0);

    for i in 0..1000{
    for n in input {
        freq += n;
        let notpresent = pastfreq.insert(freq);
        if !notpresent{
            return freq;
        }
    }
    }
    panic!("No frequency twice!");
}

#[cfg(test)]
mod tests {

    use super::*;

#[test]
    fn example1() {
     assert_eq!(solve_part1(&input_generator("+1\n+1\n+1")),3);
    }
#[test]
    fn example2() {
     assert_eq!(solve_part1(&input_generator("+1\n+1\n-2")),0);
    }
#[test]
    fn example3() {
     assert_eq!(solve_part1(&input_generator("-1\n-2\n-3")),-6);
    }

}
