use crate::*;


simple_day!(|input|{
    let bytes = input.as_bytes();
    let mut in_garbage = false;
    let mut i = 0;
    let mut skipped = 0;
    let mut sum = 0;
    let mut level = 0;
    while i < input.len() {
        if !in_garbage {
            match bytes[i] {
                b'{' => {
                    level += 1;
                    sum += level;
                },
                b'}' => level -= 1,
                b'<' => in_garbage = true,
                b',' | b'\n' => (),
                _ => panic!("Unexpected char {}", bytes[i] as char)
            }
        } else {
            match bytes[i] {
                b'!' => i += 1,
                b'>' => in_garbage = false,
                _ => skipped += 1
            }
        }

        i += 1
    }

    format!("Level sum: {sum}, skipped: {skipped}")
});