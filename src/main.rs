use csv::Error;

fn main() -> Result<(), Error> {
    let csv = "name, age, hobby
        daiki, 10, rust
        tanaka, 20, javascript
        suzuki, 30, ruby";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;
        println!(
            "I am {}, {} years old. I like {}.",
            &record[0],
            &record[1],
            &record[2]
        );
    }
    Ok(())
}
