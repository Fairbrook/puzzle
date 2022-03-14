use super::Node;

const TOP_LEFT: i64 = 0x100000000;
const TOP_CENTER: i64 = 0x10000000;
const TOP_RIGHT: i64 = 0x1000000;

const CENTER_LEFT: i64 = 0x100000;
const CENTER_CENTER: i64 = 0x10000;
const CENTER_RIGHT: i64 = 0x1000;

const BOTTOM_LEFT: i64 = 0x100;
const BOTTOM_CENTER: i64 = 0x10;
const BOTTOM_RIGHT: i64 = 0x1;

#[derive(Clone)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct State {
    data: i64,
    space_pos: i64,
    pub steps: Option<Vec<Move>>,
}

impl State {
    pub fn from(array: &[[i32; 3]; 3]) -> Self {
        let mut mul: i64 = 0x100000000;
        let mut data: i64 = 0;
        let mut space_pos: i64 = 0;
        for row in array {
            for number in row {
                if *number == 0 {
                    space_pos = mul;
                }
                data = data | ((*number & 0xf) as i64 * mul);
                mul >>= 4;
            }
        }
        Self {
            data: data,
            space_pos: space_pos,
            steps: None,
        }
    }
    pub fn swap_empty(data: i64, space_pos: i64, final_pos: i64) -> i64 {
        if final_pos == 0 {
            return 0;
        }
        let mask = 0xF * final_pos;
        let value = (data & mask) / final_pos;
        return (data & !mask) | (value * space_pos);
    }
    pub fn to_slice(&self) -> [[i32; 3]; 3] {
        [
            [
                ((self.data & (0xf * TOP_LEFT)) / TOP_LEFT) as i32,
                ((self.data & (0xf * TOP_CENTER)) / TOP_CENTER) as i32,
                ((self.data & (0xf * TOP_RIGHT)) / TOP_RIGHT) as i32,
            ],
            [
                ((self.data & (0xf * CENTER_LEFT)) / CENTER_LEFT) as i32,
                ((self.data & (0xf * CENTER_CENTER)) / CENTER_CENTER) as i32,
                ((self.data & (0xf * CENTER_RIGHT)) / CENTER_RIGHT) as i32,
            ],
            [
                ((self.data & (0xf * BOTTOM_LEFT)) / BOTTOM_LEFT) as i32,
                ((self.data & (0xf * BOTTOM_CENTER)) / BOTTOM_CENTER) as i32,
                ((self.data & (0xf * BOTTOM_RIGHT)) / BOTTOM_RIGHT) as i32,
            ],
        ]
    }
    pub fn move_space(&self, to: &Move) -> Self {
        let final_pos = match to {
            Move::Up => self.space_pos << 12,
            Move::Down => self.space_pos >> 12,
            Move::Left => self.space_pos << 4,
            Move::Right => self.space_pos >> 4,
        };
        let steps: Vec<Move> = match &self.steps {
            None => vec![to.clone()],
            Some(parent) => {
                let mut copy = parent.clone();
                copy.push(to.clone());
                copy
            }
        };
        Self {
            data: State::swap_empty(self.data, self.space_pos, final_pos),
            space_pos: final_pos,
            steps: Some(steps),
        }
    }
}

impl Node for State {
    fn equ(&self, a: &State) -> bool {
        return self.data == a.data;
    }
    fn expand<'a>(&'a self) -> Vec<Box<Self>> {
        let mut children = vec![];
        match self.space_pos {
            TOP_LEFT => {
                children.push(Box::new(self.move_space(&Move::Right)));
                children.push(Box::new(self.move_space(&Move::Down)));
            }
            TOP_CENTER => {
                children.push(Box::new(self.move_space(&Move::Left)));
                children.push(Box::new(self.move_space(&Move::Right)));
                children.push(Box::new(self.move_space(&Move::Down)));
            }
            TOP_RIGHT => {
                children.push(Box::new(self.move_space(&Move::Left)));
                children.push(Box::new(self.move_space(&Move::Down)));
            }
            CENTER_LEFT => {
                children.push(Box::new(self.move_space(&Move::Right)));
                children.push(Box::new(self.move_space(&Move::Down)));
                children.push(Box::new(self.move_space(&Move::Up)));
            }
            CENTER_CENTER => {
                children.push(Box::new(self.move_space(&Move::Left)));
                children.push(Box::new(self.move_space(&Move::Right)));
                children.push(Box::new(self.move_space(&Move::Down)));
                children.push(Box::new(self.move_space(&Move::Up)));
            }
            CENTER_RIGHT => {
                children.push(Box::new(self.move_space(&Move::Left)));
                children.push(Box::new(self.move_space(&Move::Down)));
                children.push(Box::new(self.move_space(&Move::Up)));
            }
            BOTTOM_LEFT => {
                children.push(Box::new(self.move_space(&Move::Right)));
                children.push(Box::new(self.move_space(&Move::Up)));
            }
            BOTTOM_CENTER => {
                children.push(Box::new(self.move_space(&Move::Left)));
                children.push(Box::new(self.move_space(&Move::Right)));
                children.push(Box::new(self.move_space(&Move::Up)));
            }
            BOTTOM_RIGHT => {
                children.push(Box::new(self.move_space(&Move::Left)));
                children.push(Box::new(self.move_space(&Move::Up)));
            }
            _ => {}
        };
        children
        // }

        // self.children.borrow().clone()
    }
    fn to_string(&self) -> String {
        let array = self.to_slice();
        let mut res = String::from("");
        for row in array {
            res += &row
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join(" | ");
            res += "\n"
        }
        res
    }
    fn get_data(&self) -> i64 {
        return self.data;
    }
}
