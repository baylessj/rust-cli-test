use semver::Version;
use serde::Deserialize;

const MAINLINE_LOCATION: &str = "https://purduesigbots.github.io/pros-mainline/pros-mainline.json";

// {
// 	"metadata": {
// 		"output": "bin/output.bin",
// 		"location": "https://pros.cs.purdue.edu/v5/_static/releases/kernel@3.1.0.zip"
// 	},
// 	"name": "kernel",
// 	"py/object": "pros.conductor.templates.base_template.BaseTemplate",
// 	"supported_kernels": null,
// 	"target": "v5",
// 	"version": "3.1.0"
// },
#[derive(Deserialize, Debug)]
struct TemplateMetadata {
    output: Option<String>,
    location: String,
}

#[derive(Deserialize, Debug)]
struct TemplateData {
    metadata: TemplateMetadata,
    name: String,
    #[serde(rename = "py/object")]
    pyobject: String,
    supported_kernels: Option<String>,
    target: String,
    version: Version,
}

pub fn new_project() -> Result<String, Box<dyn std::error::Error>> {
    let mainline = reqwest::blocking::get(MAINLINE_LOCATION)?.json::<Vec<TemplateData>>()?;
    // Assuming for now that we only care about PROS V5 kernels and okapilib
    let mut kernels = mainline
        .iter()
        .filter(|&x| x.target == "v5" && x.name == "kernel")
        .collect::<Vec<&TemplateData>>();
    kernels.sort_by(|a, b| b.version.cmp(&a.version)); // sort as b>a to get largest first
    let to_install = kernels
        .first()
        .ok_or("Could not find any valic kernels to install")?;

    let mut tmpfile = tempfile::tempfile().unwrap();
    reqwest::blocking::get(&to_install.metadata.location)?.copy_to(&mut tmpfile)?;
    let mut zip = zip::ZipArchive::new(tmpfile)?;

    zip.extract("./pros_project")?;
    Ok("".to_string())
}
