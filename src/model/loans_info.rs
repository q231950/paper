use serde::{Deserialize, Serialize};
use serde_json::Result;
use super::Loan;

use comfy_table::presets::UTF8_FULL;
use comfy_table::CellAlignment::Right;
use comfy_table::*;

#[derive(Serialize, Deserialize)]
pub struct LoansInfo {
    pub loans: Vec<Loan>,
}

impl LoansInfo {
    pub fn new() -> LoansInfo {
        LoansInfo {
            loans: Vec::new(),
        }
    }

    pub fn add_loan(&mut self, loan: Loan) {
        self.loans.push(loan);
    }

    pub fn to_json(&self) -> Result<String> {
        let loans_info = serde_json::to_string_pretty(&self);
        match loans_info {
            Ok(json) => Ok(format!("loansInfo: {}", json.as_str())),
            Err(err) => Err(err),
        }
    }


    pub fn as_table(&self) -> String {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Author", "Title", "Due Date", "Renewable", "Shelf Mark"]);

        let renewable_col = table.get_column_mut(3).expect("This is the renewable column");
        renewable_col.set_cell_alignment(Right);
         

        for loan in self.loans.iter() {
            table.add_row(vec![
                loan.author.to_owned(),
                loan.title.to_owned(),
                loan.date_due.to_owned(),
                (if loan.can_renew { "yes" } else { "no" }).to_string(),
                loan.item_number.to_owned(),
            ]);
        }
 
        format!("\n{}", table)
    }
}
