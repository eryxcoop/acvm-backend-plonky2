struct Square {
    x: Field,
    y: Field
}

fn main(
    watcher_map: pub [[bool; 8]; 8],
    dagger: pub Square,
    path: [Square; 8]
) {
    is_valid_path(path, dagger);
    is_safe_path(path, watcher_map);
}

fn in_board_limits(elem: Field) -> bool {
    (elem == 0) | (elem == 1) | (elem == 2) | (elem == 3) | (elem == 4) | (elem == 5) | (elem == 6) | (elem == 7)
}

fn is_valid_square(sq: Square) -> bool {
    in_board_limits(sq.x) & in_board_limits(sq.y)
}

fn is_valid_jump(sq1: Square, sq2: Square) -> bool {
    let mut dx = sq1.x - sq2.x;
    if !in_board_limits(dx) {
        dx = sq2.x - sq1.x;
    }
    let mut dy = sq1.y - sq2.y;
    if !in_board_limits(dy) {
        dy = sq2.y - sq1.y;
    }

    let a = (dx == 1) & (dy == 2);
    let b = (dx == 2) & (dy == 1);
    a | b
}

fn is_initial_square(sq: Square) -> bool {
    (sq.x == 0) & (sq.y == 0)
}

pub fn is_valid_path(path: [Square; 8], dagger: Square) {
    let mut is_valid: bool = is_initial_square(path[0]);

    for i in 1..8 {
        is_valid &= is_valid_square(path[i]);
        is_valid &= is_valid_jump(path[i], path[i-1]);
    }
    is_valid &= (dagger.x == path[7].x) & (dagger.y == path[7].y);
    assert(is_valid);
}

fn prohibited_map_from(wm: [[bool; 8]; 8]) -> [[bool; 8]; 8] {
    let mut prohibited_map: [[bool; 8]; 8] = [[false; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            if wm[i][j] {
                for k in 0..8 {
                    let i: Field = i as Field;
                    let j: Field = j as Field;
                    let k: Field = k as Field;
                    if in_board_limits(i-k) & in_board_limits(j-k) {
                        prohibited_map[i-k][j-k] = true;
                    }
                    if in_board_limits(i-k) & in_board_limits(j+k) {
                        prohibited_map[i-k][j+k] = true;
                    }
                    if in_board_limits(i+k) & in_board_limits(j-k) {
                        prohibited_map[i+k][j-k] = true;
                    }
                    if in_board_limits(i+k) & in_board_limits(j+k) {
                        prohibited_map[i+k][j+k] = true;
                    }
                }
            }
        }
    }
    prohibited_map
}

pub fn is_safe_path(path: [Square; 8], watcher_map: [[bool; 8]; 8]) {
    let prohibited_map: [[bool; 8]; 8] = prohibited_map_from(watcher_map);
    let mut unsafe: bool = false;
    for i in 0..8 {
        if prohibited_map[path[i].x][path[i].y] {
            unsafe = true;
        }
    }
    assert(!unsafe);
}