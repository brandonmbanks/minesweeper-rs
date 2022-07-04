use crate::random::random_num;

use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

type Position = (usize, usize);

pub enum RevealResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    num_mines: usize,
    mines: HashSet<Position>,
    flags: HashSet<Position>,
    lost: bool,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);

                if !self.open_fields.contains(&pos) {
                    if self.flags.contains(&pos) {
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("⬜ ")?;
                    }
                } else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    write!(f, "{} ", self.num_neighboring_mines(pos))?;
                }
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, num_mines: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            num_mines: num_mines,
            mines: HashSet::new(),
            flags: HashSet::new(),
            lost: false,
        }
    }

    fn populate_mines(&mut self, pos: Position) {
        let mut mines = HashSet::new();

        while mines.len() < self.num_mines {
            let x = random_num(0, self.width);
            let y = random_num(0, self.width);

            if (x, y) != pos {
                mines.insert((x, y));
            }
        }

        self.mines = mines;
    }

    fn neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    fn num_neighboring_mines(&self, pos: Position) -> u8 {
        self.neighbors(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn reveal(&mut self, pos: Position) -> Option<RevealResult> {
        if self.flags.contains(&pos) || self.lost {
            return None;
        }

        if self.open_fields.is_empty() {
            self.populate_mines(pos);
        }

        self.open_fields.insert(pos);

        let is_mine = self.mines.contains(&pos);
        if is_mine {
            self.lost = true;
            Some(RevealResult::Mine)
        } else {
            Some(RevealResult::NoMine(self.num_neighboring_mines(pos)))
        }
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.open_fields.contains(&pos) || self.lost {
            return;
        }

        if self.flags.contains(&pos) {
            self.flags.remove(&pos);
        } else {
            self.flags.insert(pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::minesweeper::Minesweeper;

    #[test]
    fn test() {
        let mut ms = Minesweeper::new(10, 10, 5);

        ms.reveal((5, 5));
        ms.toggle_flag((6, 6));
        ms.reveal((6, 6));

        println!("{}", ms);
    }
}
