use std::io;
use std::io::Write;
fn main() {
    let mut val = init_board();
    let mut turn = 0;
    /*
    struct B{
        potential_vals: [[u16; 8]; 8] = [[0; 8]; 8],
    }
     */
    //let mut potential_vals_B: [[u16; 8]; 8] = [[0; 8]; 8];
    //let mut potential_vals_W: [[u16; 8]; 8] = [[0; 8]; 8];
    println!("{}", val);
    loop {
        let mut display_turn = 'B';
        if turn == 0 {
            display_turn = 'B'
        } else {
            display_turn = 'W'
        };
        let mut user_input = String::new();
        println!("Enter move for colour {display_turn} (RowCol): ");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Enter a valid line");
        //io::stdout().flush().expect("Failed to flush stdout.");
        calc_board(&mut turn, &mut val);
    }
    //println!("{}", val);
}
fn init_board() -> String {
    let mut body = String::from("  abcdefgh\n");
    let col_name: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let mut grid: [[char; 8]; 8] = [['.'; 8]; 8];
    grid[3][3] = 'W';
    grid[3][4] = 'B';
    grid[4][3] = 'B';
    grid[4][4] = 'W';
    //let mut body = String::new();
    for n in 0..=grid.len() - 1 {
        let my_str: String = grid[n].iter().cloned().collect();
        let slice: &str = &my_str[..];
        body.push(col_name[n]);
        body.push_str(" ");
        body.push_str(slice);
        body.push_str("\n");
    }
    body
}
fn calc_board(user_input: &mut i32, input: &mut String) {}
