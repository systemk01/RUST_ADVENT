//ToDo of 5th of December
//Parsing csv to String and integer u32

pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}
#[derive(Debug)]
pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    pub fn parse_row(csv_row: &str) -> Result<Kid, &'static str> {
        // 🎁 Transform the CSV row into a Kid struct for Santa's list!
        // 🎅 Expected CSV: "Name,GoodDeeds,BadDeeds"
        //    Example: "Alice,3,1" -> name: "Alice", good_deeds: 3, bad_deeds: 1

        let mut fields = csv_row.split(',');
        let name = fields.next().ok_or("Missing field")?;
        let name = name.to_string();
        let good_deeds= fields.next().ok_or("Missing field")?.parse().map_err(|_| "Invalid good deeds")?;
        let bad_deeds= fields.next().ok_or("Missing field")?.parse().map_err(|_| "Invalid bad deeds")?;

        Ok(Self::new(name, good_deeds, bad_deeds))
        }

    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Self {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };

        Self { name, niceness }
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
    let people = "Hubi,20,2";
    let kid = Kid::parse_row(people);
    println!("kid {:?}", kid);
}
