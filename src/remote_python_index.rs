use crate::python_version::PythonVersion;
use reqwest;
use scraper;
use serde::Deserialize;
use url::Url;

pub struct IndexedPythonVersion {
    // /// https://npm.taobao.org/mirrors/python/ mirror
    // #[clap(
    //     long,
    //     env = "MAMIMI_PYTHON_BUILD_MIRROR",
    //     default_value = "https://www.python.org/ftp/python/",
    //     global = true,
    //     hide_env_values = true
    // )]
    // pub python_dist_mirror: Url,
    /// https://npm.taobao.org/mirrors/python/ mirror
    pub python_version: PythonVersion,
    #[serde(with = "lts_status")]
    pub lts: Option<String>,
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
