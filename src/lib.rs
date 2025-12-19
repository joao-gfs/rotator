use std::fmt::Debug;

pub struct Ciphertext {
    pub original_text: String,
    pub current_text: String,
    pub rot: u8,
}

impl Ciphertext {
    pub fn new(input: String) -> Self {
        let text = String::from(input.trim());

        Ciphertext {
            original_text: text.clone(),
            current_text: text,
            rot: 0,
        }
    }

    pub fn rotate(&mut self, rot_num: u8) {
        let mut new_text = String::new();
        
        for c in self.current_text.as_bytes() {
            let new_char = rotate_char(*c, rot_num);
            new_text.push(new_char);
        }

        self.current_text = new_text;
        self.rot = (self.rot + rot_num) % 26;
    }

    pub fn push(&mut self, c: char) {
        self.original_text.push(c);
        let rotated_char = rotate_char(c as u8, self.rot);
        self.current_text.push(rotated_char);
    }

    pub fn pop(&mut self) {
        self.original_text.pop();
        self.current_text.pop();
    }
}

fn rotate_char(c: u8, rot_num: u8) -> char {
    let offset: u8;
    let char_num: u8;
    let new_char: u8;
    match c {
                65..91 => {
                    offset = 65;
                    char_num = c - 65;
                    new_char = caculate_char_rotation(char_num, rot_num, offset);
                }
                97..123 => {
                    offset = 97;
                    char_num = c - 97;
                    new_char = caculate_char_rotation(char_num, rot_num, offset);
                }
                _ => {
                    new_char = c;
                }
            }
    new_char as char
}

fn caculate_char_rotation(char_num: u8, rot_num: u8, offset: u8) -> u8 {
    let new_char = ((char_num + rot_num) % 26) + offset;
    new_char
}

impl Debug for Ciphertext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ciphertext: original_text: {}, current_text: {}, rot: {}", self.original_text, self.current_text, self.rot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_rot() {
        let offset = 97;
        let res = caculate_char_rotation(b'a' - offset, 13, offset);
        assert_eq!(res, b'n');
    }

    #[test]
    fn calc_rot_overflow() {
        let offset = 97;
        let res = caculate_char_rotation(b'v' - offset, 13, offset);
        assert_eq!(res, b'i');
    }
}
