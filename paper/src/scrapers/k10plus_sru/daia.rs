use serde::Deserialize;

use crate::model::{Availability, Location};

const DAIA_URL: &str = "https://daia.gbv.de";

/// The availability of the copies of a record from the GBV's DAIA service
///
/// https://daia.gbv.de/?id=opac-de-1:ppn:1886167923&format=json
#[derive(Debug, Deserialize)]
pub(crate) struct DaiaResponse {
    #[serde(default)]
    document: Vec<DaiaDocument>,
}

#[derive(Debug, Deserialize)]
struct DaiaDocument {
    #[serde(default)]
    item: Vec<DaiaItem>,
}

#[derive(Debug, Deserialize)]
struct DaiaItem {
    /// e.g. `http://uri.gbv.de/document/opac-de-1:epn:4578850269`
    id: Option<String>,
    #[serde(default)]
    available: Vec<DaiaService>,
    #[serde(default)]
    unavailable: Vec<DaiaService>,
}

#[derive(Debug, Deserialize)]
struct DaiaService {
    /// `presentation`, `loan`, `interloan` or `openaccess`
    service: String,
}

impl DaiaResponse {
    pub(crate) fn url(database: &str, ppn: &str) -> String {
        format!("{DAIA_URL}/?id={database}:ppn:{ppn}&format=json")
    }

    /// The availability of the copy with the given EPN at `location`
    ///
    /// Copies that can be borrowed or used in the library are available, copies whose loan is
    /// unavailable are not available, and copies without services (e.g. online resources) are
    /// unknown.
    pub(crate) fn availability(&self, epn: Option<&str>, location: Location) -> Availability {
        let item = epn.and_then(|epn| {
            self.document
                .iter()
                .flat_map(|document| document.item.iter())
                .find(|item| {
                    item.id
                        .as_deref()
                        .is_some_and(|id| id.ends_with(&format!(":epn:{epn}")))
                })
        });

        let Some(item) = item else {
            return Availability::Unknown(location);
        };

        let offers = |services: &[DaiaService], service: &str| {
            services.iter().any(|offered| offered.service == service)
        };

        if offers(&item.available, "loan") {
            Availability::Available(location)
        } else if offers(&item.unavailable, "loan") {
            Availability::NotAvailable(location)
        } else if !item.available.is_empty() {
            Availability::Available(location)
        } else {
            Availability::Unknown(location)
        }
    }
}
