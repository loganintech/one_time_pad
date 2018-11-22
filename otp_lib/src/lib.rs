pub fn encode(plaintext: &str, key: &str) -> String {
    assert!(plaintext.len() <= key.len());
    assert!(key
        .chars()
        .all(|chr| chr.is_alphabetic() || chr.is_whitespace()));

    plaintext
        .chars()
        .zip(key.chars())
        .map(|(chr, key_chr)| {
            let chr_u32 = match chr {
                character if character == ' ' => 26,
                character => character as u32 - 65,
            };
            let key_chr_u32 = match key_chr {
                ' ' => 26,
                key_chr => key_chr as u32 - 65,
            };

            match (((chr_u32 + key_chr_u32) % 27) + 65) as u8 {
                //Character index 26 is a space. Add 65 to get to the beginning of the character arrays
                val if val == 26 + 65 => ' ',
                val => val as char
            }
        })
        .collect::<String>()
}

pub fn decode(ciphertext: &str, key: &str) -> String {
    assert!(ciphertext.len() <= key.len());
    assert!(key
        .chars()
        .all(|chr| chr.is_alphabetic() || chr.is_whitespace()));

    ciphertext
        .chars()
        .zip(key.chars())
        .map(|(chr, key_chr)| {
            let chr_u32 = match chr {
                character if character == ' ' => 26,
                character => character as u32 - 65,
            };

            let key_chr_u32 = match key_chr {
                ' ' => 26,
                key_chr => key_chr as u32 - 65,
            };

            let res = if key_chr_u32 > chr_u32 {
                27 - (key_chr_u32 - chr_u32)
            } else {
                chr_u32 - key_chr_u32
            };

            match ((res % 27) + 65) as u8 {
                //Character index 26 is a space. Add 65 to get to the beginning of the character arrays
                val if val == 26 + 65 => ' ',
                val => val as char
            }
        })
        .collect::<String>()
}

mod test {

    use super::*;

    // ENCODE

    #[test]
    fn encode_one() {
        let encoded = encode("TEXT", "AAAAA");
        assert!(encoded == "TEXT");
    }

    #[test]
    fn encode_two() {
        let encoded = encode("TEXT", "BBBBB");
        assert!(encoded == "UFYU");
    }

    #[test]
    fn encode_three() {
        let encoded = encode("TEXT", "CCCCC");
        assert!(encoded == "VGZV");
    }

    #[test]
    fn encode_four() {
        let encoded = encode("ABCDEFGHIJKLMNOPQRSTUVWXYZ ", "AAAAAAAAAAAAAAAAAAAAAAAAAAA");
        assert!(encoded == "ABCDEFGHIJKLMNOPQRSTUVWXYZ ");
    }

    #[test]
    fn encode_five() {
        let encoded = encode("ABCDEFGHIJKLMNOPQRSTUVWXYZ ", "BBBBBBBBBBBBBBBBBBBBBBBBBBB");
        assert!(encoded == "BCDEFGHIJKLMNOPQRSTUVWXYZ A");
    }

    #[test]
    fn encode_six() {
        let encoded = encode("AAA", "AB ");
        assert!(encoded == "AB ");
    }

    #[test]
    fn encode_real() {
        let encoded = encode("A REAL EXAMPLE WITH SOME ACTUAL TEXT", "FUKBEAGXTNPT AGSDP JBDRQIPSONGBMAPJQDCNQBBDXQYQSAMOTAKZQCROOSGCIPIFWUNNXGXCAYSRK");
        assert!(encoded ==   "FTAFELFAPNAHKEFNLHGITRCUHPUGGGMLTTFI");
    }

    // DECODE

    #[test]
    fn decode_one() {
        let decoded = decode("TEXT", "AAAA");
        assert!(decoded == "TEXT");
    }

    #[test]
    fn decode_two() {
        let decoded = decode("UFYU", "BBBBB");
        assert!(decoded == "TEXT");
    }

    #[test]
    fn decode_three() {
        let decoded = decode("VGZV", "CCCCC");
        assert!(decoded == "TEXT");
    }

    #[test]
    fn decode_four() {
        let decoded = decode("ABCDEFGHIJKLMNOPQRSTUVWXYZ ", "AAAAAAAAAAAAAAAAAAAAAAAAAAA");
        assert!(decoded == "ABCDEFGHIJKLMNOPQRSTUVWXYZ ");
    }

    #[test]
    fn decode_five() {
        let decoded = decode("BCDEFGHIJKLMNOPQRSTUVWXYZ A", "BBBBBBBBBBBBBBBBBBBBBBBBBBB");
        assert!(decoded == "ABCDEFGHIJKLMNOPQRSTUVWXYZ ");
    }

    #[test]
    fn decode_six() {
        let decoded = decode("AB ", "AB ");
        assert!(decoded == "AAA");
    }

    #[test]
    fn decode_real() {
        let decoded = decode("FTAFELFAPNAHKEFNLHGITRCUHPUGGGMLTTFI", "FUKBEAGXTNPT AGSDP JBDRQIPSONGBMAPJQDCNQBBDXQYQSAMOTAKZQCROOSGCIPIFWUNNXGXCAYSRK");
        assert!(decoded == "A REAL EXAMPLE WITH SOME ACTUAL TEXT");
    }

}
