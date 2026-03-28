use anyhow::Result;
use calamine::{open_workbook, Reader, Xlsx};

fn main() -> Result<()> {
    let path = "GBG counties one sheet Duncan 2025.xlsx";
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    println!("Sheet names: {:?}", workbook.sheet_names());
    let sheet_name = workbook.sheet_names()[0].clone();
    let range = workbook.worksheet_range(&sheet_name)?;
    if let Some(row) = range.rows().nth(4) {
        println!("Header Row (4):");
        for (col_idx, cell) in row.iter().enumerate() {
            println!("  {}: {}", col_idx, cell);
        }
    }
    if let Some(row) = range.rows().nth(5) {
        println!("Data Row (5):");
        for (col_idx, cell) in row.iter().enumerate() {
            println!("  {}: {}", col_idx, cell);
        }
    }
    Ok(())
}
