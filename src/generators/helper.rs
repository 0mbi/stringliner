use crate::generators::printables::printable_vect;
use crate::generators::config::Cfg;

pub fn get_chunk_lines(a: &Vec<u8>, config: &Cfg) -> Vec<String> {
    let vec_hex = a.
        into_iter()
        .map(|i| format!("{:#04X}", i))
        .collect::<Vec<String>>();
    let vh_chunks = vec_hex
        .chunks(config.chunk_length)
        .collect::<Vec<_>>();
    let chunk_lines = vh_chunks
        .into_iter()
        .map(|s| s.join(", "))
        .collect::<Vec<_>>();
    return chunk_lines;
}

pub fn chunk_string_with_printables(chunk_lines: &Vec<String>,
                                cmt_str: String,
                                a: &Vec<u8>, config: &Cfg) -> String {
    let pv = printable_vect(&a);
    let vec_printable = pv
        .chunks(config.chunk_length)
        .collect::<Vec<_>>();
    let mut x = String::new();
    let commented_chunk_lines = chunk_lines.clone()
        .into_iter()
        .enumerate()
        .map( |(idx, val)| {
            let cmt_string = vec![cmt_str.clone(), String::from_utf8_lossy(vec_printable[idx]).to_string()].join(" ");
            if idx != chunk_lines.len()-1 {
                x = val.clone();
                format!("{}, {:>width$}", val, cmt_string, width=2)
            } else {
                let w = x.len() - val.len() + cmt_string.len();
                format!("{}  {:>width$}", val, cmt_string, width=w as usize)
            }
        }).collect::<Vec<_>>();
    commented_chunk_lines.join("\n    ")
}