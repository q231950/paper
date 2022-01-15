use serde::{Deserialize, Serialize};
use serde_json::Result;
use super::Loan;

use comfy_table::presets::UTF8_FULL;
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
        println!("Adding loan: {:?}", loan.title);
        self.loans.push(loan);
    }

    pub fn to_json(&self) -> Result<String> {
        let loansInfo = serde_json::to_string_pretty(&self);
        match loansInfo {
            Ok(json) => Ok(format!("loansInfo: {}", json.as_str())),
            Err(err) => Err(err),
        }
    }


    pub fn as_table(&self) -> String {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(80);

        for loan in self.loans.iter() {
            table.add_row(vec![
                Cell::new("Title").add_attribute(Attribute::Bold),
                Cell::new(loan.title.to_owned()),
            ]);
        }
        
        format!("\n{}", table)
    }
}
