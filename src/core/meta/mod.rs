pub mod update;

pub struct AppMeta {
    name: String,
    version: semver::Version,
    authors: Vec<String>,
    homepage: String,
    additional: AppMetaAdditionalResources,
}

pub struct AppMetaAdditionalResources {
    docs: Option<String>,
    download: Option<String>,
    faq: Option<String>,
    help: Option<String>,
}
