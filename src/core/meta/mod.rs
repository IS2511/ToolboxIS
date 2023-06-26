pub mod update;


pub struct AppMeta {
    pub name: &'static str,
    pub version: semver::Version,
    pub authors: Vec<&'static str>,
    pub homepage: &'static str,
    pub additional: AppMetaAdditionalResources,
}

pub struct AppMetaAdditionalResources {
    pub docs: Option<&'static str>,
    pub download: Option<&'static str>,
    pub faq: Option<&'static str>,
    pub help: Option<&'static str>,
}
