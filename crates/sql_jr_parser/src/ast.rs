use serde::{Deserialize, Serialize};

use crate::commands::*;

/// All possible query types you can run
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlQuery {
    Select(SelectStatement),
    Insert(InsertStatement),
    // create table
    // update
    //...
}
