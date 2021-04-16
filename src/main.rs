use sqlite::State;
use sqlite::Value;

fn create_table(connection: &sqlite::Connection) {    

    connection
    .execute(
        "
        CREATE TABLE IF NOT EXISTS users (name TEXT, age INTEGER);
        INSERT INTO users VALUES ('Alice', 42);
        INSERT INTO users VALUES ('Bob', 69);
        ",
    )
    .unwrap();

}

fn read_table(connection: &sqlite::Connection) {
    
    connection
    .iterate("SELECT * FROM users WHERE age > 50", |pairs| {
        for &(column, value) in pairs.iter() {
            println!("{} = {}", column, value.unwrap());
        }
        true
    })
    .unwrap();

}

fn query_table(connection: &sqlite::Connection) {

    let mut statement = connection
    .prepare("SELECT * FROM users WHERE age > ?")
    .unwrap();

    statement.bind(1, 50).unwrap();

    while let State::Row = statement.next().unwrap() {
        println!("name = {}", statement.read::<String>(0).unwrap());
        println!("age = {}", statement.read::<i64>(1).unwrap());
    }
}

fn query_cursor(connection: &sqlite::Connection) {
    let mut cursor = connection
    .prepare("SELECT * FROM users WHERE age > ?")
    .unwrap()
    .into_cursor();

    cursor.bind(&[Value::Integer(50)]).unwrap();

    while let Some(row) = cursor.next().unwrap() {
        println!("name = {}", row[0].as_string().unwrap());
        println!("age = {}", row[1].as_integer().unwrap());
    }
}

fn main() {
    let connection = sqlite::open(":memory:").unwrap();
    create_table(&connection);
    read_table(&connection);
    query_table(&connection);
    query_cursor(&connection);
}
