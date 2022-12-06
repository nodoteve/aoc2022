fn run_1(s: &str) -> i32 {
    let v: Vec<Vec<Vec<i32>>> = s
        .split("\n")
        .map(|a| {
            a.split(",")
                .take(2)
                .map(|b| {
                    b.split("-")
                        .take(2)
                        .map(|c| c.parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    // println!("{:?}", v);
    let mut sum = 0;
    for x in v {
        let a = (x[0][0], x[0][1]);
        let b = (x[1][0], x[1][1]);
        if a.0 <= b.0 && a.1 >= b.1 || a.0 >= b.0 && a.1 <= b.1 {
            sum += 1;
        }
    }
    sum
}

fn run_2(s: &str) -> i32 {
    let v: Vec<Vec<Vec<i32>>> = s
        .split("\n")
        .map(|a| {
            a.split(",")
                .take(2)
                .map(|b| {
                    b.split("-")
                        .take(2)
                        .map(|c| c.parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    let mut sum = 0;
    for x in v {
        let a = (x[0][0], x[0][1]);
        let b = (x[1][0], x[1][1]);
        if a.1 >= b.0 && a.0 <= b.1 {
            sum += 1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn part_1() {
        let input = get_input(4);
        println!("{}", run_1(input.as_str()));
    }

    #[test]
    fn part_2() {
        let input = get_input(4);
        println!("{}", run_2(input.as_str()));
    }
}
