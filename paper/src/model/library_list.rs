use super::{Library, API};

#[derive(Debug, uniffi::Object)]
pub struct LibraryList {
    pub libraries: Vec<Library>,
}

#[uniffi::export]
impl LibraryList {
    #[uniffi::constructor]
    pub fn new() -> LibraryList {
        LibraryList {
            libraries: vec![
                Library {
                    identifier: "BERLIN".to_string(),
                    name: "Berlin".to_string(),
                    subtitle: "Staatsbibliothek Preußischer Kulturbesitz".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://lbssbb.gbv.de".to_string(),
                        catalog_url: "https://lbssbb.gbv.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    // the K10plus SRU interface serves the catalog of the Staatsbibliothek, its
                    // StabiKat (https://stabikat.de) puts its VuFind behind a human verification
                    search_api: Some(API::K10plusSru {
                        database: "opac-de-1".to_string(),
                        record_page_url: Some("https://stabikat.de/Record/{ppn}".to_string()),
                    }),
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "MAGDEBURG".to_string(),
                    name: "Magdeburg".to_string(),
                    subtitle: "Universitätsbibliothek".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://lbssbb.gbv.de".to_string(),
                        catalog_url: "https://opac.lbs-magdeburg.gbv.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "LUENEBURG".to_string(),
                    name: "Lüneburg".to_string(),
                    subtitle: "Leuphana Universitätsbibliothek".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://katalog.leuphana.gbv.de".to_string(),
                        catalog_url: "https://katalog.leuphana.gbv.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "BRAUNSCHWEIG".to_string(),
                    name: "Braunschweig".to_string(),
                    subtitle: "Universitätsbibliothek".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://lbsbrs.gbv.de".to_string(),
                        catalog_url: "https://lbsbrs.gbv.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "WOLFENBUETTEL".to_string(),
                    name: "Herzog August Bibliothek Wolfenbüttel".to_string(),
                    subtitle: "Forschungs- und Studienstätte für europäische Kulturgeschichte"
                        .to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://lbsbrs.gbv.de".to_string(),
                        catalog_url: "https://lbsbrs.gbv.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "SHLB".to_string(),
                    name: "Kiel".to_string(),
                    subtitle: "Schleswig-Holsteinische Landesbibliothek".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://katalog.ub.uni-kiel.de".to_string(),
                        catalog_url: "https://katalog.ub.uni-kiel.de/DB=3".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "MUTHESIUS".to_string(),
                    name: "Kiel".to_string(),
                    subtitle: "Muthesius Kunsthochschule".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://katalog.ub.uni-kiel.de".to_string(),
                        catalog_url: "https://katalog.ub.uni-kiel.de/DB=6".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "KIEL".to_string(),
                    name: "Kiel".to_string(),
                    subtitle: "Christian-Albrechts-Universität zu Kiel".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://katalog.ub.uni-kiel.de".to_string(),
                        catalog_url: "https://katalog.ub.uni-kiel.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "ILFH".to_string(),
                    name: "Schmalkalden".to_string(),
                    subtitle: "Cellarius-Hochschulbibliothek".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://opac.lbs-ilmenau.gbv.de".to_string(),
                        catalog_url: "https://opac.lbs-ilmenau.gbv.de/DB=2".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "ILMENAU".to_string(),
                    name: "Ilmenau".to_string(),
                    subtitle: "Universitätsbibliothek".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://opac.lbs-ilmenau.gbv.de".to_string(),
                        catalog_url: "https://opac.lbs-ilmenau.gbv.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "HKSF".to_string(),
                    name: "Hannover".to_string(),
                    subtitle: "Kurt-Schwitters-Forum".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://lbshan.gbv.de".to_string(),
                        catalog_url: "https://lbshan.gbv.de/DB=11".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "HILDESHEIM".to_string(),
                    name: "Hildesheim".to_string(),
                    subtitle: "Universitätsbibliothek".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://opac.lbs-hildesheim.gbv.de".to_string(),
                        catalog_url: "https://opac.lbs-hildesheim.gbv.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "HFHH".to_string(),
                    name: "Hannover".to_string(),
                    subtitle: "Bibliothek der Hochschule Hannover".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://opac.tib.eu".to_string(),
                        catalog_url: "https://opac.tib.eu/DB=4".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "LUH".to_string(),
                    name: "Hannover".to_string(),
                    subtitle: "Leibniz Universität Hannover".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://opac.tib.eu".to_string(),
                        catalog_url: "https://opac.tib.eu/DB=12".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: false,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "HELMUTSCHMIDT".to_string(),
                    name: "Helmut-Schmidt-Universität".to_string(),
                    subtitle: "Universitätsbibliothek".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://lbshsu.gbv.de".to_string(),
                        catalog_url: "https://lbshsu.gbv.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "HARBURG".to_string(),
                    name: "Hamburg-Harburg".to_string(),
                    subtitle: "Universitätsbibliothek der TUHH".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://katalog.b.tuhh.de".to_string(),
                        catalog_url: "https://katalog.b.tuhh.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "HAFENCITYUNI".to_string(),
                    name: "Hamburg".to_string(),
                    subtitle: "HafenCity Universität".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://katalog.b.tuhh.de".to_string(),
                        catalog_url: "https://katalog.b.tuhh.de/DB=22".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "HANNOVER".to_string(),
                    name: "Hannover".to_string(),
                    subtitle: "Technische Informationsbibliothek".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://lbshan.gbv.de".to_string(),
                        catalog_url: "https://lbshan.gbv.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "HAMBURG".to_string(),
                    name: "Hamburg".to_string(),
                    subtitle: "Staats und Universitätsbibliothek Hamburg".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://kataloge.hh.gbv.de".to_string(),
                        catalog_url: "https://kataloge.hh.gbv.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "HAMBURGPUBLIC".to_string(),
                    name: "Hamburg".to_string(),
                    subtitle: "Stiftung Hamburger Öffentliche Bücherhallen".to_string(),
                    api: API::HamburgPublic,
                    search_api: None,
                    enabled_for_search: false,
                    enabled_for_login: false,
                },
                Library {
                    identifier: "ROSTOCK".to_string(),
                    name: "Rostock".to_string(),
                    subtitle: "Universitätsbibliothek".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://opac.lbs-rostock.gbv.de".to_string(),
                        catalog_url: "https://opac.lbs-rostock.gbv.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "GREIFSWALD".to_string(),
                    name: "Greifswald".to_string(),
                    subtitle: "Universitätsbibliothek".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://lhgrw.gbv.de".to_string(),
                        catalog_url: "https://lhgrw.gbv.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: false,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "GOETTINGEN".to_string(),
                    name: "Göttingen".to_string(),
                    subtitle: "Staats und Universitätsbibliothek".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://opac.sub.uni-goettingen.de".to_string(),
                        catalog_url: "https://opac.sub.uni-goettingen.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "WISMAR".to_string(),
                    name: "Wismar".to_string(),
                    subtitle: "Hochschulbibliothek".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://lbswis.gbv.de".to_string(),
                        catalog_url: "https://lbswis.gbv.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "ERFURT".to_string(),
                    name: "Erfurt/Gotha".to_string(),
                    subtitle: "Universitäts- und Forschungsbibliothek".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://opac.uni-erfurt.de".to_string(),
                        catalog_url: "https://opac.uni-erfurt.de/DB=1".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "HAW".to_string(),
                    name: "Hamburg".to_string(),
                    subtitle: "Hochschule für Angewandte Wissenschaften".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://kataloge.hh.gbv.de".to_string(),
                        catalog_url: "https://kataloge.hh.gbv.de/DB=2".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "ANHALT".to_string(),
                    name: "Anhalt".to_string(),
                    subtitle: "Hochschule Anhalt".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://opac.lbs-anhalt.gbv.de".to_string(),
                        catalog_url: "https://opac.lbs-anhalt.gbv.de".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: true,
                    enabled_for_login: true,
                },
                Library {
                    identifier: "COMMERZ_HH".to_string(),
                    name: "Hamburg".to_string(),
                    subtitle: "Commerzbibliothek".to_string(),
                    api: API::Opc4v2_13Vzg6 {
                        base_url: "https://lbsvz2.gbv.de".to_string(),
                        catalog_url: "https://lbsvz2.gbv.de/DB=67".to_string(),
                        user_query_key: "1000".to_string(),
                    },
                    search_api: None,
                    enabled_for_search: false,
                    enabled_for_login: true,
                },
            ],
        }
    }

