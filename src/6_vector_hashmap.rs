use std::collections::HashMap;

fn main () {
    let a = [1,2,3];
    let mut v1 = vec![1,2,3];
    let mut v2 = Vec::new();
    v1.push (4);
    v2.push (5);
    v2.push (6);
    println!("{:?}", v1);
    println!("{:?}", v2);
    
    //let four = v1[3];
    match v1.get(1) {
        Some(four) => println!("This is four element = {}", four),
        None => println!("This is not four element")
    }

    for i in &mut v1 {
        *i += 10;
    }
    for i in &v1 {
        println!("{} ", i);
    }


    //Hashmap
    let mu = String::from("MU");
    let mc=String::from("MC");
    let mut scores = HashMap::new();
    scores.insert(mu, 10);
    scores.insert(mc, 9);

    let team_name = String::from("MU");
    let score = scores.get(&team_name);

    for(key, value) in &scores {
        println!("{} {}", key, value);
    }

    println!("Score = {:?}", score);

    //hashmap vd2: count so phan tu trung 
    let text = "Hello world this is wonderful world";
    let mut map = HashMap::new();
    for i in text.split_whitespace() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}