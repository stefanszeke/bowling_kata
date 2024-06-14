console.log("weee");

let test_rolls = "23 8/ 15 7/ X 17 81 4/ 70 XX1";

let bowlingScore2 = function (rolls) {
  rolls = rolls.replace(/\s/g, "").split("");

  rolls = rolls.map((char, i) => {
    if (char == "X") { return 10 }
    if (char == "/") { return 10 - parseInt(rolls[i - 1]) };
    return parseInt(char);
  });

  console.log(rolls);

  total_score = 0;
  let frame_index = 0;
  let frame = 1;

  while (frame <= 10) {
    let score = rolls[frame_index] + rolls[frame_index + 1];
    total_score += score;
    if (score >= 10) {
      total_score += rolls[frame_index + 2];
    }
    if (rolls[frame_index] != 10) {
      frame_index++;
    }

    frame_index++;
    frame++;
  }
  return total_score;
};

let bowlingScore = function (rolls) {
  let frames = [];
  let frame_index = 0;
  let frame = 1;
  while (true) {
    if (frame > 10) {
      break;
    }

    if (rolls[frame_index] != 10) {
      total = rolls[frame_index] + rolls[frame_index + 1];
      if (total == 10) {
        total += rolls[frame_index + 2];
      }
      frames.push(total);
      frame_index += 2;
    } else {
      total = 10 + rolls[frame_index + 1] + rolls[frame_index + 2];
      frames.push(total);
      frame_index += 1;
    }
    frame += 1;
  }

  return frames.reduce((a, b) => a + b, 0);
};

let score = bowlingScore2(test_rolls);
console.log(score);
