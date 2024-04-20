use leetcode_tui_db::define_schema;
use native_db::{Database, DatabaseBuilder};

pub fn build_db<'a, 'b>(
    db_builder: &'a mut DatabaseBuilder,
) -> leetcode_tui_db::errors::DBResult<Database<'a>> {
    define_schema(db_builder)?;
    Ok(db_builder.create_in_memory()?)
}
