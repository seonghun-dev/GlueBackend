
use gluesql::prelude::*;

pub fn init(){
    /*
    create GlueBoard table
    */
    let storage = SledStorage::new("data/doc-db").unwrap();
    let mut glue = Glue::new(storage);
    let sqls = vec![
        "DROP TABLE IF EXISTS GlueBoard;",
        "CREATE TABLE GlueBoard (id INTEGER, title TEXT, content TEXT);",
    ];

    for sql in sqls {
        let output = glue.execute(sql).unwrap();
        println!("{:?}", output)
    }
}

pub fn add_test_data(){
    /*
    Add test data to GlueBoard DataBase
    Parmeters:
    return:
    */

    let storage = SledStorage::new("data/doc-db").unwrap();
    let mut glue = Glue::new(storage);
    let sqls = vec![
        "INSERT INTO GlueBoard VALUES (100, 'Ferris', 'Hello World');",
        "INSERT INTO GlueBoard VALUES (200, 'Ferris', 'Hello Wolrd2');",
    ];

    for sql in sqls {
        let output = glue.execute(sql).unwrap();
        println!("{:?}", output)
    }
}

pub fn get_content() -> String {
    /*
    Get content from GlueBoard DataBase
    parameters:
    return:
    */

    let storage = SledStorage::new("data/doc-db").unwrap();
    let mut glue = Glue::new(storage);
    let sql = "SELECT * FROM GlueBoard WHERE id >= 100;";
    let output = glue.execute(sql).unwrap();
    let result = format!("{:?}", output);
    return result;
}