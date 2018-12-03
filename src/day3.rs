struct Matrix {
    cols: i32,
    rows: i32,
    data: Vec<i32>
}

impl Matrix {
 fn new(cols: i32, rows: i32) -> Matrix {
    Matrix{
     cols: cols,
     rows: rows,
     data: vec![0; (cols*rows) as usize]
    }
 }
 fn at(&self, col: i32, row: i32) -> &i32 {
    &self.data[(row*self.cols+col) as usize]
 }
 fn increment(&mut self, col: i32, row: i32, amount: i32) {
    self.data[(row*self.cols+col) as usize] += amount;
 }

 fn allocate_claim(&mut self,claim: &Claim) {
    for i in 0..claim.area.size.0 {
        for j in 0..claim.area.size.1 {
        self.increment(i+claim.area.top_left.x,j+claim.area.top_left.y, 1); 
        }
    }
 }
    fn is_claim_overlaped(&self, claim: &Claim) -> bool {
        for i in 0..claim.area.size.0 {
            for j in 0..claim.area.size.1 {
                if self.at(i+claim.area.top_left.x,j+claim.area.top_left.y).clone() > 1 {
                    return true;
                }
            }
        }
    return false
    }

 fn print(&mut self) {
    for i in 0..self.rows{
        let mut row = String::new();
        for j in 0..self.cols {
            row.push(match self.at(i,j) {
                0 => '.',
                1 => 'o',
                _ => 'x'
            });
        }
        println!("{}",row);
    }
 }
}

#[derive(Debug)]
struct Point{
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    size: (i32, i32)
    
}

#[derive(Debug)]
pub struct Claim {
    id: String,
    area: Rectangle
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    let mut cv: Vec<Claim> = Vec::new();
    input.lines().for_each(|l|{
    let claim: Vec<&str> =
        l.split(|c| c == '#' || c == '@' || c == ',' || c == ':' || c == 'x').collect();
    let c = Claim{
        id: claim[1].trim().to_string(),
        area: Rectangle {
            top_left: Point{ x: claim[2].trim().parse().unwrap(), y: claim[3].trim().parse().unwrap()},
            size: (claim[4].trim().parse().unwrap(),claim[5].trim().parse().unwrap())
        }
    };
    cv.push(c);
    });
    return cv;
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Claim>) -> usize {
    let mut fabric = Matrix::new(1000,1000);
    for c in input {
        fabric.allocate_claim(c);
    }
    fabric.data.retain(|d| d.clone() > 1);
    fabric.data.len()

}
#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Claim>) -> String {
    let mut fabric = Matrix::new(1000,1000);
    for c in input {
        fabric.allocate_claim(c);
    }
    for c in input {
        if !fabric.is_claim_overlaped(&c) {
            return c.id.to_string()
        }
    }
    panic!("All Overlapping")

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn example1() {
        let input = String::from("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2");
        assert_eq!(solve_part1(&input_generator(&input)), 4);
    }
    #[test]
    fn example2() {
        let input = String::from("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2");
        assert_eq!(solve_part2(&input_generator(&input)), 3.to_string());
    }
}
