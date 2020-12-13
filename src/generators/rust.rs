use crate::generators::config::Cfg;
use crate::generators::helper::{get_chunk_lines, chunk_string_with_printables};

pub fn rust_string(a: &Vec<u8>, config: &Cfg) -> String {

    let chunk_lines = get_chunk_lines(a, &config);

    let chunk_string = match config.printables {
        true => chunk_string_with_printables(&chunk_lines, "//".to_string(), &a, &config),
        false => chunk_lines.join(",\n    ")
    };

    return format!("let {name}: Vec<u8> = vec![\n    {vec_string}\n];",
                   name = config.name, vec_string = chunk_string);
}
