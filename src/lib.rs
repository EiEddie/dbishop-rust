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
    #[allow(dead_code)]
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

/// 获取一个 [`u8`] 序列的指纹
pub fn fingerprint(seq: Vec<u8>) -> String {
    let mut field = Field::new(17, 9);
    let idx_begin = field.pos_to_index(field.move_pnt);

    for elem in seq {
        field.pnt_move(elem);
    }

    let idx_end = field.pos_to_index(field.move_pnt);
    field.vals[idx_begin] = 15;
    field.vals[idx_end] = 16;

    return field.to_string();
}

/// hex 字符串转换的错误类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FromHexError {
    /// 无效字符. 应为: `0...9`, `a...f` 或 `A...F`
    InvalidHexCharacter { c: char, index: usize },
}

#[cfg(feature = "std")]
impl std::error::Error for FromHexError {}

impl fmt::Display for FromHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FromHexError::InvalidHexCharacter { c, index } => {
                write!(f, "Invalid character {:?} at position {}", c, index)
            }
        }
    }
}

/// 十六进制字符转十进制
fn dec(chr: u8, index: usize) -> Result<u8, FromHexError> {
    return match chr {
        b'0'..=b'9' => Ok(chr - b'0'),
        b'a'..=b'f' => Ok(chr - b'a' + 10),
        b'A'..=b'F' => Ok(chr - b'A' + 10),
        _ => Err(FromHexError::InvalidHexCharacter {
            c: chr as char,
            index,
        }),
    };
}

/// 将十六进制字符串中的字符依次翻译为十进制
///
/// # Examples
///
/// ```
/// use dbishop::*;
/// let ans: Vec<u8> = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,10,11,12,13,14,15];
/// assert_eq!(
///     decode(String::from("0123456789abcdefABCDEF")),
///     Ok(ans)
/// );
/// assert_eq!(
///     decode(String::from("g")),
///     Err(FromHexError::InvalidHexCharacter{c: 'g', index: 0})
/// );
/// assert_eq!(
///     decode(String::from("G")),
///     Err(FromHexError::InvalidHexCharacter{c: 'G', index: 0})
/// );
/// ```
pub fn decode(hex: String) -> Result<Vec<u8>, FromHexError> {
    let mut ans: Vec<u8> = Vec::new();
    for (i, chr) in hex.as_bytes().iter().enumerate() {
        let dec = dec(*chr, i)?;
        ans.push(dec);
    }
    return Ok(ans);
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
        assert_eq!(f.get_cnt(Pos(1, 1)), 1);
        assert_eq!(f.get_cnt(Pos(0, 2)), 1);

        // LU, LU
        f.pnt_move(0);
        assert_eq!(f.move_pnt, Pos(0, 0));
        assert_eq!(f.get_cnt(Pos(0, 1)), 1);
        assert_eq!(f.get_cnt(Pos(0, 0)), 1);
    }
}
