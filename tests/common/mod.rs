use rand::{thread_rng, Rng};
use std::path::{Path, PathBuf};
use std::{env, fs, result};

#[cfg(test)]

pub fn generate_random_string() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const RSTRING_LEN: usize = 8;
    let mut rng = rand::thread_rng();

    let rstring: String = (0..RSTRING_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    return rstring;
}

pub fn make_tmp_dir() -> PathBuf {
    let mut tmp_dir = env::temp_dir();

    tmp_dir.push(Path::new(&generate_random_string()));

    fs::create_dir(&tmp_dir).unwrap();

    return tmp_dir.clone();
}
