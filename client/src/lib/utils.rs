use rand;

pub fn random_char() -> char {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890".chars().collect();

    let char_index = (rand::random::<f32>() * chars.len() as f32).floor() as usize;

    return chars[char_index];
}

pub fn random_str(n: usize) -> String {
    return (0..n).into_iter().map(|_| random_char()).collect()
}

pub fn random_key() -> String {
    return random_str(32);
}

pub fn random_nonce() -> String {
    return random_str(16)
}