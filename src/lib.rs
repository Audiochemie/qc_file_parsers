pub mod array_text;
pub mod format_string;
pub mod xyz;

macro_rules! open_file {
    ($s:expr) => {{
        let file_handle = std::fs::File::open($s.to_string()).unwrap();
        std::io::BufReader::new(file_handle)
    }};
}
