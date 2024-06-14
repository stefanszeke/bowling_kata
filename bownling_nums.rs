fn main() {
    println!("Hello, world!");

    let test_input: Vec<usize> = vec![10 , 9,1 , 9,1 , 9,1 , 9,1 , 9,1 , 9,1 , 9,1 , 9,1 , 9,1, 9];
    println!("{}", bowling_score(&test_input))
}

fn bowling_score(rolls: &Vec<usize>) -> usize {

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


    println!("rolls: {:?}", rolls);
    score
}