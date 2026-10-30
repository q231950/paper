use roxmltree::{Document, Node};

const MARC_NAMESPACE: &str = "http://www.loc.gov/MARC21/slim";

/// A MARC 21 record of a K10plus SRU response
///
/// https://wiki.k10plus.de/display/K10PLUS/MARC21-Exportformat
pub(crate) struct MarcRecord<'a, 'input> {
    node: Node<'a, 'input>,
}

/// A holding of the record's library, MARC field 924
pub(crate) struct Holding {
    /// The id of the copy (EPN), which DAIA uses as well
    pub(crate) epn: Option<String>,
    pub(crate) call_number: Option<String>,
    pub(crate) url: Option<String>,
}

impl Holding {
    /// The call number of a printed copy or "Online" for an electronic copy
    pub(crate) fn name(&self) -> String {
        match (&self.call_number, &self.url) {
            (Some(call_number), _) => call_number.clone(),
            (None, Some(_)) => "Online".to_string(),
            (None, None) => "Bestand".to_string(),
        }
    }
}

impl<'a, 'input> MarcRecord<'a, 'input> {
    /// All MARC records of an SRU response
    pub(crate) fn records_in(document: &'a Document<'input>) -> Vec<MarcRecord<'a, 'input>> {
        document
            .descendants()
            .filter(|node| node.has_tag_name((MARC_NAMESPACE, "record")))
            .map(|node| MarcRecord { node })
            .collect()
    }

    /// The K10plus record id (PPN)
    pub(crate) fn ppn(&self) -> Option<String> {
        self.node
            .children()
            .find(|node| {
                node.has_tag_name((MARC_NAMESPACE, "controlfield"))
                    && node.attribute("tag") == Some("001")
            })
            .and_then(|node| node.text())
            .map(|text| text.trim().to_string())
    }

    /// Title and subtitle, e.g. "Human-AI collaboration in research : practical applications"
    pub(crate) fn title(&self) -> Option<String> {
        let title = self.subfield("245", 'a')?;

        Some(match self.subfield("245", 'b') {
            Some(subtitle) => format!("{title} : {subtitle}"),
            None => title,
        })
    }

    /// Title with the statement of responsibility, e.g. "Trauriger Tiger / Neige Sinno"
    pub(crate) fn full_title(&self) -> Option<String> {
        let title = self.title()?;

        Some(match self.subfield("245", 'c') {
            Some(responsibility) => format!("{title} / {responsibility}"),
            None => title,
        })
    }

    /// The main author or, without one, the first further person or corporate body
    pub(crate) fn author(&self) -> Option<String> {
        ["100", "110", "111", "700", "710"]
            .iter()
            .find_map(|tag| self.subfield(tag, 'a'))
    }

    /// Place, publisher and date, e.g. "München : dtv, 2024"
    pub(crate) fn publication(&self) -> Option<String> {
        // the first 264 with the second indicator 1 is the publication, a second one the copyright
        let field = self
            .data_fields("264")
            .find(|field| field.attribute("ind2") == Some("1"))
            .or_else(|| self.data_fields("264").next())
            .or_else(|| self.data_fields("260").next())?;

        let places = Self::subfields_of(field, 'a').join(" ; ");
        let publisher = Self::subfields_of(field, 'b').join(" : ");
        let date = Self::subfields_of(field, 'c').join(" ");

        let place_and_publisher = vec![places, publisher]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" : ");

        let publication = vec![place_and_publisher, date]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(", ");

        (!publication.is_empty()).then_some(publication)
    }

    /// The URL of a cover image, which some records link in 856 with `$3 Cover`
    pub(crate) fn cover_url(&self) -> Option<String> {
        self.data_fields("856")
            .find(|field| {
                Self::subfields_of(*field, '3')
                    .iter()
                    .any(|label| label == "Cover")
            })
            .and_then(|field| Self::subfields_of(field, 'u').into_iter().next())
    }

    pub(crate) fn holdings(&self) -> Vec<Holding> {
        self.data_fields("924")
            .map(|field| Holding {
                epn: Self::subfields_of(field, 'a').into_iter().next(),
                call_number: Self::subfields_of(field, 'g').into_iter().next(),
                url: Self::subfields_of(field, 'k').into_iter().next(),
            })
            .collect()
    }

    /// The first subfield `code` of the first field `tag`
    pub(crate) fn subfield(&self, tag: &str, code: char) -> Option<String> {
        self.data_fields(tag)
            .find_map(|field| Self::subfields_of(field, code).into_iter().next())
    }

    /// The subfields `code` of all fields `tag`
    pub(crate) fn subfields(&self, tag: &str, code: char) -> Vec<String> {
        self.data_fields(tag)
            .flat_map(|field| Self::subfields_of(field, code))
            .collect()
    }

    fn data_fields<'tag>(&self, tag: &'tag str) -> impl Iterator<Item = Node<'a, 'input>> + 'tag
    where
        'a: 'tag,
    {
        self.node.children().filter(move |node| {
            node.has_tag_name((MARC_NAMESPACE, "datafield")) && node.attribute("tag") == Some(tag)
        })
    }

    fn subfields_of(field: Node, code: char) -> Vec<String> {
        let code = code.to_string();

        field
            .children()
            .filter(|node| {
                node.has_tag_name((MARC_NAMESPACE, "subfield"))
                    && node.attribute("code") == Some(code.as_str())
            })
            .filter_map(|node| node.text())
            .map(|text| text.trim().to_string())
            .filter(|text| !text.is_empty())
            .collect()
    }
}
