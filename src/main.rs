fn main() {
    let string: &str = "TÅRÖÄÆØ";
    println!("{}", string);
    let new_string = remove_diacritics(&string);
    println!("{}", new_string);
}

fn remove_diacritics(string: &str) -> String {
    let v = string.chars();
    let mut split_string: Vec<String> = Vec::new();
    for c in v {
        split_string.push(String::from(c));
    }
    let new_string: String = split_string
        .iter()
        .map(|j| match j.as_ref() {
            "À" | "Á" | "Â" | "Ã" | "Ä" | "Å" | "Æ" => "A",
            "Þ" => "B",
            "Ç" | "Č" => "C",
            "Ď" | "Ð"  => "D",
            "Ě" | "È" | "É" | "Ê" | "Ë" => "E",
            "Ƒ" => "F",
            "Ì" | "Í" | "Î" | "Ï" => "I",
            "Ň" | "Ñ" => "N",
            "Ò" | "Ó" | "Ô" | "Õ" | "Ö" | "Ø" => "O",
            "Ř" =>"R",
            "ß" => "ss",
            "Š" => "S",
            "Ť" => "T",
            "Ů" | "Ù" | "Ú" | "Û" | "Ü" => "U",
            "Ý" => "Y",
            "Ž" => "Z",

            "à" | "á" | "â" | "ã" | "ä" | "å" | "æ" => "a",
            "þ"=> "b",
            "ç" | "č" => "c",
            "ď" | "ð"  => "d",
            "ě" | "è" | "é" | "ê" | "ë" => "e",
            "ƒ" => "f",
            "ì" | "í" | "î" | "ï" => "i",
            "ñ" | "ň" => "n",
            "ò" | "ó" | "ô" | "õ" | "ö" | "ø" => "o",
            "ř" => "r",
            "š" => "s",
            "ť" => "t",
            "ů" | "ù" | "ú" | "û" | "ü" => "u",
            "ý" | "ÿ" => "y",
            "ž" => "z",

            _ => j,
        })
        .collect();
    new_string
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_uppercase(){
        assert_eq!(remove_diacritics("TÅRÖÄÆØ"), String::from("TAROAAO"))
    }
    #[test]
    fn test_lowercase(){
        assert_eq!(remove_diacritics("čďêƒíó"), String::from("cdefio"))
    }
}