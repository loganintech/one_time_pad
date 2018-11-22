pub fn encode(plaintext: &str, key: &str) -> Result<String, String> {
    if plaintext.len() > key.len() {
        return Err("The key must be equal or longer than the plaintext.".into());
    }
    if !key
        .chars()
        .all(|chr| chr.is_alphabetic() || chr.is_whitespace())
    {
        return Err("The key source was malformed.".into());
    }

    Ok(plaintext
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
                val => val as char,
            }
        })
        .collect::<String>())
}

pub fn decode(ciphertext: &str, key: &str) -> Result<String, String> {
    if ciphertext.len() > key.len() {
        return Err("The key must be equal or longer than the ciphertext.".into());
    }
    if !key
        .chars()
        .all(|chr| chr.is_alphabetic() || chr.is_whitespace())
    {
        return Err("The key source was malformed.".into());
    }

    Ok(ciphertext
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
                val => val as char,
            }
        })
        .collect::<String>())
}

mod test {

    use super::*;

    // ENCODE

    #[test]
    fn encode_one() {
        let encoded = encode("TEXT", "AAAAA");
        assert!(encoded.unwrap() == "TEXT");
    }

    #[test]
    fn encode_two() {
        let encoded = encode("TEXT", "BBBBB");
        assert!(encoded.unwrap() == "UFYU");
    }

    #[test]
    fn encode_three() {
        let encoded = encode("TEXT", "CCCCC");
        assert!(encoded.unwrap() == "VGZV");
    }

    #[test]
    fn encode_four() {
        let encoded = encode("ABCDEFGHIJKLMNOPQRSTUVWXYZ ", "AAAAAAAAAAAAAAAAAAAAAAAAAAA");
        assert!(encoded.unwrap() == "ABCDEFGHIJKLMNOPQRSTUVWXYZ ");
    }

    #[test]
    fn encode_five() {
        let encoded = encode("ABCDEFGHIJKLMNOPQRSTUVWXYZ ", "BBBBBBBBBBBBBBBBBBBBBBBBBBB");
        assert!(encoded.unwrap() == "BCDEFGHIJKLMNOPQRSTUVWXYZ A");
    }

    #[test]
    fn encode_six() {
        let encoded = encode("AAA", "AB ");
        assert!(encoded.unwrap() == "AB ");
    }

    #[test]
    fn encode_real() {
        let encoded = encode(
            "A REAL EXAMPLE WITH SOME ACTUAL TEXT",
            "FUKBEAGXTNPT AGSDP JBDRQIPSONGBMAPJQDCNQBBDXQYQSAMOTAKZQCROOSGCIPIFWUNNXGXCAYSRK",
        );
        assert!(encoded.unwrap() == "FTAFELFAPNAHKEFNLHGITRCUHPUGGGMLTTFI");
    }

    // DECODE

    #[test]
    fn decode_one() {
        let decoded = decode("TEXT", "AAAA");
        assert!(decoded.unwrap() == "TEXT");
    }

    #[test]
    fn decode_two() {
        let decoded = decode("UFYU", "BBBBB");
        assert!(decoded.unwrap() == "TEXT");
    }

    #[test]
    fn decode_three() {
        let decoded = decode("VGZV", "CCCCC");
        assert!(decoded.unwrap() == "TEXT");
    }

    #[test]
    fn decode_four() {
        let decoded = decode("ABCDEFGHIJKLMNOPQRSTUVWXYZ ", "AAAAAAAAAAAAAAAAAAAAAAAAAAA");
        assert!(decoded.unwrap() == "ABCDEFGHIJKLMNOPQRSTUVWXYZ ");
    }

    #[test]
    fn decode_five() {
        let decoded = decode("BCDEFGHIJKLMNOPQRSTUVWXYZ A", "BBBBBBBBBBBBBBBBBBBBBBBBBBB");
        assert!(decoded.unwrap() == "ABCDEFGHIJKLMNOPQRSTUVWXYZ ");
    }

    #[test]
    fn decode_six() {
        let decoded = decode("AB ", "AB ");
        assert!(decoded.unwrap() == "AAA");
    }

    #[test]
    fn decode_real() {
        let decoded = decode(
            "FTAFELFAPNAHKEFNLHGITRCUHPUGGGMLTTFI",
            "FUKBEAGXTNPT AGSDP JBDRQIPSONGBMAPJQDCNQBBDXQYQSAMOTAKZQCROOSGCIPIFWUNNXGXCAYSRK",
        );
        assert!(decoded.unwrap() == "A REAL EXAMPLE WITH SOME ACTUAL TEXT");
    }

}
