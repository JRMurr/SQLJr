use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SelectStatement {
    pub tables: Vec<String>,
    pub fields: Vec<String>,
}

impl fmt::Display for SelectStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SELECT ")?;

        write!(f, "{}", self.fields.join(", "))?;

        write!(f, " FROM ")?;

        write!(f, "{}", self.tables.join(", "))?;

        Ok(())
    }
}

/// All possible query types you can run
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlQuery {
    Select(SelectStatement),
    // create table
    // insert
    // update
    //...
}
