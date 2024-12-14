//ToDo of 4th of December

pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)] // needed for tests
pub enum Niceness {
    // Create the enum variants `Nice` and `Naughty`
    // Variant `Nice` is a tuple struct that holds the number of good deeds
    Nice(u32),
    Naughty,
}
#[derive(Debug)]
pub struct Kid {
    // Add a field `name` of type `String`
    // and `niceness` field of type `Niceness`
    // Make all fields public
    pub name: String,
    pub niceness: Niceness,
}

// Move yesterday's function to an associated function in the struct

impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        let x: Kid = if !Kid::is_nice(good_deeds, bad_deeds) {
            Kid {
                name,
                niceness: Niceness::Naughty,
            }
        } else {
            Kid {
                name,
                niceness: Niceness::Nice(good_deeds),
            }
        };
        x
    }

    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }

        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

        let ratio = good_deeds / (good_deeds + bad_deeds);

        ratio >= 0.75
    }
}

fn main() {
    let bad = Niceness::Naughty;
    let good = Niceness::Nice(4);

    let good_kid = Kid {
        name: "hins".to_string(),
        niceness: good,
    };
    let bad_kid = Kid {
        name: "fofo".to_string(),
        niceness: bad,
    };

    println!("good kid is {:?}", good_kid);
    println!("bad kid is {:?}", bad_kid);

    println!("{}", Kid::is_nice(1, 10));
}
