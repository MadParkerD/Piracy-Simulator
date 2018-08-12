#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
mod econosim;
use econosim::start_econ::*;
use std::io::{stdin, stdout, Write};

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
        money: 0,
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
    println!("{:?}", player.money);
    player.sell(&mut places[0]);
    player.buy(&mut places[0]);
    println!("{:?}", player.money);
    player.visit(&mut places);
    player.visit(&mut places);
    player.visit(&mut places);
    player.visit(&mut places);
    player.visit(&mut places);
    println!("{:?}", player);
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Ship {
    pub name: String,
    pub sizeof: btype,
    pub hold: Vec<goods>,
    pub visited: Vec<usize>,
    pub crew: Vec<Sailor>,
    pub money: u64,
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
    large = 100,
    medium = 50,
    small = 25,
}

impl Health {
    pub fn total(&self) -> u64 {
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

pub fn navigate(places: &mut Vec<port>, index: usize) -> usize {
    let mut tracker = 0;
    let mut destination = index;
    for i in (&places[index].route) {
        tracker = tracker + 1;
        println!("{}:{:?}", tracker, places[*i]);
    }
    let mut s = String::new();
    print!("Which port to visit?: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    let x: usize = s.parse().unwrap();
    destination = places[index].route[x - 1];
    destination
}
impl Ship {
    pub fn buy(&mut self, place: &mut port) {
        let capacity = match &self.sizeof {
            btype::large => btype::large as u64,
            btype::medium => btype::medium as u64,
            btype::small => btype::small as u64,
        };
    }
    pub fn sell(&mut self, currentLoc: &mut port) {
        self.money = &self.money + currentLoc.update(self.hold.as_mut_slice(), false);
        self.hold = vec![];
    }
    pub fn visit(&mut self, places: &mut Vec<port>) {
        let index = self.visited[&self.visited.len() - 1];
        self.visited.push(navigate(places, index));
        if self.visited.len() > 10 {
            self.visited.remove(0);
        }
    }
}
