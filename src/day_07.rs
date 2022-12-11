use std::collections::HashMap;
struct File_item {
    name: String,
    size: u64,
}

fn run_1(s: &str) -> u64 {
    let v = s.split("$").filter(|line| !line.is_empty());

    let mut path: Vec<String> = Vec::new();
    let mut g: HashMap<Vec<String>, Vec<File_item>> = HashMap::new();
    for input in v {
        if input.starts_with(" cd ") {
            let dir = input.replace(" cd ", "").trim().to_string();

            match dir.as_str() {
                ".." => {
                    path.pop();
                }
                a => {
                    path.push(a.to_string());
                }
            };
        } else if input.starts_with(" ls\n") {
            let files_list: Vec<(u64, String)> = input
                .split("\n")
                .filter(|line| line.split(" ").nth(0).unwrap().parse::<u64>().is_ok())
                .map(|line| {
                    let words: Vec<&str> = line.split(" ").collect();
                    (
                        words.first().unwrap().parse::<u64>().unwrap(),
                        words.first().unwrap().to_string(),
                    )
                    // words.nth(1).unwrap().to_string())
                })
                .collect();

            let files_list = files_list
                .iter()
                .map(|(size, name)| File_item {
                    size: *size,
                    name: name.clone(),
                })
                .collect();

            g.entry(path.clone()).or_insert(files_list);
        }
    }

    let mut order_of_paths: Vec<Vec<String>> = Vec::new();
    for (path, _list) in &g {
        order_of_paths.push(path.clone());
    }
    order_of_paths.sort_by(|a, b| b.len().cmp(&a.len()));

    let mut total_size_g: HashMap<Vec<String>, u64> = HashMap::new();
    for path in &order_of_paths {
        let list = g.get(path).unwrap();
        let sum = list.iter().fold(0, |a, item| a + item.size);
        println!("{:?}", sum);

        total_size_g
            .entry(path.clone())
            .and_modify(|value| *value += sum)
            .or_insert(sum);
        let sum = *total_size_g.get(path).unwrap();

        let mut prev_path = path.clone();
        prev_path.pop();
        if !prev_path.is_empty() {
            total_size_g
                .entry(prev_path.clone())
                .and_modify(|value| *value += sum)
                .or_insert(sum);
        }
    }
    let mut sum: u64 = 0;
    for (_path, total_size) in &total_size_g {
        if *total_size <= 100000 {
            sum += total_size;
        }
    }
    sum
}

fn run_2(s: &str) -> u64 {
    let v = s.split("$").filter(|line| !line.is_empty());

    let mut path: Vec<String> = Vec::new();
    let mut g: HashMap<Vec<String>, Vec<File_item>> = HashMap::new();
    for input in v {
        if input.starts_with(" cd ") {
            let dir = input.replace(" cd ", "").trim().to_string();

            match dir.as_str() {
                ".." => {
                    path.pop();
                }
                a => {
                    path.push(a.to_string());
                }
            };
        } else if input.starts_with(" ls\n") {
            let files_list: Vec<(u64, String)> = input
                .split("\n")
                .filter(|line| line.split(" ").nth(0).unwrap().parse::<u64>().is_ok())
                .map(|line| {
                    let words: Vec<&str> = line.split(" ").collect();
                    (
                        words.first().unwrap().parse::<u64>().unwrap(),
                        words.first().unwrap().to_string(),
                    )
                    // words.nth(1).unwrap().to_string())
                })
                .collect();

            let files_list = files_list
                .iter()
                .map(|(size, name)| File_item {
                    size: *size,
                    name: name.clone(),
                })
                .collect();

            g.entry(path.clone()).or_insert(files_list);
        }
    }

    let mut order_of_paths: Vec<Vec<String>> = Vec::new();
    for (path, _list) in &g {
        order_of_paths.push(path.clone());
    }
    order_of_paths.sort_by(|a, b| b.len().cmp(&a.len()));

    let mut total_size_g: HashMap<Vec<String>, u64> = HashMap::new();
    for path in &order_of_paths {
        let list = g.get(path).unwrap();
        let sum = list.iter().fold(0, |a, item| a + item.size);
        println!("{:?}", sum);

        total_size_g
            .entry(path.clone())
            .and_modify(|value| *value += sum)
            .or_insert(sum);
        let sum = *total_size_g.get(path).unwrap();

        let mut prev_path = path.clone();
        prev_path.pop();
        if !prev_path.is_empty() {
            total_size_g
                .entry(prev_path.clone())
                .and_modify(|value| *value += sum)
                .or_insert(sum);
        }
    }

    let root = vec!["/".to_string()];
    let free_space: u64 = 70000000 - total_size_g.get(&root).unwrap();
    let amount_of_trash = 30000000 - free_space;
    let mut best_sz = 30000000u64;
    for (_path, sz) in total_size_g {
        if sz >= amount_of_trash {
            if sz < best_sz {
                best_sz = sz;
            }
        }
    }
    best_sz
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn part_1() {
        let input = get_input(7);
        println!("{}", run_1(input.as_str()));
    }

    #[test]
    fn part_2() {
        let input = get_input(7);
        println!("{}", run_2(input.as_str()));
    }
}
