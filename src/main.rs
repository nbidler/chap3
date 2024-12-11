use std::io;

fn main() {
    
    let gifts = ["partridge in a pear tree", "turtledoves","french hens","calling birds","GOOOOOOOLD RIIIIIINGS", "geese a-laying", "swans a-swimming", "maids a-milking","ladies dancing","lords a-leaping", "pipers piping","drummers drumming"];
    let mut verse = 1;

    for _item in gifts {
        println!("On the {verse} day of Christmas, my true love gave to me:");
        for number in (0..verse).rev() {
            let present = gifts[number];
            println!("{} {present}", number + 1);
        }
        verse = verse + 1;
    }

    println!("Input a positive number LESS THAN 47");
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let fibo_num: u32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };
       
    //let answer = fibo(fibo_num);
    println!("The nth {fibo_num} fibonacci number is: {}", fibo(fibo_num));
}

fn fibo(input:u32) -> u32 {
    if input < 2 {
        return 1;
    }
    let mut sum = 0;
    let mut prev = 0;
    let mut current = 1;
    for _entry in 0..input {
        sum = current + prev;
        prev = current;
        current = sum;
        println!("{sum}")
    }
    sum
}