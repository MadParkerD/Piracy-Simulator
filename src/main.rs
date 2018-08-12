#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
mod econosim;
use econosim::start_econ::*;

fn main() {
    let mut places = bootstrap("test.txt".to_string());
    for i in &places {
        println!("{:?}", i);
    }
    let mut player: Ship = Ship {
        name: "Black Pearl".to_string(),
        sizeof: btype::large,
        hold: vec![
            goods::iron(1),
            goods::sugar(100),
            goods::gold(34),
            goods::cotton(50),
        ],
        visited: vec![0, 1, 2, 1, 2, 1, 0, 1, 0],
        crew: vec![Sailor {
            Name: "Jack".to_string(),
            Health: Health {
                Wounds: vec![wound::arm(10), wound::leg(10)],
            },
        }],
    };
    println!("\n{:?}", player);
    let Bob = Sailor {
        Name: "Bob".to_string(),
        Health: Health {
            Wounds: vec![wound::arm(10), wound::leg(25)],
        },
    };
    let x: u64 = Bob.Health.total();
    println!("{}", x);
    places[0].update(player.hold.as_mut_slice(), true);
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Ship {
    pub name: String,
    pub sizeof: btype,
    pub hold: Vec<goods>,
    pub visited: Vec<usize>,
    pub crew: Vec<Sailor>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Sailor {
    pub Name: String,
    pub Health: Health,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Health {
    pub Wounds: Vec<wound>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum wound {
    arm(u64),
    hand(u64),
    leg(u64),
    chest(u64),
    vitals(u64),
    eye(u64),
}
#[derive(Debug, Serialize, Deserialize)]
pub enum btype {
    large,
    meduim,
    small,
}

impl Health {
    fn total(&self) -> u64 {
        let mut health = 0;
        for i in &self.Wounds {
            health = match i {
                wound::arm(value) => value + health,
                wound::leg(value) => value + health,
                wound::hand(value) => value + health,
                wound::chest(value) => value + health,
                wound::vitals(value) => value + health,
                wound::eye(value) => value + health,
                _ => 0 + health,
            };
        }
        health
    }
}
