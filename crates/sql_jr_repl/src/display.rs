use sql_jr_execution::ExecResponse;
use tabled::builder::Builder;

pub fn display_response(res: ExecResponse) {
    match res {
        ExecResponse::Select(rows) => {
            let mut builder = Builder::default();

            let row = rows.get(0).expect("For now assuming we get data back");

            let columns: Vec<String> = row
                .columns()
                .iter()
                .map(|col| col.name.to_string()) // Involves a clone, maybe we can make col name a Cow?
                .collect();
            builder.set_columns(&columns);
            for row in rows.into_iter() {
                builder.add_record(columns.iter().map(|col| row.get(col)));
            }
            println!("{}", builder.build())
        }
        _ => println!("{res}"),
    }
}
