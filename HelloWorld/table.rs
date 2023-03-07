use prettytable::{Table, Row, Cell};

fn main() {
    let vec: Vec<String> = vec!["AIE".to_string(), "CSE".to_string(), "ECE".to_string(), "EEE".to_string(), "ELC".to_string()];

    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Index"), Cell::new("Item")]));
    for (index, item) in vec.iter().enumerate() {
        table.add_row(Row::new(vec![Cell::new(&index.to_string()), Cell::new(item)]));
    }
    table.printstd();
}

/* 
in cargo.toml
[dependencies]
prettytable = "0.8.0"
*/
