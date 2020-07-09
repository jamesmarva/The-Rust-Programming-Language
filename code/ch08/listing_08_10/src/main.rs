fn main() {
    println!("Hello, world!");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("ffffffffffffffff")),
        SpreadsheetCell::Float(2.22),
    ];
    for i in &row {
        match i {
            SpreadsheetCell::Int(int32) => {
                println!("Int : {}", int32);
            },
            SpreadsheetCell::Float(float64) => {  
                println!("Float : {}", float64);
            },
            SpreadsheetCell::Text(text) => {
                println!("Text : {}", text);
            }   
        }
    }
}
enum SpreadsheetCell  { 
    Int(i32),
    Float(f64),
    Text(String),
}