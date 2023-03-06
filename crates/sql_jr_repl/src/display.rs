use sql_jr_execution::ExecResponse;
use tabled::builder::Builder;

pub fn display_response(res: ExecResponse) {
    match res {
        ExecResponse::Select(table_iter) => {
            let mut builder = Builder::default();

            let columns: Vec<String> = table_iter
                .columns
                .iter()
                .map(|col| col.name.to_string()) // Involves a clone, maybe we can make col name a Cow?
                .collect();
            builder.set_columns(&columns);
            for row in table_iter {
                builder.add_record(columns.iter().map(|col| row.get(col).to_string()));
            }
            println!("{}", builder.build())
        }
        _ => println!("{res}"),
    }
}
