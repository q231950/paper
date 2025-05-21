use super::Loan;

#[derive(Debug, PartialEq, uniffi::Record)]
pub struct Loans {
    pub loans: Vec<Loan>,
}

impl IntoIterator for Loans {
    type Item = Loan;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.loans.into_iter()
    }
}

impl Loans {
    pub(crate) fn new() -> Self {
        Loans { loans: vec![] }
    }
}
