use std::collections::HashSet;

fn show(h: &HashSet<(i32, i32)>) {
    let max = h
        .iter()
        .fold((0, 0), |acc, el| (acc.0.max(el.0), acc.1.max(el.1)));
    let min = h
        .iter()
        .fold((0, 0), |acc, el| (acc.0.min(el.0), acc.1.min(el.1)));
    let mut s: Vec<Vec<char>> = vec![
        std::iter::repeat(' ')
            .take((max.0 - min.0 + 1) as usize)
            .collect::<Vec<char>>();
        (max.1 - min.1 + 1) as usize
    ];
    for (x, y) in h {
        s[(-min.1 + *y) as usize][(-min.0 + *x) as usize] = '#';
    }
    let s: String = s
        .iter()
        .map(|v| v.iter().collect::<String>() + "\n")
        .collect();
    use std::fs;
    fs::write("./output/tmp", s).unwrap();
}

fn run_1(s: &str) -> i32 {
    let v: Vec<&str> = s.split("\n").collect();

    let mut g: HashSet<(i32, i32)> = HashSet::new();

    let mut ph = (0, 0);
    let mut pt = (0, 0);
    g.insert(pt);
    for dir in v {
        let repeat = dir.split(" ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        for _ in 0..repeat {
            let mut t = "";
            if dir.starts_with("R") {
                ph.0 += 1;
                t = "r";
            }
            if dir.starts_with("L") {
                ph.0 -= 1;
                t = "l";
            }
            if dir.starts_with("U") {
                ph.1 += 1;
                t = "u";
            }
            if dir.starts_with("D") {
                ph.1 -= 1;
                t = "d";
            }
            let touch: bool = ((ph.0 - pt.0) as i32).pow(2) + ((ph.1 - pt.1) as i32).pow(2) < 4;

            if !touch {
                if ph.0 != pt.0 && ph.1 != pt.1 {
                    if ph.1 > pt.1 {
                        if ph.0 > pt.0 {
                            pt.1 += 1;
                            pt.0 += 1;
                        } else {
                            pt.1 += 1;
                            pt.0 -= 1;
                        }
                    } else {
                        if ph.0 > pt.0 {
                            pt.1 -= 1;
                            pt.0 += 1;
                        } else {
                            pt.1 -= 1;
                            pt.0 -= 1;
                        }
                    }
                } else {
                    pt = match t {
                        "u" => (pt.0, pt.1 + 1),
                        "d" => (pt.0, pt.1 - 1),
                        "r" => (pt.0 + 1, pt.1),
                        _ => (pt.0 - 1, pt.1),
                    }
                }
                g.insert(pt);
                // println!("{:?}",pt);
            }
        }
    }
    // show(&g);
    g.len() as i32
}

fn run_2(s: &str) -> i32 {
    let v: Vec<&str> = s.split("\n").collect();

    let mut g: HashSet<(i32, i32)> = HashSet::new();

    let mut p: Vec<(i32, i32)> = vec![(0, 0); 10];
    g.insert((0, 0));
    for dir in v {
        let repeat = dir.split(" ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        for _ in 0..repeat {
            if dir.starts_with("R") {
                p[0].0 += 1;
            }
            if dir.starts_with("L") {
                p[0].0 -= 1;
            }
            if dir.starts_with("U") {
                p[0].1 += 1;
            }
            if dir.starts_with("D") {
                p[0].1 -= 1;
            }
            for k in 1..10 {
                let touch: bool = ((p[k].0 - p[k - 1].0) as i32).pow(2)
                    + ((p[k].1 - p[k - 1].1) as i32).pow(2)
                    < 4;

                if !touch {
                    if p[k - 1].0 != p[k].0 && p[k - 1].1 != p[k].1 {
                        if p[k - 1].1 > p[k].1 {
                            if p[k - 1].0 > p[k].0 {
                                p[k].1 += 1;
                                p[k].0 += 1;
                            } else {
                                p[k].1 += 1;
                                p[k].0 -= 1;
                            }
                        } else {
                            if p[k - 1].0 > p[k].0 {
                                p[k].1 -= 1;
                                p[k].0 += 1;
                            } else {
                                p[k].1 -= 1;
                                p[k].0 -= 1;
                            }
                        }
                    } else {
                        p[k] = (
                            p[k].0 + (p[k - 1].0 - p[k].0) as i32 / 2,
                            p[k].1 + (p[k - 1].1 - p[k].1) as i32 / 2,
                        );
                    }
                    g.insert(p[9]);
                    // println!("{:?}",p[9]);
                }
            }
        }
    }
    // show(&g);
    g.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn part_1() {
        let input = get_input(9);
        println!("{}", run_1(input.as_str()));
    }

    #[test]
    fn part_2() {
        let input = get_input(9);
        println!("{}", run_2(input.as_str()));
    }
}
