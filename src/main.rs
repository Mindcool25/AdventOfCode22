use std::fs;


fn main() {
    println!("--- DAY 1 ---");
    day1();
    println!("");

    println!("--- DAY 2 ---");
    day2();
}

fn day1() {
    let file_path = "input1.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Failed to read file");

    let split_contents = contents.split("\n\n");

    let mut numbers: Vec<i32> = vec![0];
    for s in split_contents{
        let mut total = 0;
        for g in s.split('\n'){
            if !g.is_empty() {
                let num = g.parse::<i32>().expect("Failed to cast, probably not an issue.");
                total += num;
            }
        }
        numbers.push(total);
    }
    let mut current = 0;
    let mut current2 = 0;
    let mut current3 = 0;

    for i in numbers {
        if i > current {
            current3 = current2;
            current2 = current;
            current = i;
        }
        else if i > current2 {
            current3 = current2;
            current2 = i;
        }
        else if i > current3 {
            current3 = i;
        }
    }
    println!("Total top: {}", current);
    println!("Total of top 3: {}", current+current2+current3);
}

fn day2() {
    let rock_score = 1;
    let paper_score = 2;
    let scissors_score = 3;

    let lose_score = 0;
    let draw_score = 3;
    let win_score = 6;
}
