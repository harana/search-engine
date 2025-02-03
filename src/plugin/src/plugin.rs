use harana_common::itertools::Either;
use harana_common::serde::{self, Deserialize, Serialize};
use crate::cards::PluginCardBuiltin;

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(crate = "self::serde")]
pub struct Plugin {
    pub name: String,
    pub description: String,
    pub version: String,
    pub parameters: Either<Vec<PluginParameter>, Vec<PluginParameterGroup>>,
    pub authentication_type: PluginAuthenticationType,
    pub action_buttons: Vec<Either<PluginActionButtonBuiltin, PluginActionButton>>,
    pub cards: Vec<Either<PluginCardBuiltin, PluginCard>>
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(crate = "self::serde")]
#[serde(rename_all = "kebab-case")]
pub enum PluginActionButtonBuiltin {
    OpenFinder,
    Share,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(crate = "self::serde")]
#[serde(rename_all = "kebab-case")]
pub struct PluginActionButton {
    pub title: String,
    pub action_name: String
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(crate = "self::serde")]
pub struct PluginAction {
    pub title: String,
    pub description: Option<String>,
    pub parameters: Vec<PluginParameter>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(crate = "self::serde")]
#[serde(rename_all = "kebab-case")]
pub enum PluginAuthenticationType {
    None,
    HttpBasic,
    Oauth2,
    Token
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(crate = "self::serde")]
pub struct PluginCard {
    pub title: String,
    pub description: Option<String>,
    pub parameters: Vec<PluginParameter>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(crate = "self::serde")]
pub struct PluginParameterGroup {
    pub title: String,
    pub description: Option<String>,
    pub parameters: Vec<PluginParameter>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(crate = "self::serde")]
pub struct PluginParameter {
    pub name: String,
    pub title: String,
    pub description: Option<String>,
    pub kind: PluginParameterKind,
    pub options: Vec<PluginParameterOption>
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(crate = "self::serde")]
#[serde(rename_all = "kebab-case")]
pub enum PluginParameterKind {
    File,
    Integer,
    IntegerList,
    String,
    StringList,
    Url
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(crate = "self::serde")]
pub struct PluginParameterOption {
    pub title: String,
    pub description: String,
    pub value: String
}