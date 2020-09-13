struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let noval = String::from("call me Ishmael. Some");
    let first_sentence = noval.split(".").next().expect("Could not find a'.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
