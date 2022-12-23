mod insert;
mod select;
pub use insert::InsertStatement;
pub use select::SelectStatement;

pub(crate) use insert::insert_statement;
pub(crate) use select::select_statement;
