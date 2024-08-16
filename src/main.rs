use rand::Rng;

fn main() {
    const WIDTH: i8 = 10;
    const HEIGHT: i8 = 10;
    let mut len = 3;

    let mut food = (5, 5);
    let mut snake: Vec<(i8, i8)> = Vec::new();
    let mut head = (0, 0);
    snake.push(head);
    
    // game loop
    for _ in 0..20 {
        // Render Board
        print!("╔");
        for _ in 0..WIDTH {
            print!("═");
        }
        println!("╗");
        for y in 0..HEIGHT {
            print!("║");
            for x in 0..WIDTH {
                if (x, y) == food {
                    print!("×");
                } else if (x, y) == head {
                    print!("▓");
                } else if snake.contains(&(x, y)) {
                    print!("▒");
                } else {
                    print!(" ");
                }
            }
            println!("║");
        }
        print!("╚");
        for _ in 0..WIDTH {
            print!("═");
        }
        println!("╝");

        // Move Snake
        if snake.len() >= len {
            snake.remove(0);
        }
        match input().trim() {
            "w" => head = (head.0, head.1 - 1),
            "a" => head = (head.0 - 1, head.1),
            "s" => head = (head.0, head.1 + 1),
            "d" => head = (head.0 + 1, head.1),
            _ => println!("Invalid move")
            
        }
        snake.push(head);

        // Check if snake eats food
        if snake.last().unwrap() == &food {
            len += 1;
            let mut rng = rand::thread_rng();
            food = (rng.gen::<i8>().abs() % WIDTH, rng.gen::<i8>().abs() % HEIGHT);
            println!("Food eaten {:?}", food);
        }
    }
}

fn input() -> String {
    // https://www.tutorialspoint.com/rust/rust_input_output.htm
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line;
}