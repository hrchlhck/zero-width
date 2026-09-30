#![allow(unused)]

const ZW_ZERO: char = '\u{200C}';
const ZW_ONE: char = '\u{200B}';

pub fn char_to_bin(c: char) -> String {
    let mut byte: u8 = c as u8;
    let mut ret: String = String::new();

    for _ in 0..8 {
        let bit: u8 = byte & 1;
        byte  >>= 1;
        
        ret.push((48 + bit) as char);
    }
    
    ret.chars().rev().collect()
}

pub fn string_to_bin(s: &String) -> String {
    let mut ret: String = String::new();

    for c in s.chars() {
        ret.push_str(&char_to_bin(c));
    }
    ret
}

pub fn bin_to_string(bin: String) -> String {
    let mut ret: String = String::new();

    let step = 8;
    for i in (0..bin.len()).step_by(step) {
        let s = &bin[i..i+step];

        ret.push(u8::from_str_radix(s, 2).unwrap() as char);
    }

    ret
}

pub fn hide(cover: String, message: String) -> String {
    let mut ret: String = String::new();

    let message = string_to_bin(&message);

    let mut cover = cover.chars().map(Some).chain(std::iter::repeat(None));
    let mut message = message.chars().map(Some).chain(std::iter::repeat(None));
    let zipped: Vec<(char, char)> = std::iter::from_fn(|| {
        match (cover.next().flatten(), message.next().flatten()) {
            (Some(l), Some(r)) => Some((l, r)),
            (Some(l), None)    => Some((l, '#')),
            (None, Some(r))    => Some(('#', r)),
            (None, None)       => None,
        }
    }).collect();

    for (c, m) in zipped {
        if c != '#' {
            ret.push(c);
        }
        if m == '0' {
            ret.push(ZW_ZERO);
        } else {
            ret.push(ZW_ONE);
        }
    }

    ret
}

pub fn reveal(payload: String) -> String {
    let mut ret: String = String::new();

    for c in payload.chars() {
        if c != ZW_ONE && c != ZW_ZERO {
            continue;
        }

        if c == ZW_ONE {
            ret.push('1');
        } else {
            ret.push('0');
        }
    }

    bin_to_string(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_char_to_bin() {
        let desired = "01101111";
        let got = char_to_bin('o');

        assert_eq!(desired, got);
    }

    #[test]
    fn test_string_to_bin() {
        let desired = String::from("011011110110110001100001");
        let got = string_to_bin(&String::from("ola"));

        assert_eq!(desired, got);
    }

    #[test]
    fn test_hide_matching_sizes() {
        let desired = String::from("b‌a​b​a‌b​a​b​a​");
        let got = hide(String::from("babababa"), String::from("o"));

        assert_eq!(desired, got);
    }

    #[test]
    fn test_hide_dismatching_sizes() {
        let desired = String::from("b‌a​b​a‌b​a​b​a​‌​​‌​​‌‌‌​​‌‌‌‌​");
        let got = hide(String::from("babababa"), String::from("ola"));

        assert_eq!(desired, got);
    }

    #[test]
    fn test_bin_to_string() {
        let desired = String::from("ola");
        let got = bin_to_string(String::from("011011110110110001100001"));

        assert_eq!(desired, got);
    }

    #[test]
    fn test_reveal_dismatching_sizes() {
        let desired = String::from("ola");
        let got = reveal(String::from("b‌a​b​a‌b​a​b​a​‌​​‌​​‌‌‌​​‌‌‌‌​"));

        assert_eq!(desired, got);
    }

    #[test]
    fn test_reveal_matching_sizes() {
        let desired = String::from("o");
        let got = reveal(String::from("b‌a​b​a‌b​a​b​a​"));

        assert_eq!(desired, got);
    }
}