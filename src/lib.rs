use std::process;

pub fn help(binary_name: &String){
    println!("Urlencoding with Rust.\n");
    println!("Usage: {} input\n", binary_name);
    println!("To URL encode or decoda a string\n");
    
    process::exit(0);
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_encoding() {
        let decoded = "http://www.example.net/index.html?session=A54C6FE2#info";
        let encoded = urlencoding::encode(decoded);
        assert_eq!(encoded, "http%3A%2F%2Fwww.example.net%2Findex.html%3Fsession%3DA54C6FE2%23info");
    }

    #[test]
    fn example_decoding() {
        let encoded = "http%3A%2F%2Fwww%2Eexample%2Enet%2Findex%2Ehtml%3Fsession%3DA54C6FE2%23info";
        let decoded = urlencoding::decode(encoded).unwrap();
        assert_eq!("http://www.example.net/index.html?session=A54C6FE2#info", decoded);
    }

    #[test]
    fn random_string(){
        let mut i = 0;
        while i < 100000 {
            let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890:/?#[]@!$%&'()*+,;=";
            let random_string = random_string::generate(100, charset);
            let encoded = urlencoding::encode(random_string.as_str());
            let decoded = urlencoding::decode(&encoded).unwrap();
            i = i + 1;

            assert_eq!(decoded, random_string);
        }
    }
}