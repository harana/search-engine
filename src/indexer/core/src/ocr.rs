use std::path::Path;
// use leptess::Variable;

use harana_common::anyhow::{Error, Result};

#[cfg(target_os = "macos")]
static DEBUG_FILE: &str = "/dev/null";
#[cfg(target_os = "linux")]
static DEBUG_FILE: &str = "/dev/null";
#[cfg(target_os = "windows")]
static DEBUG_FILE: &str = "NUL";

// FIXME: Need to add Tessearct/Liptonica to Conan
pub fn text_file(path: &Path) -> Result<String> {
    Result::Ok("")
    // let mut lt = leptess::LepTess::new(None, "eng")?;
    // lt.set_variable(Variable::TesseditCharWhitelist, " :./|0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")?;
    // lt.set_variable(Variable::TesseditPagesegMode, "11")?;
    // lt.set_image(path)?;
    // lt.set_variable(Variable::DebugFile, DEBUG_FILE)?;
    // lt.get_utf8_text().map_err(|e| { Error::msg(e.to_string()) })
}

pub fn text_image(tiff_image: &[u8], dpi: i32) -> Result<String> {
    Result::Ok("")
    // let mut lt = leptess::LepTess::new(None, "eng")?;
    // lt.set_variable(Variable::TesseditCharWhitelist, " :./|0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")?;
    // lt.set_image_from_mem(tiff_image)?;
    // lt.set_variable(Variable::TesseditPagesegMode, "11")?;
    // lt.set_source_resolution(dpi);
    // lt.set_variable(Variable::DebugFile, DEBUG_FILE)?;
    // lt.get_utf8_text().map_err(|e| { Error::msg(e.to_string()) })
}