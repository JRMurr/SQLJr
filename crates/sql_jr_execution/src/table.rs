use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

type Row = HashMap<String, String>;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]

pub(crate) struct Table {
    /// row id to row
    pub rows: BTreeMap<usize, Row>,
}
