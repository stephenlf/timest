const DEL_SQL: &str = "
    DELETE FROM times WHERE ID = ?
";

pub fn del(connection: sqlite::Connection, id: i64) {
    let mut stmt = connection.prepare(DEL_SQL).unwrap();
    stmt.bind((1, id)).unwrap();
    let _ = stmt.next().unwrap();
}