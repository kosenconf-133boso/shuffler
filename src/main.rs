use std::env;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let args: Vec<String> = env::args().collect();

    let col = args[1].parse::<u32>().unwrap();
    let paticipants = args[2].parse::<u32>().unwrap();

    let mut numbers: Vec<u32> = (1..paticipants+1).collect();
    let mut rng = thread_rng();
    numbers.shuffle(&mut rng);

    for i in 0..paticipants {
        let j = i as usize;
        match (i+1)%col {
            0 => println!(" {:?}", numbers[j]),
            _ => print!(" {:?}", numbers[j])
        }
    }

    println!("");
}
