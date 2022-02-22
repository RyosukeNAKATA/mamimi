use crate::python_version::PythonVersion;
use scraper;
use serde::Deserialize;
use url::Url;

mod lts_status {
    use serde::{Deserialize, Deserializer};

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    #[serde(untagged)]
    enum LtsStatus {
        Nope(bool),
        Yes(String),
    }

    impl From<LtsStatus> for Option<String> {
        fn from(status: LtsStatus) -> Self {
            match status {
                LtsStatus::Nope(_) => None,
                LtsStatus::Yes(x) => Some(x),
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(LtsStatus::deserialize(deserializer)?.into())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[derive(Deserialize)]
        struct TestSubject {
            #[serde(deserialize_with = "deserialize")]
            lts: Option<String>,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct IndexedPythonVersion {
    /// https://npm.taobao.org/mirrors/python/ mirror
    pub python_version: PythonVersion,
    #[serde(with = "lts_status")]
    pub lts: Option<String>,
    pub files: Vec<String>,
}

pub fn list() -> Result<Vec<IndexedPythonVersion>, reqwest::Error> {
    let value = reqwest::blocking::get(format!("https://www.python.org/ftp/python/").as_str())
        .unwrap()
        .text()
        .unwrap();
    let doc = scraper::Html::parse_document(&value);
    let sel = scraper::Selector::parse("a").unwrap();

    let mut versions = vec![];
    for (index, node) in doc.select(&sel).enumerate() {
        if node.inner_html().is_empty() || index == 0 {
            continue;
        }
        let mut version = node.inner_html();
        version.retain(|c| c != '/');
        versions.push(IndexedPythonVersion {
            python_version: match PythonVersion::parse(&version.to_string()) {
                Ok(v) => v,
                Err(_) => continue,
            },
        })
    }
    versions.sort_by(|a, b| a.python_version.cmp(&b.python_version));
    Ok(versions)
}
