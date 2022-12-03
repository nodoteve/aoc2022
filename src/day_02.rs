struct Hand {
    c: i32,
}

impl Hand {
    pub fn new(a: i32) -> Self {
        Self { c: a }
    }
    pub fn from_str(a: &str) -> Self {
        Self {
            c: {
                match a.chars().nth(0).unwrap() {
                    'A' | 'X' => 0,
                    'B' | 'Y' => 1,
                    'C' | 'Z' => 2,
                    _ => 0,
                }
            },
        }
    }
}
impl std::cmp::PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.c == other.c
    }
}
impl std::cmp::Eq for Hand {}

use std::cmp::Ordering;
impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
// 2 > 1 ? => 2 wins over 1
// 2 > 0 ? => 2 losses to 0
impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.c == other.c {
            return Ordering::Equal;
        }
        let p = (other.c + 1) % 3;
        if self.c.cmp(&p) == Ordering::Equal {
            return Ordering::Greater;
        }
        Ordering::Less
    }
}

fn run_1(s: &str) -> i32 {
    let v = s.split("\n");
    let mut vp: Vec<(Hand, Hand)> = Vec::new();
    for x in v {
        let p: Vec<&str> = x.split(" ").take(2).collect();
        if p.len() == 2 {
            vp.push((
                Hand::from_str(p.first().unwrap()),
                Hand::from_str(p.last().unwrap()),
            ));
        }
    }
    let mut sum = 0;
    for (a, b) in vp {
        sum += b.c + 1;
        if b > a {
            sum += 6;
        } else if b == a {
            sum += 3;
        }
    }
    return sum;
}

fn run_2(s: &str) -> i32 {
    let v = s.split("\n");
    let mut vp: Vec<(Hand, Hand)> = Vec::new();
    for x in v {
        let p: Vec<&str> = x.split(" ").take(2).collect();
        if p.len() == 2 {
            let a = Hand::from_str(p.first().unwrap());
            let b = Hand::from_str(p.last().unwrap());
            let c: Hand = match b.c {
                0 => Hand::new((a.c + 2) % 3),
                1 => Hand::new(a.c),
                _ => Hand::new((a.c + 1) % 3),
            };
            vp.push((a, c));
        }
    }
    let mut sum = 0;
    for (a, b) in vp {
        sum += b.c + 1;
        if b > a {
            sum += 6;
        } else if b == a {
            sum += 3;
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_input;

    #[test]
    fn part_1() {
        let input = get_input(2);

        println!("{}", run_1(input.as_str()).to_string());
    }
    #[test]
    fn part_2() {
        let input = get_input(2);

        println!("{}", run_2(input.as_str()).to_string());
    }
}
