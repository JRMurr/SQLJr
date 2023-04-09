mod page;

struct Cell;

struct Page<Records> {
    cells: Vec<Cell>,

    records: Vec<Records>,
}
