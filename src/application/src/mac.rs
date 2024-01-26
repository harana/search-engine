use std::{
    path::{Path, PathBuf},
    process::Command,
};
use std::fs::copy;
use std::process::Stdio;
use directories::UserDirs;

use plist::Value;
use rust_search::SearchBuilder;
use harana_common::itertools::Itertools;
use crate::manager::Application;

pub fn applications(target_path: &PathBuf) -> Vec<Application> {
    SearchBuilder::default()
        .location("/Applications")
        .more_locations(vec![
            "/System/Applications",
            "/System/Applications/Utilities",
            UserDirs::new().unwrap().home_dir().join("Applications").to_str().unwrap()
        ])
        .depth(2)
        .ext(".app")
        .ignore_case()
        .build()
        .map(|app_path| {
            let app_name = app_path.split("/").last().unwrap().to_string();
            let source_icon_path = get_icon_path(app_path.as_str());

            Application {
                name: app_name.clone().replace(".app", ""),
                path: app_path.clone(),
                source_icon_path: source_icon_path.clone(),
                target_icon_path:
                    if source_icon_path.clone().is_some() {
                        Some(format!("{}/{}.png", target_path.to_str().unwrap(), app_name.clone()))
                    }else {
                        None
                    }
            }
        })
        .collect_vec()
}

pub fn generate_icons(applications: Vec<Application>) {
    applications.clone()
        .iter()
        .filter(|a| a.source_icon_path.is_some())
        .foreach(|a|
            if a.source_icon_path.clone().unwrap().ends_with(".png") {
                let _ = copy(a.source_icon_path.clone().unwrap(), a.target_icon_path.clone().unwrap());
            } else {
                to_png(
                    PathBuf::from(a.source_icon_path.clone().unwrap()),
                    PathBuf::from(a.target_icon_path.clone().unwrap())
                )
            }
        );
}

fn to_png(src_path: PathBuf, target_path: PathBuf) {
    if !target_path.exists() {
        Command::new("sips")
            .arg("-s")
            .arg("format")
            .arg("png")
            .arg(src_path)
            .arg("--out")
            .arg(target_path)
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .expect("failed to execute process");
    }
}

fn get_icon_path(app_path: &str) -> Option<String> {
    let plist = Value::from_file(app_path.to_owned() + &"/Contents/Info.plist");
    match plist {
        Ok(plist) => {
            let icon_path = plist.as_dictionary().unwrap().get("CFBundleIconFile");
            if icon_path.is_some() {
                let mut icon_path = icon_path.unwrap().as_string().unwrap().to_string();
                if !icon_path.ends_with(".icns") && !icon_path.ends_with(".png") { icon_path.push_str(".icns") };
                return Some(app_path.to_owned() + &"/Contents/Resources/" + icon_path.as_str());
            }
            return None;
        }
        Err(_) => {
            return None;
        }
    }
}