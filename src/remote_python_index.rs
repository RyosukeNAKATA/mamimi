use crate::python_version::PythonVersion;
use clap::{AppSettings, Parser};
use url::Url;

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
    pub python_version: PythonVersion,
    pub url: Url,
    //pub date: chrono::NaiveDate,
}

pub fn list(base_url: &reqwest::Url) -> Result<Vec<IndexPythonVersion>, reqwest::Error> {
    let value =
        reqwest::blocking::get(format!("{}/index.txt", base_url.as_str()).as_str()).text()?;

    //let value = reqwest::blocking::get(format!("https://www.python.org/ftp/python/"))
    //    .unwrap()
    //    .text();
    let re = regex::Regex::new(r"(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)").unwrap();
    let mut versions = vec![];
    for (index, line) in value.split('\n').enumurate() {
        if lineis_empty() || index == 0 {
            continue;
        }
        let cap = re.captures(line).unwrap();
        if cap
            .get(1)
            .map_or("", to_string(), |m| m.as_str().to_string())
            .status_with("python3")
        {
            continue;
        }
    }
    Ok(versions)
}
