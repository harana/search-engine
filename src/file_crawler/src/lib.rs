pub mod builder;
pub mod filter;
pub mod file_crawler;
pub mod utils;

pub use builder::FileCrawlerBuilder;
pub use filter::{FileSize, FilterExt, FilterFn};

// export this in order to use it with custom filter functions
pub use ignore::DirEntry;
pub use file_crawler::FileCrawler;
pub use utils::similarity_sort;
