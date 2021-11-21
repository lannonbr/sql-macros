use sql_macros::SQL;

#[derive(SQL)]
struct User {
    name: String,
    email: String,
}

fn main() {
    let create_stmt = User::create_table();
    println!("{}", create_stmt);
}

// OUTPUTS:

// CREATE TABLE User (
//   name text,
//   email text
// );
