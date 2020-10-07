struct ReplacePair<'a> {
    base: &'a str,
    diacritics: &'a str,
}
fn main() {
    let replace_pairs = create_pairs();
    let word = "TÅR";
    for i in word.chars(){
        println!("{}",i);
    }
    for a in replace_pairs {
        println!("{},  {}", a.base, a.diacritics);
    }
}

fn create_pairs<'a>() -> Vec<ReplacePair<'a>> {
    let mut vector = Vec::new();
    vector.push(ReplacePair {
        base: "A",
        diacritics: "Å",
    });
    vector
}
fn chars(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}