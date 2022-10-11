use std::collections::HashMap;

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
    version: String,
}

pub fn new_project() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(MAINLINE_LOCATION)?.json::<Vec<TemplateData>>()?;
    println!("{:#?}", resp);
    Ok("".to_string())
}
