use chrono::NaiveDate;
use strum::EnumIter;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter, Debug)]
pub enum AttributeType {
   Title, 
   Author, 
   Locale,
   Language,
   Site,
   Date,
   Url,
   Type
}

#[derive(Clone, Debug)]
pub enum Attribute {
    Title(String),
    Author(Vec<String>),
    Date(NaiveDate),
    Language(String),
    Locale(String),
    Site(String),
    Url(String),
    Journal(String),
    Publisher(String)
}