fn main() {
    println!("Hello, world!");

    let test_input: String = String::from("23 8/ 15 7/ X 17 81 4/ 70 XX1");
    // 23 -> 5
    // 8/ -> 10 + 1 = 11 
    // 15 -> 6
    // 7/ -> 10 + 10 = 20
    // X -> 10 + 1 + 7 = 18
    // 17 -> 8
    // 81 -> 9
    // 4/ -> 10 + 7 = 17
    // 70 -> 7
    // XX1 -> 10 + 10 + 1 = 21
    // 5 + 11 + 6 + 20 + 18 + 8 + 9 + 17 + 7 + 21 = 122

    println!("{}", bowling_score(&test_input))
}

fn bowling_score(rolls_string: &String) -> usize {

    let mut rolls: Vec<&str> = rolls_string.split("").filter(|s| !s.is_empty() && *s != " ").collect();

    println!("\nrolls:");
    println!("{:?}", rolls);

    let mut rolls: Vec<usize> = rolls.iter().enumerate().map(|(i, &s)| {
        match s {
            "X" => 10,
            "/" => 10 - rolls[i - 1].parse::<usize>().unwrap(),
            _ => s.parse::<usize>().unwrap()
        }
    }).collect();


    println!("\nrolls mapped:");
    println!("{:?}", rolls);


    let mut frame_index:usize = 0;
    let mut score:usize = 0;

    for i in 1..=10 {
        println!("frame: {} | frame i: {}",i, frame_index);

        let frame_score = rolls[frame_index] + rolls[frame_index + 1];
        score = score + frame_score;
        
        if frame_score >= 10 {
            score = score + rolls[frame_index + 2];
        }

        if rolls[frame_index] != 10 {
            frame_index += 1;
        }

        frame_index += 1;
    }

    score
}