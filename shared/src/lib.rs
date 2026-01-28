mod list_of_lists;
mod logging;
mod man;

pub use list_of_lists::fetch_lists;
pub use logging::setup_logging;
pub use man::{ManExample, generate_man_page};
