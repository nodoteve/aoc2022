fn run_1(s: &str) -> i32 {
    let v: Vec<char> = s.chars().collect();
    let mut chash = Vec::new();
    chash.resize(26, 0);
    for (x, c) in v.iter().enumerate() {
        if x >= 4 {
            if chash.iter().fold(0, |a, b| a.max(*b)) == 1 {
                return x as i32;
            }
            chash[v[x - 4] as usize - 'a' as usize] -= 1;
        }
        chash[*c as usize - 'a' as usize] += 1;
    }
    return -1;
}

fn run_2(s: &str) -> i32 {
    let v: Vec<char> = s.chars().collect();
    let mut chash = Vec::new();
    chash.resize(26, 0);
    for (x, c) in v.iter().enumerate() {
        if x >= 14 {
            if chash.iter().fold(0, |a, b| a.max(*b)) == 1 {
                return x as i32;
            }
            chash[v[x - 14] as usize - 'a' as usize] -= 1;
        }
        chash[*c as usize - 'a' as usize] += 1;
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn part_1() {
        let input = get_input(6);
        println!("{}", run_1(input.as_str()));
    }

    #[test]
    fn part_2() {
        let input = get_input(6);
        println!("{}", run_2(input.as_str()));
    }
}
