use std::fmt::Display;

use slab::Slab;

type SlabKey = usize; // TODO: maybe new type it?

#[derive(Debug, Clone)]
struct Cell<Key> {
    /// The primary key of the row
    row_primary_key: Key,
    /// The key used to find the row in the slab
    slab_key: SlabKey,
}

/// A DB page following the [slotted page](https://siemens.blog/posts/database-page-layout/) approach
///
/// The actual records will be stored in a [`Slab`] for efficient lookup. The
/// keys for the slab are stored in the [`Cell`]s list along with their row
/// primary key
#[derive(Debug, Clone)]
struct Page<K, V> {
    cells: Vec<Cell<K>>,

    records: Slab<V>, // Look in to slot map
}

impl<K: PartialEq + Ord + Display, V> Page<K, V> {
    pub fn new() -> Self {
        // TODO: pass in b-tree params for max num elements to figure out slab
        // capacity
        let max_entries = 5;

        Self {
            cells: Vec::with_capacity(max_entries),
            records: Slab::with_capacity(max_entries),
        }
    }

    /// Insert a new key value pair into the page
    pub fn insert(&mut self, key: K, value: V) {
        // insert into cells in sorted order by key
        let cell_pos = match self
            .cells
            .binary_search_by(|cell| cell.row_primary_key.cmp(&key))
        {
            Ok(_) => panic!("Key {key} already in page"), // TODO: return result
            Err(pos) => pos,
        };

        let cell = {
            let entry = self.records.vacant_entry();
            let slab_key = entry.key();

            entry.insert(value);
            Cell {
                row_primary_key: key,
                slab_key,
            }
        };
        self.cells.insert(cell_pos, cell);
    }

    /// Remove a value from the page by key, returns [`None`] if the key is not
    /// found
    pub fn remove(&mut self, key: K) -> Option<V> {
        let (idx, cell) = match self.find_cell_idx(key) {
            Some(cell_idx) => cell_idx,
            None => return None,
        };

        let value = self.records.remove(cell.slab_key);

        self.cells.remove(idx);

        Some(value)
    }

    fn find_cell_idx(&self, key: K) -> Option<(usize, &Cell<K>)> {
        self.cells
            .iter()
            .enumerate()
            .find(|(_, cell)| cell.row_primary_key == key)
    }
}
