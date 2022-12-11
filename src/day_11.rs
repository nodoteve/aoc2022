use std::collections::HashMap;

#[derive(Default,Debug,Clone)]
struct Monkee{
    id: i32,
    items: Vec<i64>,
    operation: Vec<String>,
    test: /* divisible by */ i64,
    case_true: /* throw to monkee # */ i32,
    case_false: /* throw to monkee # */ i32,
    counter_turns: i64,
}

impl Monkee {
    fn with_id(id:i32)->Monkee{
        Monkee{id,..Default::default()}
    }
}


fn run_1(s: &str) -> i64 {
    let v:Vec<&str>=s.split("\n").collect();
    let mut g:HashMap<i32,Monkee>=HashMap::new();
    let mut monke=Monkee::with_id(-1);
    for l in v{
        let l = l.trim_start();
        if l.starts_with("Monkey"){
            monke=Monkee::with_id(l
                .replace("Monkey", "")
                .replace(":", "")
                .trim()
                .parse::<i32>()
                .unwrap())
        }
        else if l.starts_with("Starting items:")
        {
            monke.items = l
                .replace("Starting items:", "")
                .trim()
                .split(", ")
                .map(|x|x.parse::<i64>().unwrap()).collect();
        }
        else if l.starts_with("Operation:")
        {
            monke.operation = l
                .replace("Operation: new =", "")
                .trim()
                .split(" ")
                .map(|x|x.to_string()).collect();
        }
        else if l.starts_with("Test:")
        {
            monke.test= l
                .replace("Test: divisible by", "")
                .trim()
                .parse::<i64>().unwrap();
        }
        else if l.starts_with("If true:")
        {
            monke.case_true=l
                .replace("If true: throw to monkey", "")
                .trim()
                .parse::<i32>().unwrap();
        }
        else if l.starts_with("If false:")
        {
            monke.case_false=l
                .replace("If false: throw to monkey", "")
                .trim()
                .parse::<i32>().unwrap();
            // println!("{}", monke.id);
            g.insert(monke.id,monke.clone());
        }
    }
    let mut ids:Vec<i32> = g.keys().into_iter().map(|k|*k).collect();
    ids.sort();

    for _round in 0..10000{
        // let mut gv = g.iter()
            // .map(|(k,v)|{(k.clone(),v.clone())})
            // .collect::<Vec<(i32,Monkee)>>();
        // g.iter_mut().for_each(|(_k,v)|{v.items=Vec::new();});
        // gv.sort_by(|a,b|a.0.cmp(&b.0));
        for id in &ids{
            // println!("Monke {}", id);
            let monke = g.get(&id).unwrap().clone();
            g.get_mut(id).unwrap().items=Vec::new();
            let items = monke.items;
            g.entry(*id).and_modify(|m|m.counter_turns+=items.len() as i64);
            for old in items{
                // println!("\titem {}", old);
                let mut worry_item = 0;
                for ops in monke.operation.chunks(3){
                    match ops {
                        [lhs, op, rhs]=>{
                            let lhs = match lhs.as_str() {
                                "old"=>old,
                                _=>lhs.parse::<i64>().unwrap()
                            };
                            let rhs = match rhs.as_str() {
                                "old"=>old,
                                _=>rhs.parse::<i64>().unwrap()
                            };
                            match op.as_str() {
                                "+"=>worry_item=lhs+rhs,
                                "*"=>worry_item=lhs*rhs,
                                "-"=>worry_item=lhs-rhs,
                                "/"=>worry_item=lhs/rhs,
                                _=>{}
                            }
                        }
                        _=>{}
                    }
                }
                // println!("\tworry_item {} -> {}", worry_item, worry_item/3);

                worry_item/=3;
                if worry_item%monke.test==0{
                    g.entry(monke.case_true)
                        .and_modify(|m|m.items.push(worry_item));
                    // println!("\tto monke  {}", monke.case_true);
                }
                else{                    
                    g.entry(monke.case_false)
                        .and_modify(|m|m.items.push(worry_item));
                    // println!("\tto monke  {}", monke.case_false);
                }
            }
        }
        // println!("{}: ",_round);
        // for id in &ids{
            // println!("  {} = {:?}",id, g.get(&id).unwrap().items);
        // }
    }
    let mut arr=g.iter().map(|(_id,m)|m.counter_turns).collect::<Vec<i64>>();
    arr.sort();
    arr=arr.clone().into_iter().rev().collect();
    // println!("{:?}",arr);
    arr[0]*arr[1]
}


