fn run_1(s: &str) -> i32 {
    let v: Vec<&str> = s.split("\n").collect();

    let mut cycle = 0;
    let mut sum = 0;
    let mut x = 1;
    for (n, line) in v.iter().enumerate() {
        if line.starts_with("noop") {
            cycle += 1;
            if cycle % 40 == 20 {
                sum += x * cycle;
                println!("{}: {}, {}, {}", n, cycle, x, x * cycle);
            }
        } else if line.starts_with("addx") {
            let dx = line.split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            cycle += 1;
            if cycle % 40 == 20 {
                sum += x * cycle;
                println!("{}: {}, {}, {}", n, cycle, x, x * cycle);
            }
            cycle += 1;
            if cycle % 40 == 20 {
                sum += x * cycle;
                println!("{}: {}, {}, {}", n, cycle, x, x * cycle);
            }
            x += dx;
        }
    }
    sum
}

fn run_2(s: &str) -> String {
    let v: Vec<&str> = s.split("\n").collect();
    let mut s = "".to_string();
    let mut cycle = 0;
    let mut x = 1;
    for (_, line) in v.iter().enumerate() {
        if line.starts_with("noop") {
            if ((cycle % 40) - 1..=(cycle % 40) + 1).contains(&x) {
                s += "#";
            } else {
                s += " ";
            }
            // println!("{}: {}, {}, {}",n, cycle, x, ((cycle%40)-1..=(cycle%40)+1).contains(&x));
            cycle += 1;

            if cycle % 40 == 0 {
                s += "\n";
            }
        } else if line.starts_with("addx") {
            let dx = line.split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            // println!("{}: {}, {}, {}",n, cycle, x, ((cycle%40)-1..=(cycle%40)+1).contains(&x));
            if ((cycle % 40) - 1..=(cycle % 40) + 1).contains(&x) {
                s += "#";
            } else {
                s += " ";
            }
            cycle += 1;
            if cycle % 40 == 0 {
                s += "\n";
            }
            // println!("{}: {}, {}, {}",n, cycle, x, ((cycle%40)-1..=(cycle%40)+1).contains(&x));
            if ((cycle % 40) - 1..=(cycle % 40) + 1).contains(&x) {
                s += "#";
            } else {
                s += " ";
            }
            cycle += 1;
            if cycle % 40 == 0 {
                s += "\n";
            }
            x += dx;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn part_1() {
        let input = get_input(10);
        println!("{}", run_1(input.as_str()));
    }

    #[test]
    fn part_2() {
        let input = get_input(10);
        println!("{}", run_2(input.as_str()));
    }
}
