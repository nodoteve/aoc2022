use std::collections::VecDeque;


fn run_1(s:&str)->String{
    let v:Vec<&str>=s.split('\n').collect();
    let n = (v[0].len()+1)/4;
    let mut g:Vec<VecDeque<char>>=Vec::new();
    g.resize(n, VecDeque::new());
    for b in v.iter(){
        if b.starts_with("["){
        for (x,c) in b.chars()
                .collect::<Vec<char>>()
                .chunks(4).enumerate()
            {
                if x<n && c[1]!=' ' {
                    g[x].push_front(c[1]);
                }
            }
        }
        else{
            break;   
        }
    }

    for line in v{
        if line.starts_with("m"){
            let mut m:Vec<usize> = vec![];
            for k in line.split(" ") {
                match k.parse::<i32>(){
                    Ok(w)=>m.push(w as usize),
                    _=>{}
                }
            }
            let len = g[m[1]-1].len();
            let crates = g[m[1]-1].split_off(len-m[0]);
            g[m[2]-1].extend(crates.iter());
        }
    }
    println!("{:?}",g);


    g.iter().map(
        |deq|
            match deq.back(){
                Some(s)=>format!("{}",s),
                _=>"".to_string()
            }
        ).collect::<Vec<String>>().join("")
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::utils::*;

    #[test]
    fn part_1(){
        let input = get_input(5);
        println!("{}", run_1(input.as_str()));
    }
}