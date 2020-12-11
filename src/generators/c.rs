use crate::generators::config::Cfg;

pub fn c_string(a: &Vec<u8>, config: &Cfg) -> String {
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
    let chunk_string = chunk_lines.join(",\n    ");

    return format!("char {name}[] = {{\n    {vec_string}}};",
                   name = config.name, vec_string = chunk_string);
}
