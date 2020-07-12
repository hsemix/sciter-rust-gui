mod structs;
mod common;
// use serde_json::{ Result, Value, json };
pub fn get_users() -> String {
    let all_persons: Vec<structs::Person> =
        common::pool().prep_exec("SELECT id, name from person", ())
 
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, name) = mysql::from_row(row);
 
                    structs::Person {
                        id,
                        name
                    }
                }).collect()
            }).unwrap(); // Unwrap `Vec<Person>`

    let j = serde_json::to_string(&all_persons).unwrap();

    j
}