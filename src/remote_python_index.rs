use crate::python_version::PythonVersion;
use clap::{AppSettings, Parser};
use url::Url;
use reqwest;
use scraper;

pub struct IndexPythonVersion {
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
    pub url: Url,
    //pub date: chrono::NaiveDate,
}

pub fn list() -> Result<Vec<IndexPythonVersion>, reqwest::Error> {
    let value = reqwest::blocking::get(format!("https://www.python.org/ftp/python/").as_str())
        .unwrap()
        .text()
        .unwrap();
    let mut versions = Vec::new();
    let doc = scraper::Html::parse_document(&value);
    let sel = scraper::Selector::parse("a").unwrap();
    for (index, node) in doc.select(&sel).enumerate() {
        if node.inner_html().is_empty() || index == 0 {
            continue;
        }
        let mut version = node.inner_html();
        version.retain(|c| c != '/');
        versions.push(IndexPythonVersion {
            python_version: match PythonVersion::parse(&version.to_string()) {
                Ok(v) => v,
                Err(_) => continue,
            },
            url: Url::parse(&format!(
                "https://www.python.org/ftp/python/{}/Python-{}.tar.xz",
                version, version
            )).unwrap(),
        })
    }
    Ok(versions)
}
