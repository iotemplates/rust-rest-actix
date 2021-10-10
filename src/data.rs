use std::fs;
use std::collections::HashMap;
use crate::models::Person;

pub fn read_data(mut data:  HashMap<String, Person>) -> HashMap<String, Person> {
    let content: String = fs::read_to_string("persons.json").expect("Failed to read file persons.json");
    let persons: Vec<Person> = serde_json::from_str(&content).expect("Failed to deserialize content of person.json.");
    for person in persons {
        data.insert(person.id.to_string(), person);
    }
    data 
}

pub fn write_data(data: HashMap<String, Person>) -> std::io::Result<()>  {
    let persons: Vec<Person> = data.into_iter().map(|(_, v)| v).collect();
    let content: String = serde_json::to_string(&persons).expect("Failed to serialize map to json.");
    fs::write("persons.json", content)
}
