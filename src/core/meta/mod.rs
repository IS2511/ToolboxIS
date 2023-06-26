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


impl Default for AppMeta {
    fn default() -> Self {
        AppMeta {
            name: "ToolboxIS",
            version: semver::Version::parse(env!("CARGO_PKG_VERSION"))
                .expect("CARGO_PKG_VERSION is not a valid SemVer, which should be impossible"),
            authors: vec!["IS2511 <is2511@is2511.com>"],
            homepage: "https://is2511.com/",
            additional: AppMetaAdditionalResources {
                docs: None,
                download: None,
                faq: None,
                help: None,
            },
        }
    }
}