    pub fn library_for_identifier(&self, identifier: &str) -> Option<Library> {
        self.libraries
            .iter()
            .find(|library| library.identifier == identifier)
            .cloned()
    }

    pub fn libraries(&self) -> Vec<Library> {
        self.libraries.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::LibraryList;
    use crate::model::API;

    #[test]
    fn berlin_searches_k10plus_and_signs_in_to_its_opac() {
        let berlin = LibraryList::new().library_for_identifier("BERLIN").unwrap();

        assert!(matches!(
            berlin.api,
            API::Opc4v2_13Vzg6 { ref base_url, .. } if base_url == "https://lbssbb.gbv.de"
        ));
        assert_eq!(
            berlin.search_api(),
            API::K10plusSru {
                database: "opac-de-1".to_string(),
                record_page_url: Some("https://stabikat.de/Record/{ppn}".to_string()),
            }
        );
        assert!(berlin.enabled_for_search);
        assert!(berlin.enabled_for_login);
    }

    #[test]
    fn stabikat_is_not_a_separate_library() {
        assert_eq!(LibraryList::new().library_for_identifier("STABIKAT"), None);
    }

    #[test]
    fn libraries_without_search_api_search_with_their_api() {
        let hamburg = LibraryList::new()
            .library_for_identifier("HAMBURG")
            .unwrap();

        assert_eq!(hamburg.search_api, None);
        assert_eq!(hamburg.search_api(), hamburg.api);
    }

    #[test]
    fn library_identifiers_are_unique() {
        let libraries = LibraryList::new().libraries();
        let mut identifiers: Vec<_> = libraries.iter().map(|l| l.identifier.clone()).collect();
        identifiers.sort();
        identifiers.dedup();

        assert_eq!(identifiers.len(), libraries.len());
    }
}
