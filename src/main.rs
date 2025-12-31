use calamine::{open_workbook, Reader, Error, Xlsx};

fn main() -> Result<(), Error>{
    let path = "./config/thrust_maps/T200_Full.xlsx";

    //open
    let workbook: Xlsx<_> = open_workbook(path)?;

    //load all the sheets
    let sheets = workbook.sheet_names().to_owned();
    println!("Sheets: {:?}", sheets);

    

    return Ok(());
}