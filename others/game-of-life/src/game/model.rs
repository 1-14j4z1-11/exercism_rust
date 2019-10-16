use super::array::Array2;
use super::rule::Rule;

pub struct GameModel {
    map: Array2<bool>,
    rule: Rule,
}

impl GameModel {
    pub fn new(width: usize, height: usize, rule: Rule) -> Self {
        GameModel {
            map: Array2::new(width, height),
            rule,
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.map.width()
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.map.height()
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.map.get(x, y)
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.map.set(x, y, value);
    }

    pub fn next(&mut self) {
        let prev_map = self.map.clone();

        for y in 0..prev_map.height() {
            for x in 0..prev_map.width() {
                let n_lives = Self::living_neighbours(&prev_map, x, y);
                self.map
                    .set(x, y, self.rule.next_state(self.map.get(x, y), n_lives));
            }
        }
    }

    pub fn to_str_lines(&self, live: char, dead: char) -> Vec<String> {
        let mut lines = vec![];
        
        for y in 0..self.height() {
            let mut line = String::new();

            for x in 0..self.width() {
                line.push(if self.get(x, y) {
                    live
                } else {
                    dead
                });
            }

            lines.push(line);
        }

        lines
    }

    pub fn from_str_lines(lines: &[&str], live: char, rule: Rule) -> Option<Self> {
        if lines.len() == 0 {
            return None;
        }

        let width = match lines.iter().filter(|l| l.len() > 0).map(|l| l.len()).min() {
            None => return None,
            Some(w) => w,
        };

        let mut model = Self::new(width, lines.len(), rule);

        for (y, &line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                model.set(x, y, c == live);
            }
        }

        return Some(model);
    }

    fn living_neighbours(map: &Array2<bool>, x: usize, y: usize) -> usize {
        let mut n_lives = 0;

        for &j in &[y.checked_sub(1), Some(y), y.checked_add(1)] {
            for &i in &[x.checked_sub(1), Some(x), x.checked_add(1)] {
                let (nx, ny) = match (i, j) {
                    (Some(i), Some(j)) => (i, j),
                    _ => continue,
                };

                if nx == x && ny == y {
                    continue;
                }

                if nx >= map.width() || ny >= map.height() {
                    continue;
                }

                if map.get(nx, ny) {
                    n_lives += 1;
                }
            }
        }

        n_lives
    }
}
