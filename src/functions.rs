use rusqlite::functions::{Context, FunctionFlags};
use rusqlite::{Connection, Result};
use std::path::Path;

fn parent_path(ctx: &Context) -> Result<Option<String>> {
    assert_eq!(ctx.len(), 1, "called with unexpected number of arguments");
    let input = ctx.get_raw(0).as_str()?;
    let path = Path::new(input);
    let output = path.parent().map(|p| format!("{}/", p.display()));

    Ok(output)
}

pub fn add_parent_function(conn: &Connection) -> Result<()> {
    conn.create_scalar_function(
        "parent_path",
        1,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        parent_path,
    )
}
