use crate::detail::util::share::*;

use crate::*;

use crate::component::pbuilder::*;
use crate::component::pbuilder as pb;

use crate::component::sqltype as sql;



#[test]
fn example_create_database()
{
    let mut b = pb::CreateDatabase::new().unwrap();
    b.user("user");
    b.password("password");
    b.page_size(4096);
    Connection::create_database("666.fdb", b).unwrap();
}

#[test]
fn example_attach_database()
{
    let mut b = pb::Connect::new().unwrap();
    b.user("user");
    b.password("password");
    let con = Connection::connect("665.fdb", b).unwrap();
}

#[test]
fn example_update()
{
    let mut b = pb::Connect::new().unwrap();
    b.user("lck");
    b.password("1");
    let con = Connection::connect("test.fdb", b).unwrap();
    let mut b = pb::Transaction::new().unwrap();
    b.write();
    let transaction = con.transaction(b).unwrap();
    let stmt = transaction.prepare("UPDATE test SET xxx = 887 WHERE id = 2").unwrap();
    transaction.execute_prepared(&stmt);
    transaction.commit();
}

#[test]
fn example_select()
{
    type Varchar = Vec<u8>;

    let mut builder = pb::Connect::new().unwrap();
    builder.user("lck");
    builder.password("1");
    let con = Connection::connect("test.fdb", builder).unwrap();
    let mut builder = pb::Transaction::new().unwrap();
    builder.write();
    let transaction = con.transaction(builder).unwrap();
    let stmt = transaction.prepare("SELECT * from persons").unwrap();
    let mut rows = transaction.execute_prepared_rows(&stmt, &[]).unwrap();
    rows.fetch_next();
    let x = rows.get::<Varchar>(1).unwrap().unwrap();
    println!("{}", String::from_utf8(x).unwrap());
    rows.fetch_next();
    rows.fetch_next();
    let x = rows.get::<Varchar>(1).unwrap().unwrap();
    println!("{}", String::from_utf8(x).unwrap());
    // rows.fetch_next();
    // let x = rows.get::<sql::Double>(0).unwrap().unwrap().value();
    // println!("{}", x);
    transaction.commit();
}

#[test]
fn example_select_with_input()
{
    type Varchar = Vec<u8>;

    let mut builder = pb::Connect::new().unwrap();
    builder.user("lck");
    builder.password("1");
    let con = Connection::connect("test.fdb", builder).unwrap();
    let mut builder = pb::Transaction::new().unwrap();
    builder.write();
    let transaction = con.transaction(builder).unwrap();
    let stmt = transaction.prepare("SELECT * from persons WHERE personid = ?").unwrap();
    let mut rows = transaction.execute_prepared_rows(&stmt, &[&4i32]).unwrap();
    rows.fetch_next();
    let x = rows.get::<Varchar>(1).unwrap().unwrap();
    println!("{}", String::from_utf8(x).unwrap());
    // rows.fetch_next();
    // rows.fetch_next();
    // let x = rows.get::<Varchar>(1).unwrap().unwrap();
    // println!("{}", String::from_utf8(x).unwrap());
    // // rows.fetch_next();
    // // let x = rows.get::<sql::Double>(0).unwrap().unwrap().value();
    // // println!("{}", x);
    transaction.commit();
}