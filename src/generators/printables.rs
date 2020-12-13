pub fn printable_string(bstring: &Vec<u8>) -> String {
    String::from_utf8(printable_vect(&bstring)).unwrap()
}

pub fn printable_vect(bstring: &Vec<u8>) -> Vec<u8> {
    return bstring
        .into_iter()
        .map(|x| printable_char(x))
        .collect();
}

pub fn printable_char(c: &u8) -> u8 {
    if c.is_ascii() && !c.is_ascii_control() {
        return c.clone() as u8
    } else {
        return '.' as u8
    }
}
