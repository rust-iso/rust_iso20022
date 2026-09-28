use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CatalogueMessage {
    pub message_id: &'static str,
    pub namespace: &'static str,
    pub root_element: &'static str,
    pub required_feature: &'static str,
}

#[derive(Debug, Serialize)]
pub struct CatalogueData {
    pub family: String,
    pub messages: Vec<CatalogueMessage>,
}

#[derive(Debug, Serialize)]
pub struct VersionsData {
    pub family: String,
    pub versions: Vec<&'static str>,
}

pub fn catalog(family: &str) -> CatalogueData {
    CatalogueData {
        family: family.to_owned(),
        messages: rust_iso20022::catalogue::list_family(family)
            .iter()
            .map(|descriptor| CatalogueMessage {
                message_id: descriptor.identity,
                namespace: descriptor.namespace,
                root_element: descriptor.root_element,
                required_feature: descriptor.required_feature,
            })
            .collect(),
    }
}

pub fn versions(family: &str) -> VersionsData {
    VersionsData {
        family: family.to_owned(),
        versions: rust_iso20022::catalogue::versions(family)
            .iter()
            .map(|descriptor| descriptor.identity)
            .collect(),
    }
}
