//! This module prints letters of latin alphabet on 2 lines by even and odd positions in the
//! alphabet.
//!
//! # Examples
//!
//! ```
//! println!("Original alphabet letters: {:?}", ascii_alphabet());
//! println!("Alphabet letters split by odd/even positions:");
//! odd_even_letters();
//! ```

/// Returns latin alphabet
pub fn ascii_alphabet() -> Vec<char> {
    let mut alphabet = Vec::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        alphabet.push(a as char);
        a += 1;
    }
    alphabet
}

/// Prints odd and even letters on separate lines.
pub fn odd_even_letters() {
    let alphabet = ascii_alphabet();
    let mut odd = Vec::new();
    let mut even = Vec::new();
    for (pos, letter) in alphabet.iter().enumerate() {
        let is_even = |pos| pos % 2 == 0;
        if is_even(pos) {
            odd.push(letter);
        } else {
            even.push(letter);
        }
    }
    println!("{:?}", even); // ['b', 'd', 'f', 'h', 'j', 'l', 'n', 'p', 'r', 't', 'v', 'x', 'z']
    println!("{:?}", odd); // ['a', 'c', 'e', 'g', 'i', 'k', 'm', 'o', 'q', 's', 'u', 'w', 'y']
}
