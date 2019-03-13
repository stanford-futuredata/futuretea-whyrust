
use serde::*;
use serde_json;

#[derive(Serialize,Deserialize,Debug,Clone)]
struct Student(String);

#[derive(Serialize,Deserialize,Debug,Clone)]
struct Professor(String);

#[derive(Serialize,Deserialize,Debug,Clone)]
struct FutureData {
    phd_students: Vec<Student>,
    professors: Vec<Professor>,
}

fn main() {
    let fds = FutureData {
        phd_students: vec![
            "cody", "deeptir", "egan1",
            "fabuzaid", "kaisheng", "kexin",
            "kraftp", "pratiksha", "sahaana",
            "shoumik", "ddkang", "deepakn94",
            "jamesjoethomas", "keshav",
            "ankit", "saachi", "justin",
            "swetha", "animesh"
        ].into_iter().map(|e| Student(e.to_string())).collect(),

        professors: vec!["matei", "pbailis"].into_iter()
            .map(|e| Professor(e.to_string())).collect(),
    };

    println!("{}", serde_json::to_string(&fds).unwrap());
}
