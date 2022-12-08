use std::fmt;

struct vvb(Vec<Vec<bool>>);

impl fmt::Display for vvb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_string();
        for line in &self.0 {
            for &b in line {
                if b {
                    s += "1";
                } else {
                    s += "0";
                }
            }
            s += "\n";
        }
        write!(f, "{}", s)
    }
}

fn run_1(s: &str) -> u64 {
    let v: Vec<&str> = s.split("\n").collect();
    let n = v.len();
    let h: Vec<Vec<char>> = v
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut g: Vec<Vec<i16>> = h
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| c.to_string().parse::<i16>().unwrap())
                .collect()
        })
        .collect();
    let mut visible: Vec<Vec<bool>> = g
        .iter()
        .map(|line| line.iter().map(|i| false).collect())
        .collect();

    for x in 0..n {
        let mut hightest = -1;
        for y in 0..n {
            if g[y][x] > hightest {
                hightest = g[y][x];
                visible[y][x] = true;
            }
        }
        let mut hightest = -1;
        for y in (0..n).rev() {
            if g[y][x] > hightest {
                hightest = g[y][x];
                visible[y][x] = true;
            }
        }
    }

    for y in 0..n {
        let mut hightest = -1;
        for x in 0..n {
            if g[y][x] > hightest {
                hightest = g[y][x];
                visible[y][x] = true;
            }
        }
        let mut hightest = -1;
        for x in (0..n).rev() {
            if g[y][x] > hightest {
                hightest = g[y][x];
                visible[y][x] = true;
            }
        }
    }

    // println!("{}",vvb(visible.clone()));

    visible.iter().fold(0, |acc, line| {
        acc + line.iter().fold(0, |a, &val| if val { a + 1 } else { a })
    })
}

fn run_2(s: &str) -> u64 {
    let v: Vec<&str> = s.split("\n").collect();
    let n = v.len();
    let h: Vec<Vec<char>> = v
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut g: Vec<Vec<i16>> = h
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| c.to_string().parse::<i16>().unwrap())
                .collect()
        })
        .collect();
    let mut score: Vec<Vec<u64>> = g
        .iter()
        .map(|line| line.iter().map(|i| 0).collect())
        .collect();

    for x in 0..n {
        for y in 0..n {
            let mut mult = 1;
            let mut a = 0;
            while x + a + 1 < n {
                a += 1;
                if g[y][x + a] >= g[y][x] {
                    break;
                }
            }
            mult *= a;
            let mut a = 0;
            while y + a + 1 < n {
                a += 1;
                if g[y + a][x] >= g[y][x] {
                    break;
                }
            }
            mult *= a;
            let mut a = 0;
            while x.checked_sub(a + 1) != None {
                a += 1;
                if g[y][x - a] >= g[y][x] {
                    break;
                }
            }
            mult *= a;
            let mut a = 0;
            while y.checked_sub(a + 1) != None {
                a += 1;
                if g[y - a][x] >= g[y][x] {
                    break;
                }
            }
            mult *= a;
            score[y][x] = mult as u64;
        }
    }

    score.iter().fold(0, |acc, line| {
        acc.max(line.iter().fold(0, |a, &val| a.max(val as u64)))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn part_1() {
        let input = get_input(8);
        println!("{}", run_1(input.as_str()));
    }

    #[test]
    fn part_2() {
        let input = get_input(8);
        println!("{}", run_2(input.as_str()));
    }
}
