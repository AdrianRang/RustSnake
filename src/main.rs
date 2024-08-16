fn main() {
    const WIDTH: i8 = 10;
    const HEIGHT: i8 = 10;

    let food = [5, 5];
    let mut snake: Vec<(i8, i8)> = Vec::new();

    snake.push((2, 1));
    snake.push((2, 2));
    snake.push((3, 2));

    
    // Render Board
    for y in 0..HEIGHT {
        print!("|");
        for x in 0..WIDTH {
            if [x, y] == food {
                print!("F");
            } else if snake.contains(&(x, y)) {
                print!("S");
            } else {
                print!(" ");
            }
        }
        println!("|");
    }
}

fn input() -> String {
    // https://www.tutorialspoint.com/rust/rust_input_output.htm
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line;
}