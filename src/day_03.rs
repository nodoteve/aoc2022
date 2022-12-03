fn mask_compartment(s: &str) -> Vec<i32> {
    let mut out = vec![0; 52];
    for x in s.chars() {
        let idx:u32;
        if x.is_ascii_lowercase(){
            idx=x as u32 - 'a' as u32;
        }
        else if x.is_ascii_uppercase(){
            idx=x as u32 - 'A' as u32+26;
        }
        else {
            return out;
        }
        out[idx as usize] = 1;
    }
    out
}

fn run_1(s: &str) -> i32 {
    let v = s
        .split("\n")
        .map(|p| p.split_at(p.len() / 2))
        .collect::<Vec<(&str, &str)>>();
    let mut sum = 0;
    for (a, b) in v {
        let (mask_a, mask_b) = (mask_compartment(a), mask_compartment(b));
        let it = mask_a.iter().zip(mask_b).enumerate().reduce(|w, p| {
            if *p.1.0 == 1 && p.1.1 == 1 {
                p
            } else {
                w
            }
        });
        println!("{:?}", it);
        sum += it.unwrap().0 as i32 + 1;
    }
    sum
}

fn run_2(s: &str) -> i32 {
    let v:Vec<&str> = s
        .split("\n").collect();
    let mut sum = 0;
    for t in v.chunks(3){
        let vals:Vec<Vec<i32>> = t.iter().map(|el|mask_compartment(el)).collect();
        let (a,b,c) =(vals[0].clone(),vals[1].clone(),vals[2].clone());
        let it = a.iter()
        .zip(b)
        .zip(c)
        .enumerate()
        .reduce(|w, p| {
            if *p.1.0.0 == 1 && p.1.0.1 == 1 && p.1.1 == 1 {
                p
            } else {
                w
            }
        });
        // println!("{:?}", it);
        sum += it.unwrap().0 as i32 + 1;
    }
    sum
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn part_1() {
        // println!("{}",'a' as u32 as usize);
        let input = get_input(3);
        println!("{}", run_1(input.as_str()));
    }

    #[test]
    fn part_2(){
        let input = get_input(3);
        println!("{}", run_2(input.as_str()));
    }
}
