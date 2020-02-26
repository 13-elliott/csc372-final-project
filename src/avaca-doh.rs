/// ## Program: avaca-doh.rs
///
/// ## Project Google doc:
/// https://docs.google.com/document/d/1GaKXhcJAGxK3tKRVn_ZWYFrF0QlMTlOs2Nke4Wdz6u0/edit?usp=sharing
///
/// ##
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// A struct for calculating a mean
struct Averager {
    running_total: f64,
    num_averages: u64,
}

impl Averager {
    #[inline]
    /// Produces a new Averager with the given starting value
    fn new(first: f64) -> Self {
        Averager {
            running_total: first,
            num_averages: 1,
        }
    }

    ///produces the current average
    fn average(&self) -> f64 {
        self.running_total / self.num_averages as f64
    }

    /// adds the given value to the averager.
    fn add(&mut self, val: f64) {
        self.running_total += val;
        self.num_averages += 1;
    }
}

#[derive(PartialEq)]
/// A struct for organizing avocado sale information
struct AvocadoRecord {
    region: String,
    avg_cost: f64,
}

impl AvocadoRecord {
    #[inline]
    /// Constructor
    fn new(region: String, avg_cost: f64) -> Self {
        AvocadoRecord { region, avg_cost }
    }
}

impl Eq for AvocadoRecord {}
impl Ord for AvocadoRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        let mine = self.avg_cost;
        let theirs = other.avg_cost;
        if mine == theirs {
            Ordering::Equal
        } else if mine < theirs {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for AvocadoRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let file = File::open("avocado.csv").expect("Could not file file, aaaa(vocado)");
    let mapping = load_file(&file);
    let mut seq: Vec<_> = mapping
        .into_iter()
        .map(|(region, avgr)| AvocadoRecord::new(region, avgr.average()))
        .collect();
    sorts::msort(&mut seq);
    output(seq);
}

/// Display the results
fn output(records: Vec<AvocadoRecord>) {
    println!("Average Avocado costs by region, in ascending order --");
    for (i, rec) in records.into_iter().enumerate() {
        println!("{:2}) {:20}: ${:.2}", i + 1, rec.region, rec.avg_cost);
    }
}

fn load_file(file: &File) -> HashMap<String, Averager> {
    const READ_ERR: &str = "Error reading from input file";

    let mut map: HashMap<String, Averager> = HashMap::new();
    let mut in_buffer = BufReader::new(file);
    let mut line = String::new();
    let mut fields = String::new();
    in_buffer.read_line(&mut fields).expect(READ_ERR);
    loop {
        let bytes_read = in_buffer.read_line(&mut line).expect(READ_ERR);
        if bytes_read > 0 {
            let mut spliterator = line.split(",");
            let price: f64 = spliterator.nth(2).unwrap().parse().unwrap();
            let region = String::from(spliterator.nth(10).unwrap().trim());
            match map.get_mut(&region) {
                Some(avgr) => avgr.add(price),
                None => {
                    map.insert(region, Averager::new(price));
                }
            }
            line.clear();
        } else {
            // reached EOF
            return map;
        }
    }
}
