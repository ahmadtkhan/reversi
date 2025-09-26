use std::io;
const N: usize = 8;
fn main() {
    let mut val = init_board();
    let mut turn = 0;
    loop {
        println!("{}", calc_board(&val));
        let player = if turn == 0 { 'B' } else { 'W' };
        let opp;
        if player == 'B' {
            opp = 'W';
        } else {
            opp = 'B';
        }
        let check_legal = legal_moves(&val, player, opp);

        if check_legal.is_empty() {
            let opp_legal = legal_moves(&val, opp, player);
            if opp_legal.is_empty() {
                let (b, w) = count_dics(&val);
                println!("Game Over! Black: {b}, White: {w}");
                match b.cmp(&w) {
                    std::cmp::Ordering::Greater => println!("Black wins by {} points!", b - w),
                    std::cmp::Ordering::Equal => println!("Draw!"),
                    std::cmp::Ordering::Less => println!("White wins by {} points!", w - b),
                }
                break;
            } else {
                println!("{} player has no valid move", player);
                turn ^= 1;
                continue;
            }
        }

        println!("Enter move for colour {}: ", player);
        let mut user_input = String::new();
        if io::stdin().read_line(&mut user_input).is_err() {
            println!("Invalid move. Try again");
            continue;
        }

        let user_input = user_input.trim().to_lowercase();
        let player_move = parse_move(&user_input);
        let (x, y) = match player_move {
            Some(xy) => xy,
            None => {
                println!("Invalid move. Try again.");
                continue;
            }
        };
        if let Some(flips) = check_legal.iter().find_map(|(xx, yy, flips)| {
            if *xx == x && *yy == y {
                Some(flips.clone())
            } else {
                None
            }
        }) {
            apply_move(&mut val, player, x, y, &flips);
            turn ^= 1; // next player's turn
        } else {
            println!("Illegal move. Try again.");
        }
    }
}

fn init_board() -> [[char; N]; N] {
    let mut grid = [['.'; N]; N];
    grid[3][3] = 'W';
    grid[3][4] = 'B';
    grid[4][3] = 'B';
    grid[4][4] = 'W';
    grid
}

fn calc_board(board: &[[char; N]; N]) -> String {
    let mut s = String::new();
    s.push_str("  abcdefgh\n");
    for x in 0..N {
        s.push((b'a' + x as u8) as char);
        s.push(' ');
        for y in 0..N {
            s.push(board[x][y]);
        }
        s.push('\n');
    }
    s
}

fn parse_move(s: &str) -> Option<(usize, usize)> {
    let mut it = s.chars();
    let x = it.next()?;
    let y = it.next()?;
    if it.next().is_some() {
        return None;
    }
    let x = letter_to_index(x)?;
    let y = letter_to_index(y)?;
    Some((x, y))
}

fn letter_to_index(ch: char) -> Option<usize> {
    let cl = ch.to_ascii_lowercase();
    if ('a'..='h').contains(&cl) {
        Some((cl as u8 - b'a') as usize)
    } else {
        None
    }
}

fn apply_move(
    board: &mut [[char; N]; N],
    player: char,
    x: usize,
    y: usize,
    flips: &[(usize, usize)],
) {
    board[x][y] = player;
    for &(fx, fy) in flips {
        board[fx][fy] = player;
    }
}

fn legal_moves(
    board: &[[char; N]; N],
    player: char,
    opp: char,
) -> Vec<(usize, usize, Vec<(usize, usize)>)> {
    let mut result = Vec::new();
    for x in 0..N {
        for y in 0..N {
            if board[x][y] != '.' {
                continue;
            }
            if let Some(flips) = flipping(board, player, opp, x, y) {
                if !flips.is_empty() {
                    result.push((x, y, flips));
                }
            }
        }
    }
    result
}

fn flipping(
    board: &[[char; N]; N],
    player: char,
    opp: char,
    x: usize,
    y: usize,
) -> Option<Vec<(usize, usize)>> {
    if board[x][y] != '.' {
        return None;
    }
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1), //potential issue
    ];

    let mut all_flips = Vec::new();

    for (dirs_x, dirs_y) in directions {
        let mut x_point = x as isize + dirs_x;
        let mut y_point = y as isize + dirs_y;

        if !bound_check(x_point, y_point) || board[x_point as usize][y_point as usize] != opp {
            continue;
        }
        let mut captured = Vec::new();
        while bound_check(x_point, y_point) && board[x_point as usize][y_point as usize] == opp {
            captured.push((x_point as usize, y_point as usize));
            x_point += dirs_x;
            y_point += dirs_y;
        }
        if bound_check(x_point, y_point)
            && board[x_point as usize][y_point as usize] == player
            && !captured.is_empty()
        {
            all_flips.extend(captured);
        }
    }

    Some(all_flips)
}

fn bound_check(x: isize, y: isize) -> bool {
    x >= 0 && x < N as isize && y >= 0 && y < N as isize
}

fn count_dics(board: &[[char; N]; N]) -> (usize, usize) {
    let mut b = 0;
    let mut w = 0;
    for x in 0..N {
        for y in 0..N {
            match board[x][y] {
                'B' => b += 1,
                'W' => w += 1,
                _ => {}
            }
        }
    }
    (b, w)
}
