use std::io;
fn main() {
    let mut t =String::new();
    io::stdin().read_line(&mut t).expect("cant read line");
    let t = t.trim().parse::<i32>().expect("is not number");
    let mut arr: Vec<(i64, i64)> = Vec::new();

    for _ in 0..t 
    {
       
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("cant read line");

        let mut parts = input.split_whitespace();
        let mut a: i64 = parts.next().unwrap().parse().unwrap();
        let mut b: i64 = parts.next().unwrap().parse().unwrap();
        let c;
        if b < a{
            c = a;
            a =b;
            b =c;
        }
        
        
       let mut count: i64 = 0;
       let mut sum = 0;

        for i in a..=b {
            if i % 3 == 0 && i % 5 != 0 {
                count += 1;
                sum += i;
            }
        }
        arr.push((count, sum));

    }
     for tuple in &arr {
        println!("{} {}", tuple.0, tuple.1);
    }
}