fn run_2(s: &str) -> i64 {
    let v:Vec<&str>=s.split("\n").collect();
    let mut g:HashMap<i32,Monkee>=HashMap::new();
    let mut monke=Monkee::with_id(-1);
    for l in v{
        let l = l.trim_start();
        if l.starts_with("Monkey"){
            monke=Monkee::with_id(l
                .replace("Monkey", "")
                .replace(":", "")
                .trim()
                .parse::<i32>()
                .unwrap())
        }
        else if l.starts_with("Starting items:")
        {
            monke.items = l
                .replace("Starting items:", "")
                .trim()
                .split(", ")
                .map(|x|x.parse::<i64>().unwrap()).collect();
        }
        else if l.starts_with("Operation:")
        {
            monke.operation = l
                .replace("Operation: new =", "")
                .trim()
                .split(" ")
                .map(|x|x.to_string()).collect();
        }
        else if l.starts_with("Test:")
        {
            monke.test= l
                .replace("Test: divisible by", "")
                .trim()
                .parse::<i64>().unwrap();
        }
        else if l.starts_with("If true:")
        {
            monke.case_true=l
                .replace("If true: throw to monkey", "")
                .trim()
                .parse::<i32>().unwrap();
        }
        else if l.starts_with("If false:")
        {
            monke.case_false=l
                .replace("If false: throw to monkey", "")
                .trim()
                .parse::<i32>().unwrap();
            g.insert(monke.id,monke.clone());
        }
    }
    let mut ids:Vec<i32> = g.keys().into_iter().map(|k|*k).collect();
    ids.sort();
    let worry_mod = g.values().into_iter().fold(1, |acc,el|{acc*el.test});

    for _round in 0..10000{
        for id in &ids{
            let monke = g.get(&id).unwrap().clone();
            g.get_mut(id).unwrap().items=Vec::new();
            let items = monke.items;
            g.entry(*id).and_modify(|m|m.counter_turns+=items.len() as i64);
            for old in items{
                let mut worry_item = 0;
                for ops in monke.operation.chunks(3){
                    match ops {
                        [lhs, op, rhs]=>{
                            let lhs = match lhs.as_str() {
                                "old"=>old,
                                _=>lhs.parse::<i64>().unwrap()
                            };
                            let rhs = match rhs.as_str() {
                                "old"=>old,
                                _=>rhs.parse::<i64>().unwrap()
                            };
                            match op.as_str() {
                                "+"=>worry_item=lhs+rhs,
                                "*"=>worry_item=lhs*rhs,
                                "-"=>worry_item=lhs-rhs,
                                "/"=>worry_item=lhs/rhs,
                                _=>{}
                            }
                        }
                        _=>{}
                    }
                }

                worry_item%=worry_mod;
                if worry_item%monke.test==0{
                    g.entry(monke.case_true)
                        .and_modify(|m|m.items.push(worry_item));
                }
                else{                    
                    g.entry(monke.case_false)
                        .and_modify(|m|m.items.push(worry_item));
                }
            }
        }
    }
    let mut arr=g.iter().map(|(_id,m)|m.counter_turns).collect::<Vec<i64>>();
    arr.sort();
    arr=arr.clone().into_iter().rev().collect();
    arr[0]*arr[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn part_1() {
        let input = get_input(11);
        println!("{}", run_1(input.as_str()));
    }

    #[test]
    fn part_2() {
        let input = get_input(11);
        // println!("{}",5*11*2*13*7*3*17*19);
        println!("{}", run_2(input.as_str()));
    }
}
