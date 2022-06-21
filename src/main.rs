use std::fmt::format;
use gluesql::prelude::*;
use gluesql::sled_storage::sled::IVec;

type GlueInstance = Glue<IVec, SledStorage>;

fn create_table(glue: &mut GlueInstance) {
    let sqls = vec![
        "DROP TABLE IF EXISTS Glue;",
        "CREATE TABLE Glue (id INTEGER, name TEXT);",
    ];
    for sql in sqls {
        glue.execute(sql).unwrap();
    }
}

fn save(glue: &mut GlueInstance, id: i32, name: &str) {
    let sql = format!("INSERT INTO Glue VALUES ({}, '{}');", id.to_string(), name);
    glue.execute(sql).unwrap();

}

fn delete(glue: &mut GlueInstance, id: i32) {
    let sql = format!("DELETE FROM Glue WHERE id = {};", id.to_string());
    glue.execute(sql).unwrap();
}

fn find_by_id(glue: &mut GlueInstance, id: i32) {
    let sql = format!("SELECT * FROM Glue WHERE id = {};", id.to_string());
    let result= glue.execute(sql).unwrap();

    println!("{:?}", result)
}

fn main() {
    let storage = SledStorage::new("data/doc-db").unwrap();
    let mut glue = Glue::new(storage);

    // 테이블 생성
    create_table(&mut glue);

    // 1번 ID row 저장
    save(&mut glue, 1, "hello");
    // 1번 ID row 조회
    find_by_id(&mut glue, 1);
    // 1번 ID row 삭제
    delete(&mut glue, 1);
    // 1번 ID row 조회
    find_by_id(&mut glue, 1);
}
