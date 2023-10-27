use std::collections::LinkedList;
use std::error::Error;
use rust_xlsxwriter::*;
use crate::car::Car;
use crate::state::State;


pub struct Report {
    workbook: Workbook
}

impl Default for Report {
    fn default() -> Self {
        Report {
            workbook: Workbook::new()
        }
    }
}

impl Report {
    pub fn create(&mut self, states: &LinkedList<State>, car: &Car) -> Result<(),Box<dyn Error>> {

        // Create some formats to use in the worksheet.
        let bold_format = Format::new().set_bold();
        let decimal_format = Format::new().set_num_format("0.000");
        let date_format = Format::new().set_num_format("yyyy-mm-dd");
        let merge_format = Format::new()
            .set_border(FormatBorder::Thin)
            .set_align(FormatAlign::Center);

        // Add a worksheet to the workbook.
        let worksheet = self.workbook.add_worksheet();

        // Set the column width for clarity.
        worksheet.set_column_width(0, 22)?;
        worksheet.set_column_width(1, 22)?;
        worksheet.set_column_width(2, 22)?;

        worksheet.write_with_format( 0, 0,"Posição (x)", &bold_format )?;
        worksheet.write_with_format( 0, 1,"Velocidade (v)", &bold_format )?;
        worksheet.write_with_format( 0, 2,"Tempo (t)", &bold_format )?;
        worksheet.write_with_format( 0, 3,"Aceleração (a)", &bold_format )?;

        worksheet.write_with_format(0, 3, car.a, &decimal_format).unwrap();

        states.iter().enumerate().for_each(|(index, state)| {
            let row_num:RowNum = (index + 1) as RowNum;
            worksheet.write_with_format(row_num, 0, state.x, &decimal_format).unwrap();
            //println!("x: {}", state.x);
            worksheet.write_with_format(row_num, 1, state.v, &decimal_format).unwrap();
            //println!("v: {}", state.v);
            worksheet.write_with_format(row_num, 2, state.th, &decimal_format).unwrap();
            //println!("th: {}", state.th);
        });

        // Save the file to disk.
        self.workbook.save("relatorio.xlsx")?;

        Ok(())
    }
}