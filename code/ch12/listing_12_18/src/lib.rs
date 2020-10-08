

pub fn search<'a> (query: str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        if line.contains(query) {
            
        }
    }
}