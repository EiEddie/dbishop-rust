use std::{
    fmt,
    ops::{Add, AddAssign},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pos(i32, i32);

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        return Pos(self.0 + rhs.0, self.1 + rhs.1);
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

struct Field {
    width: usize,
    height: usize,
    vals: Vec<i32>,
    move_pnt: Pos,
}

impl Field {
    fn new(width: usize, height: usize) -> Field {
        return Field {
            width,
            height,
            vals: vec![0; width * height],
            move_pnt: Pos(width as i32 / 2, height as i32 / 2),
        };
    }

    /// 将索引转为 `field` 中的位置
    fn index_to_pos(&self, index: usize) -> Pos {
        return Pos((index % self.width) as i32, (index / self.width) as i32);
    }

    /// 将 `field` 中的位置转为索引
    fn pos_to_index(&self, pos: Pos) -> usize {
        let w = self.width as i32;
        let h = self.height as i32;
        return ((pos.0 + pos.1 * w) % (w * h)) as usize;
    }

    /// 限制位置在 [`Field`] 范围内
    /// 若超出, 则取最大/小值
    fn limit(&self, pos: Pos) -> Pos {
        let w = self.width as i32;
        let h = self.height as i32;
        let mut pos = pos;

        if pos.0 < 0 {
            pos.0 = 0;
        } else if pos.0 >= w {
            pos.0 = w - 1;
        }

        if pos.1 < 0 {
            pos.1 = 0;
        } else if pos.1 >= h {
            pos.1 = h - 1;
        }

        return pos;
    }

    /// 根据传入的方向移动两次
    /// 仅 [`u8`] 的后 4 bit 为有效方向
    /// - `00` LU
    /// - `01` RU
    /// - `10` LD
    /// - `11` RD
    fn pnt_move(&mut self, dirs: u8) -> Pos {
        let mut dirs = dirs;

        for _ in 0..2 {
            self.move_pnt += match dirs & 0b11 {
                0b00 => Pos(-1, -1),
                0b01 => Pos(1, -1),
                0b10 => Pos(-1, 1),
                0b11 => Pos(1, 1),
                _ => Pos(0, 0),
            };
            self.move_pnt = self.limit(self.move_pnt);
            let index = self.pos_to_index(self.move_pnt);
            self.vals[index] += 1;
            dirs >>= 2;
        }

        return self.move_pnt;
    }

    /// 获取指定点的途径次数
    fn get_cnt(&self, pos: Pos) -> i32 {
        return self.vals[self.pos_to_index(pos)];
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chars_list = [
            ' ', '.', 'o', '+', '=', '*', 'B', 'O', 'X', '@', '%', '&', '#', '/', '^', 'S', 'E',
        ];

        write!(f, "+{}+\n", "-".repeat(self.width))?;
        for y in 0..self.height as i32 {
            write!(f, "|")?;
            for x in 0..self.width as i32 {
                write!(f, "{}", chars_list[self.get_cnt(Pos(x, y)) as usize])?;
            }
            write!(f, "|\n")?;
        }
        write!(f, "+{}+\n", "-".repeat(self.width))?;
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_pos_add() {
        let mut a = Pos(1, 2);
        let b = Pos(2, 3);
        assert_eq!(a + b, Pos(3, 5));
        a += b;
        assert_eq!(a, Pos(3, 5));
    }

    #[test]
    fn test_field_init() {
        let f = Field::new(17, 9);
        assert_eq!(f.move_pnt, Pos(8, 4));
    }

    #[test]
    fn test_field_index_and_pos() {
        let f = Field::new(17, 9);
        assert_eq!(f.pos_to_index(Pos(4, 7)), 123);
        assert_eq!(f.index_to_pos(123), Pos(4, 7));
    }

    #[test]
    fn test_field_move() {
        let mut f = Field::new(17, 9);
        f.move_pnt = Pos(2, 2);

        // LU, LD
        f.pnt_move(8);
        assert_eq!(f.move_pnt, Pos(0, 2));

        // LU, LU
        f.pnt_move(0);
        assert_eq!(f.move_pnt, Pos(0, 0));
    }
}
