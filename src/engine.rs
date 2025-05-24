use std::{
    collections::HashMap,
    fmt,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    White,
    Black,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::White => write!(f, "1"),
            Self::Black => write!(f, "2"),
        }
    }
}

impl Player {
    pub fn next(self) -> Player {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
pub struct Loc {
    pub row: usize,
    pub col: usize,
}

impl Loc {
    pub fn move_in_by(&self, dir: Dir, count: usize) -> Loc {
        match dir {
            Dir::North => Loc {
                row: self.row - count,
                col: self.col,
            },
            Dir::East => Loc {
                row: self.row,
                col: self.col + count,
            },
            Dir::South => Loc {
                row: self.row + count,
                col: self.col,
            },
            Dir::West => Loc {
                row: self.row,
                col: self.col - count,
            },
        }
    }

    pub fn move_in(&self, dir: Dir) -> Loc {
        self.move_in_by(dir, 1)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StoneType {
    Flat,
    Standing,
    Capstone,
}

impl fmt::Display for StoneType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Flat => Ok(()),
            Self::Standing => write!(f, "S"),
            Self::Capstone => write!(f, "C"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Stone {
    owner: Player,
    typ: StoneType,
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.owner, self.typ)
    }
}

#[derive(Debug)]
pub enum Turn {
    Place {
        loc: Loc,
        player: Player,
        typ: StoneType,
    },
    Move {
        loc: Loc,
        player: Player,
        dir: Dir,
        total: usize,
        drops: Vec<usize>,
    },
}

impl Turn {
    pub fn player(&self) -> Player {
        match self {
            Self::Place {
                loc: _,
                player,
                typ: _,
            } => *player,
            Self::Move {
                loc: _,
                player,
                dir: _,
                total: _,
                drops: _,
            } => *player,
        }
    }
}

#[derive(Debug)]
struct Board(Vec<Vec<Vec<Stone>>>);

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for stack in row {
                if stack.len() == 0 {
                    write!(f, "x")?
                } else {
                    for stone in stack {
                        write!(f, "{}", stone)?
                    }
                }
                write!(f, ",")?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

impl Board {
    fn new(size: usize) -> Self {
        Board(
            std::iter::repeat_with(|| {
                std::iter::repeat_with(|| Vec::new())
                    .take(size.into())
                    .collect()
            })
            .take(size.into())
            .collect(),
        )
    }

    fn size(&self) -> usize {
        self.0.len()
    }

    fn valid_loc(&self, loc: Loc) -> bool {
        loc.row < self.size() && loc.col < self.size()
    }

    fn valid_turn(&self, turn: &Turn) -> bool {
        match turn {
            Turn::Place {
                loc,
                player: _,
                typ: _,
            } => self.valid_loc(*loc) && self[*loc].is_empty(),
            Turn::Move {
                loc,
                player,
                dir,
                total,
                drops,
            } => {
                // Drops is nonempty
                if !(drops.len() > 0) {
                    return false;
                }
                // Total is at most the carry limit
                if !(*total <= self.size()) {
                    return false;
                }
                // Total is less than is there
                // Doesn't pick up more than is there
                if !(*total <= self[*loc].len()) {
                    return false;
                }
                // Drops sums to total
                if !(drops.iter().sum::<usize>() == *total) {
                    return false;
                }
                // Starts on the board
                if !(self.valid_loc(*loc)) {
                    return false;
                }
                // Doesn't leave the board
                if !(self.valid_loc(loc.move_in_by(*dir, drops.len()))) {
                    return false;
                }
                // Top stone is correct player
                let top_here = self[*loc].last().unwrap();
                if !(top_here.owner == *player) {
                    return false;
                }
                // Only the capstone (alone) can crush walls, nothing can stack capstones
                let mut next_loc = *loc;
                for drop in drops {
                    next_loc = next_loc.move_in(*dir);
                    if let Some(top_there) = self[next_loc].last() {
                        if matches!(top_there.typ, StoneType::Standing)
                            && !(matches!(top_here.typ, StoneType::Capstone) && *drop == 1)
                        {
                            return false;
                        }
                        if matches!(top_there.typ, StoneType::Capstone) {
                            return false;
                        }
                    }
                }
                true
            }
        }
    }

    fn apply_turn(&mut self, turn: &Turn) {
        match turn {
            Turn::Place { loc, player, typ } => self[*loc].push(Stone {
                owner: *player,
                typ: *typ,
            }),
            Turn::Move {
                loc,
                player: _,
                dir,
                total,
                drops,
            } => {
                let stack_here = &mut self[*loc];
                let mut held_stack = stack_here.split_off(stack_here.len() - total);
                let mut next_loc = *loc;

                for drop in drops {
                    next_loc = next_loc.move_in(*dir);
                    let new_held_stack = held_stack.split_off(*drop);
                    if let Some(stack_top) = self[next_loc].last_mut() {
                        stack_top.typ = StoneType::Flat
                    }
                    self[next_loc].append(&mut held_stack);
                    held_stack = new_held_stack;
                }
            }
        }
    }
}

impl Index<Loc> for Board {
    type Output = Vec<Stone>;

    fn index(&self, index: Loc) -> &Self::Output {
        &self.0[index.row as usize][index.col as usize]
    }
}

impl IndexMut<Loc> for Board {
    fn index_mut(&mut self, index: Loc) -> &mut Self::Output {
        &mut self.0[index.row as usize][index.col as usize]
    }
}

#[derive(Debug, Clone, Copy)]
struct Reserve {
    reg: u8,
    cap: u8,
}

#[derive(Debug)]
pub struct GameState {
    current_player: Player,
    board: Board,
    reserves: HashMap<Player, Reserve>,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "current_player: {:?},\nreserves: {:?},\nboard:\n{},",
            self.current_player, self.reserves, self.board
        )
    }
}

impl GameState {
    pub fn new(size: usize) -> GameState {
        let reserve = match size {
            3 => Reserve { reg: 10, cap: 0 },
            4 => Reserve { reg: 15, cap: 0 },
            5 => Reserve { reg: 21, cap: 1 },
            6 => Reserve { reg: 30, cap: 1 },
            7 => Reserve { reg: 40, cap: 2 },
            8 => Reserve { reg: 50, cap: 2 },
            _ => panic!("Board size should be between 3 and 8 for a valid game"),
        };
        GameState {
            current_player: Player::White,
            board: Board::new(size),
            reserves: HashMap::from([(Player::White, reserve), (Player::Black, reserve)]),
        }
    }

    pub fn valid_turn(&self, turn: &Turn) -> bool {
        if !(turn.player() == self.current_player) {
            return false;
        }
        if let Turn::Place {
            loc: _,
            player: _,
            typ,
        } = turn
        {
            match typ {
                StoneType::Flat | StoneType::Standing => {
                    if self.reserves[&turn.player()].reg == 0 {
                        return false;
                    }
                }
                StoneType::Capstone => {
                    if self.reserves[&turn.player()].cap == 0 {
                        return false;
                    }
                }
            }
        }
        self.board.valid_turn(turn)
    }

    pub fn apply_turn(&mut self, turn: &Turn) -> bool {
        if !(self.valid_turn(turn)) {
            return false;
        }

        self.board.apply_turn(turn);
        self.current_player = self.current_player.next();
        if let Turn::Place {
            loc: _,
            player: _,
            typ,
        } = turn
        {
            self.reserves
                .entry(turn.player())
                .and_modify(|res| match typ {
                    StoneType::Flat | StoneType::Standing => res.reg -= 1,
                    StoneType::Capstone => res.cap -= 1,
                });
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut state = GameState::new(5);
        println!("{}", state);
        assert!(state.apply_turn(&Turn::Place {
            loc: Loc { row: 0, col: 0 },
            player: Player::White,
            typ: StoneType::Flat,
        }));
        println!("{}", state);
        assert!(state.apply_turn(&Turn::Place {
            loc: Loc { row: 1, col: 0 },
            player: Player::Black,
            typ: StoneType::Flat,
        }));
        println!("{}", state);
        assert!(state.apply_turn(&Turn::Place {
            loc: Loc { row: 2, col: 0 },
            player: Player::White,
            typ: StoneType::Standing,
        }));
        println!("{}", state);
        assert!(state.apply_turn(&Turn::Place {
            loc: Loc { row: 1, col: 1 },
            player: Player::Black,
            typ: StoneType::Flat,
        }));
        println!("{}", state);
        assert!(state.apply_turn(&Turn::Move {
            loc: Loc { row: 0, col: 0 },
            player: Player::White,
            dir: Dir::South,
            total: 1,
            drops: vec![1],
        }));
        println!("{}", state);
        assert!(state.apply_turn(&Turn::Move {
            loc: Loc { row: 1, col: 1 },
            player: Player::Black,
            dir: Dir::West,
            total: 1,
            drops: vec![1],
        }));
        println!("{}", state);
        assert!(state.apply_turn(&Turn::Move {
            loc: Loc { row: 2, col: 0 },
            player: Player::White,
            dir: Dir::North,
            total: 1,
            drops: vec![1],
        }));
        println!("{}", state);
        assert!(state.apply_turn(&Turn::Place {
            loc: Loc { row: 0, col: 3 },
            player: Player::Black,
            typ: StoneType::Capstone,
        }));
        println!("{}", state);
        assert!(state.apply_turn(&Turn::Move {
            loc: Loc { row: 1, col: 0 },
            player: Player::White,
            dir: Dir::East,
            total: 4,
            drops: vec![2, 1, 1],
        }));
        println!("{}", state);
        assert!(state.apply_turn(&Turn::Move {
            loc: Loc { row: 0, col: 3 },
            player: Player::Black,
            dir: Dir::South,
            total: 1,
            drops: vec![1],
        }));
        println!("{}", state);
    }
}
