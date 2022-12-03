fn run_1(a: &str) -> i32 {
    let v: Vec<&str> = a.split('\n').collect();
    let mut m = 0;
    let mut h = 0;
    for x in &v {
        match x.parse::<i32>() {
            Ok(s) => {
                h += s;
                m = i32::max(m, h);
            }
            _ => {
                h = 0;
            }
        }
    }
    m
}

fn run_2(a: &str) -> i32 {
    let v: Vec<&str> = a.split('\n').collect();
    let mut h = 0;
    let mut a: Vec<i32> = Vec::new();
    for x in &v {
        match x.parse::<i32>() {
            Ok(s) => {
                h += s;
            }
            _ => {
                if h != 0 {
                    a.push(h);
                }
                h = 0;
            }
        }
    }
    a.sort();
    a.split_at(a.len() - 3).1.into_iter().fold(0, |s, x| s + x)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn get_input(i: i32) -> String {
        fs::read_to_string(format!("./input/{}", i)).expect(format!("No input file {}", i).as_str())
    }

    #[test]
    fn day_01_1() {
        let input = get_input(1);

        println!("{}", run_1(input.as_str()).to_string());
    }
    #[test]
    fn day_01_2() {
        let input = get_input(1);

        println!("{}", run_2(input.as_str()).to_string());
    }
}
