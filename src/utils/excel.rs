use rust_xlsxwriter::{worksheet::Worksheet, Color, Format, Workbook, XlsxError};

pub struct Excel {
    workbook: Workbook,
    pub path: String,
}

impl Excel {
    pub fn new(path: &str) -> Self {
        Excel {
            workbook: Workbook::new(),
            path: format!("{}.xlsx", path),
        }
    }

    fn store(worksheet: &mut Worksheet, row: u32, col: u16, data: &str) -> Result<(), XlsxError> {
        worksheet.write_string(row, col, data)?;

        Ok(())
    }

    fn store_heading(
        worksheet: &mut Worksheet,
        row: u32,
        col: u16,
        data: &str,
    ) -> Result<(), XlsxError> {
        worksheet.write_with_format(
            row,
            col,
            data,
            &Format::new().set_background_color(Color::RGB(0xDAA520)),
        )?;

        Ok(())
    }

    pub fn write_file(&mut self, data: &Vec<Vec<String>>) -> Result<(), XlsxError> {
        let mut worksheet = self.workbook.add_worksheet();

        for (row, value) in data.iter().enumerate() {
            if value[0] == "title" {
                for (col, value) in value.iter().enumerate() {
                    Excel::store_heading(&mut worksheet, row as u32, col as u16, value)?;
                }
            } else {
                for (col, value) in value.iter().enumerate() {
                    Excel::store(&mut worksheet, row as u32, col as u16, value)?;
                }
            }
        }

        self.workbook.save(&self.path)?;
        Ok(())
    }
}
