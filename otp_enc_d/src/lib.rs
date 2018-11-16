pub fn pad_string(plaintext: &str, pad: &str) -> String {
    assert!(plaintext.len() <= pad.len());
    assert!(pad
        .chars()
        .all(|chr| chr.is_alphabetic() || chr.is_whitespace()));

    plaintext
        .chars()
        .zip(pad.chars())
        .map(|(chr, pad)| {
            let chr_u32 = match chr as u32 {
                character if character < 65 => 26,
                character => character - 65,
            };
            let pad_u32 = pad as u32 - 65;

            match (((chr_u32 + pad_u32) % 27) + 65) as u8 {
                //Character index 26 is a space. Add 65 to get to the beginning of the character arrays
                val if val == 26 + 65 => ' ',
                val => val as char
            }
        })
        .collect::<String>()
}

mod test {

    use super::pad_string;

    #[test]
    fn pad_one() {
        let encoded = pad_string("TEXT", "AAAAA");
        assert!(encoded == "TEXT");
    }

    #[test]
    fn pad_two() {
        let encoded = pad_string("TEXT", "BBBBB");
        assert!(encoded == "UFYU");
    }

    #[test]
    fn pad_three() {
        let encoded = pad_string("TEXT", "CCCCC");
        assert!(encoded == "VGZV");
    }

    #[test]
    fn pad_four() {
        let encoded = pad_string("ABCDEFGHIJKLMNOPQRSTUVWXYZ ", "AAAAAAAAAAAAAAAAAAAAAAAAAAA");
        assert!(encoded == "ABCDEFGHIJKLMNOPQRSTUVWXYZ ");
    }

    #[test]
    fn pad_five() {
        let encoded = pad_string("ABCDEFGHIJKLMNOPQRSTUVWXYZ ", "BBBBBBBBBBBBBBBBBBBBBBBBBBB");
        assert!(encoded == "BCDEFGHIJKLMNOPQRSTUVWXYZ A");
    }
}
