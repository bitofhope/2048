use ncurses;
use getrandom::getrandom;

#[derive(Clone,Copy)]
enum Direction {
    Down,
    Left,
    Right,
    Up
}

fn main() {
    let stdscr = ncurses::initscr();
    ncurses::cbreak();
    ncurses::noecho();
    ncurses::keypad(stdscr, true);

    let mut board = [[0u64; 4]; 4];
    let mut score = 0;

    generate_square(&mut board);
    while check_moves(&board) {
        draw_board(&board);
        ncurses::mvprintw(4, 0, &format!("Score: {}", score));
        let direction = match get_direction() {
            Some(d) => d,
            None => {
                ncurses::endwin();
                return;
            }
        };
        match move_board(&mut board, direction) {
            Some(s) => {
                score += s;
                generate_square(&mut board);
            },
            None => ()
        }
    }
    ncurses::mvprintw(5, 0, "Game over.");

    ncurses::getch();
    ncurses::endwin();
}

fn generate_square(board: &mut [[u64; 4]; 4]) {
    let mut rndbuf = [0u8];
    match getrandom(&mut rndbuf) {
        Ok(()) => (),
        Err(e) => {
            ncurses::endwin();
            panic!("getrandom() failed: {}", e);
        }
    }
    let mut random = rndbuf[0];
    let new_square: u64 = if random % 2 == 0 { 2 } else { 4 };
    let mut x = 0;
    let mut y = 0;
    while random > 0 {
        y = (y + 1) % board[x].len();
        if y == 0 {
            x = (x + 1) % board.len();
        }
        if board[x][y] == 0 {
            random -= 1;
        }
    }
    board[x][y] = new_square;
}

fn check_moves(board: &[[u64; 4]; 4]) -> bool {
    for x in 0..board.len() {
        for y in 0..board[x].len() {
            if board[x][y] == 0 {
                return true;
            } else if (x > 0) && (board[x-1][y] == board[x][y]) {
                return true;
            } else if (x < 3) && (board[x+1][y] == board[x][y]) {
                return true;
            } else if (y > 0) && (board[x][y-1] == board[x][y]) {
                return true;
            } else if (y < 3) && (board[x][y+1] == board[x][y]) {
                return true;
            }
        }
    }
    false
}

fn draw_board(board: &[[u64; 4]; 4]) {
    for x in 0..board.len() {
        for y in 0..board.len() {
            let xx: i32 = x.try_into().unwrap();
            let yy: i32 = (6 * y).try_into().unwrap();
            ncurses::mvprintw(xx, yy, &format!("{:5}", board[x][y]));
        }
    }
}

fn get_direction() -> Option<Direction> {
    const H: i32 = 'h' as i32;
    const J: i32 = 'j' as i32;
    const K: i32 = 'k' as i32;
    const L: i32 = 'l' as i32;
    const Q: i32 = 'q' as i32;
    loop {
        match ncurses::getch() {
            ncurses::constants::KEY_DOWN  | H => return Some(Direction::Down),
            ncurses::constants::KEY_UP    | J => return Some(Direction::Up),
            ncurses::constants::KEY_LEFT  | K => return Some(Direction::Left),
            ncurses::constants::KEY_RIGHT | L => return Some(Direction::Right),
            Q => return None,
            _ => ()
        }
    }
}

fn move_board(board: &mut [[u64; 4]; 4], dir: Direction) -> Option<u64> {
    let mut score: u64 = 0;
    let mut moved = false;
    for x in 0..board.len() {
        for y in 0..board[x].len() {
            let (xx, yy) = match dir {
                Direction::Up => (x, y),
                Direction::Down => (3-x, y),
                Direction::Left => (x, y),
                Direction::Right => (x, 3-y)
            };
            match move_square(board, xx, yy, dir) {
                Some(s) => {
                    score += s; 
                    moved = true;
                },
                None => ()
            }
        }
    }
    if moved {
        Some(score)
    } else {
        None
    }
}

fn move_square(board: &mut [[u64; 4]; 4], x: usize, y: usize, dir: Direction) -> Option<u64> {
    if board[x][y] == 0 {
        return None;
    }
    let mut next_x: usize = x;
    let mut next_y: usize = y;
    match (&dir, x, y) {
        (Direction::Up,    0, _) => { return None },
        (Direction::Up,    _, _) => { next_x = x - 1 },
        (Direction::Down,  3, _) => { return None },
        (Direction::Down,  _, _) => { next_x = x + 1 },
        (Direction::Left,  _, 0) => { return None },
        (Direction::Left,  _, _) => { next_y = y - 1 },
        (Direction::Right, _, 3) => { return None },
        (Direction::Right, _, _) => { next_y = y + 1 }
    }
    if board[next_x][next_y] == board[x][y]  {
        // Merge
        board[next_x][next_y] += board[x][y];
        board[x][y] = 0;
        Some(board[next_x][next_y])
    } else if board[next_x][next_y] == 0 {
        // Slide
        board[next_x][next_y] = board[x][y];
        board[x][y] = 0;
        move_square(board, next_x, next_y, dir);
        Some(0)
    } else {
        None
    }
}
