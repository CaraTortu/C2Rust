mod lib;

use self::lib::crypt;

fn main() {
    let msg = "Hello there!".to_owned();
    let key = "KnGDmdXEaZDwBnhPYxBUytcjrgDfbvat".to_owned();
    let nonce = "ABCDEFGHIJKL".to_owned();
    let enc = crypt::encrypt(&msg, &key, &nonce);
    println!("{}", &enc);

    let dec = crypt::decrypt(&enc, &key, &nonce);
    println!("{}", dec);
}
