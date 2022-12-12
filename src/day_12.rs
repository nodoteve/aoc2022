use std::collections::VecDeque;


fn run_1(s: &str) -> i32 {
    let v:Vec<Vec<char>> = s.split("\n").map(|l|l.chars().collect()).collect();
    // println!(":{:?}",v);
    let mut s=(0,0);
    let mut e=(0,0);
    let mut h:Vec<Vec<i32>>=v.iter().map(|l|l.iter().map(|_|0).collect()).collect();
    let mut d:Vec<Vec<i32>>=v.iter().map(|l|l.iter().map(|_|-1).collect()).collect();
    for (y,row) in v.iter().enumerate(){
        for (x,c) in row.iter().enumerate(){
            // println!("{} {} {}",x,y,h.len());
            match c{
                'S'=>{
                    s=(y as i32,x as i32);
                    h[y][x]=0;
                }
                'E'=>{
                    e=(y ,x );
                    h[y][x]=(b'z'-b'a') as i32;
                }
                k=>h[y][x]=(*k as u8-b'a') as i32
            }

        }
    }
    // println!("{:?}",h);
    let mut q:VecDeque<(usize,usize)>=VecDeque::new();
    q.push_back((s.0 as usize, s.1 as usize));
    while !q.is_empty(){
        let (x,y)=q.pop_front().unwrap();
        let (x,y)=(x as usize, y as usize);
        let mut do_stuff=|a:i32,b:i32|{
            let (a,b) = ((x as i32+a) as usize, (y as i32+b) as usize);
            if h[a][b]-1<=h[x][y]{
                if (d[a][b]==-1 )||(d[a][b]>d[x][y]+1){
                    d[a][b]=d[x][y]+1;
                    q.push_back((a,b));
                }
            }
        };
        if x.checked_sub(1).is_some(){
            do_stuff(-1,0);
        }
        if y.checked_sub(1).is_some(){
            do_stuff(0,-1);
        }
        if x+1<h.len(){
            do_stuff(1,0);
        }
        if y+1<h[0].len(){
            do_stuff(0,1);
        }

    }
    // println!("{:?}",d);
    d[e.0][e.1]+1
}


fn run_2(s: &str) -> i32 {
    let v:Vec<Vec<char>> = s.split("\n").map(|l|l.chars().collect()).collect();
    // println!(":{:?}",v);
    let mut s=(0,0);
    let mut e=(0,0);
    let mut h:Vec<Vec<i32>>=v.iter().map(|l|l.iter().map(|_|0).collect()).collect();
    let mut d:Vec<Vec<i32>>=v.iter().map(|l|l.iter().map(|_|-1).collect()).collect();
    for (y,row) in v.iter().enumerate(){
        for (x,c) in row.iter().enumerate(){
            // println!("{} {} {}",x,y,h.len());
            match c{
                'S'=>{
                    s=(y as i32,x as i32);
                    h[y][x]=0;
                }
                'E'=>{
                    e=(y ,x );
                    h[y][x]=(b'z'-b'a') as i32;
                }
                k=>h[y][x]=(*k as u8-b'a') as i32
            }

        }
    }
    // println!("{:?}",h);
    let mut q:VecDeque<(usize,usize)>=VecDeque::new();
    q.push_back((e.0 as usize, e.1 as usize));

    let mut mind = (h[0].len()*h.len()+1) as i32;
    while !q.is_empty(){
        let (x,y)=q.pop_front().unwrap();
        let (x,y)=(x as usize, y as usize);
        let mut do_stuff=|a:i32,b:i32|{
            let (a,b) = ((x as i32+a) as usize, (y as i32+b) as usize);
            if h[a][b]+1>=h[x][y]{
                if (d[a][b]==-1 )||(d[a][b]>d[x][y]+1){
                    d[a][b]=d[x][y]+1;
                    q.push_back((a,b));
                }
            }
        };
        if x.checked_sub(1).is_some(){
            do_stuff(-1,0);
        }
        if y.checked_sub(1).is_some(){
            do_stuff(0,-1);
        }
        if x+1<h.len(){
            do_stuff(1,0);
        }
        if y+1<h[0].len(){
            do_stuff(0,1);
        }
        if h[x][y]==0{
            mind=mind.min(d[x][y]);
        }
    }
    // println!("{:?}",d);
    mind+1;
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn part_1() {
        let input = get_input(12);
        println!("{}", run_1(input.as_str()));
    }

    #[test]
    fn part_2() {
        let input = get_input(12);
        println!("{}", run_2(input.as_str()));
    }
}
