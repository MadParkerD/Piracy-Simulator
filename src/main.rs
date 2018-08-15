#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate num_derive;
extern crate num_traits;
mod econosim;
use econosim::start_econ::*;
use num_traits::FromPrimitive;
use serde_json::to_string;
use serde_json::*;
use std::fmt;

use rand::{thread_rng, Rng};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{stdin, stdout, Write};
use std::path::Path;

fn main() {
    println!("Welcome to  ̶ ̶p̶i̶r̶a̶c̶y̶  economy simulator");
    println!("I managed to vastly underestimate the scope of this project. Currently it has implimented a working economy and the ability to grow your riches as a merchant ship. ");
    let mut places = bootstrap("places.txt".to_string());
    let mut ships = bootstraps("ships.txt".to_string());
    let mut player: Ship = Ship {
        name: "Black Pearl".to_string(),
        sizeof: btype::large,
        hold: vec![
            goods::iron(0),
            goods::sugar(0),
            goods::gold(0),
            goods::cotton(0),
            goods::ice(0),
        ],
        visited: vec![0, 1, 2, 1, 2, 1, 0, 1, 0],
        crew: vec![
            Sailor {
                Name: "Jack Sparrow".to_string(),
                Health: Health {
                    Wounds: vec![wound::arm(10), wound::leg(10)],
                },
            },
            Sailor {
                Name: "William Turner".to_string(),
                Health: Health { Wounds: vec![] },
            },
        ],
        money: 5000,
    };
    let mut session_not_over = true;
    while session_not_over {
        match main_menu() {
            1 => println!("{}", player),
            2 => market_menu(
                &mut places[player.visited[player.visited.len() - 1]],
                &mut player,
            ),
            3 => {
                player.visit(&mut places);
                for i in 0..ships.len() {
                    ships[i].npc_visit(&mut places);
                    let x = ships[i].visited[ships[i].visited.len() - 1];
                    ships[i].sell_all(&mut places[x]);
                    ships[i].npc_buy(&mut places[x]);
                }
            }
            4 => {
                println!("{}", places[player.visited[player.visited.len() - 1]]);
                println!("\n\nOther ships at this port:");
                for i in 0..ships.len() {
                    if player.visited[player.visited.len() - 1]
                        == ships[i].visited[ships[i].visited.len() - 1]
                    {
                        println!("{}", ships[i]);
                    }
                }
            }
            5 => session_not_over = false,
            _ => panic!("Nope >5 number >_>"),
        }
    }
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
#[derive(Debug, Serialize, Deserialize, FromPrimitive, Clone, Copy)]
pub enum btype {
    large = 100,
    medium = 50,
    small = 25,
}

pub fn main_menu() -> usize {
    println!("1: Display ship");
    println!("2: Buy/Sell");
    println!("3: Travel to another port");
    println!("4: Display current port");
    println!("5: Exit");
    let mut x = choice();
    if x > 5 {
        x = 6
    }
    x
}

pub fn market_menu(place: &mut port, player: &mut Ship) {
    println!("Do you want to buy(1), sell(2), or check prices(3)?");
    let x = choice();
    match x {
        1 => player.buy(place),
        2 => player.sell(place),
        3 => println!("{:?}", place.economy),
        _ => {}
    }
}

pub fn choice() -> usize {
    let mut s = String::new();
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
    let mut x: usize = s.parse().unwrap();
    x
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
        println!("{}: {}", tracker, places[*i]);
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
        let capacity = self.get_space();
        let x: (Vec<goods>, u64) = self.buy_menu(place, capacity);
        self.money = x.1;
        self.hold = {
            let mut newhold: Vec<goods> = vec![];
            for i in x.0 {
                for j in &self.hold {
                    if j.same(&i) {
                        newhold.push(i.combine(*j));
                    }
                }
            }
            newhold
        };
    }
    pub fn sell(&mut self, currentLoc: &mut port) {
        let mut valid_sale = false;
        let mut values: Vec<u64> = vec![];
        let mut tracker = 0;
        for i in &self.hold {
            valid_sale = false;
            while !valid_sale {
                valid_sale = true;

                let mut s = String::new();
                println!("Sell how many {:?}(Remaining)", i);
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
                let x: u64 = s.parse().unwrap();

                if (x < i.getVal() || x == i.getVal()) {
                    self.money =
                        self.money + currentLoc.update(vec![i.setVal(x)].as_mut_slice(), false);
                    values.push(x)
                //Add new value after sale
                //swap i and new value, remove i
                } else {
                    println!("Invalid number");
                    valid_sale = false;
                }
            }
            tracker = tracker + 1;
        }
        for i in 0..self.hold.len() - 1 {
            let v = self.hold[i].getVal();
            let c = self.hold[i].clone();
            c.swap(&mut self.hold, i, v - values[i]);
        }
    }
    pub fn sell_all(&mut self, currentLoc: &mut port) {
        self.money = &self.money + currentLoc.update(self.hold.as_mut_slice(), false);
        self.hold = vec![
            goods::iron(0),
            goods::gold(0),
            goods::cotton(0),
            goods::sugar(0),
            goods::ice(0),
        ];
    }
    pub fn npc_visit(&mut self, places: &mut Vec<port>) {
        let mut rng = thread_rng();
        let n: usize = rng.gen_range(0, places[self.visited[self.visited.len() - 1]].route.len());
        let x = places[self.visited[self.visited.len() - 1]].route[n];
        self.visited.push(x);
        if self.visited.len() > 10 {
            self.visited.remove(0);
        }
    }
    pub fn npc_buy(&mut self, place: &mut port) {
        let mut rng = thread_rng();
        let n: usize = rng.gen_range(0, 4);
        let (a, b, c, d, e) = (0, 0, 0, 0, 0);
        let mut uhh: Vec<u64> = vec![a, b, c, d, e];
        let uhhtwopointoh = self.sizeof.clone();
        uhh[n] = uhhtwopointoh as u64;
        let mut x = vec![
            goods::sugar(uhh[0]),
            goods::iron(uhh[1]),
            goods::ice(uhh[2]),
            goods::cotton(uhh[3]),
            goods::gold(uhh[4]),
        ];
        place.update(x.as_mut_slice(), false);
        self.hold = x;
    }
    pub fn visit(&mut self, places: &mut Vec<port>) {
        let index = self.visited[&self.visited.len() - 1];
        self.visited.push(navigate(places, index));
        if self.visited.len() > 10 {
            self.visited.remove(0);
        }
    }
    pub fn buy_menu(&self, place: &mut port, capacity: usize) -> (Vec<goods>, u64) {
        let mut x = 0;
        let mut total = 0;
        let mut remaining_money = 0 as isize;
        let mut bad_hold_size = true;
        let mut newhold: Vec<goods> = vec![];
        let mut temp = goods::sugar(1000000);
        while bad_hold_size {
            total = 0;
            remaining_money = self.money as isize;
            for i in &place.economy {
                println!("Money remaining: {}", remaining_money);
                println!("Hold space remaining: {}", capacity - total);
                println!("How many {:?}(cost)", i);
                let mut s = String::new();
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
                total = total + x;
                remaining_money = remaining_money - (i.getVal() as usize * x) as isize;
                temp = match i {
                    goods::sugar(val) => goods::sugar(x as u64),
                    goods::ice(val) => goods::ice(x as u64),
                    goods::cotton(val) => goods::cotton(x as u64),
                    goods::gold(val) => goods::gold(x as u64),
                    goods::iron(val) => goods::iron(x as u64),
                };
                newhold.push(temp);
            }
            if ((total < capacity || total == capacity) && remaining_money.is_positive()) {
                bad_hold_size = false;
                place.update(newhold.as_mut_slice(), true);
            } else if remaining_money.is_negative() {
                println!("Out of money error- not enough money");
            } else {
                println!("Hold does not have enough room, try again");
            }
        }
        (newhold, remaining_money as u64)
    }
    pub fn get_space(&self) -> usize {
        let mut x = match &self.sizeof {
            btype::large => btype::large as usize,
            btype::medium => btype::medium as usize,
            btype::small => btype::small as usize,
        };
        for i in &self.hold {
            x = x - i.getVal() as usize;
        }
        x
    }
}

impl fmt::Display for Sailor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n    Name: {} | {}", self.Name, self.Health)
    }
}
impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Health: {}", 100 - self.total())
    }
}
pub struct PrintCrew<'a> {
    pub x: &'a Vec<Sailor>,
}
impl<'a> fmt::Display for PrintCrew<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.x.len() {
            write!(f, "{}", self.x[i]);
        }
        write!(f, "")
    }
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "              |    |    |\n             )_)  )_)  )_)\n            )___))___))___)\\\n           )____)____)_____)\\\\\n         _____|____|____|____\\\\__\n---------\\                   /---------\n     ^^^^^ ^^^^^^^^^^^^^^^^^^^^^\n      ^^^^      ^^^^     ^^^    ^^\n           ^^^^      ^^^\nName: {:?}\nSize: {:?}\nCrew: {}\nHold: {:?}\nMoney: {:?}",self.name,self.sizeof, PrintCrew{x: &self.crew},self.hold,self.money)
    }
}

pub fn get_js() -> String {
    let mut toj: Ship = Ship {
        name: "Black Pearl".to_string(),
        sizeof: btype::large,
        hold: vec![
            goods::iron(1),
            goods::sugar(20),
            goods::gold(34),
            goods::cotton(20),
            goods::ice(0),
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

    let s = to_string(&toj);
    s.unwrap()
}
pub fn bootstraps(filename: String) -> Vec<Ship> {
    let mut places = vec![];
    let path = Path::new(&*filename);
    let display = path.display();
    let mut f = match File::open(&path) {
        Err(why) => panic!("No"),
        Ok(file) => file,
    };
    let mut s = String::new();
    let file = BufReader::new(&f);
    for (num, line) in file.lines().enumerate() {
        let l = line.unwrap();
        let line: String = l.chars().collect();
        places.push(from_str(&line).unwrap());
    }

    places
}

#[test]
fn serialize() {
    let mut places = bootstrap("test.txt".to_string());
    assert_eq!(3, places.len());
}
#[test]
fn get_space_ship() {
    let mut places = bootstrap("places.txt".to_string());
    let mut player: Ship = Ship {
        name: "Black Pearl".to_string(),
        sizeof: btype::large,
        hold: vec![
            goods::iron(1),
            goods::sugar(20),
            goods::gold(34),
            goods::cotton(20),
            goods::ice(0),
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
    player.sell_all(&mut places[0]);
    assert_eq!(100, player.get_space());
}
#[test]
fn health() {
    let Bob = Sailor {
        Name: "Bob".to_string(),
        Health: Health {
            Wounds: vec![wound::arm(10), wound::leg(25)],
        },
    };
    let x: u64 = Bob.Health.total();
    assert_eq!(35, x);
}

#[test]
fn npc_ship_move() {
    let mut ships = bootstraps("ships.txt".to_string());

    let mut places = bootstrap("places.txt".to_string());
    for i in 0..ships.len() {
        let xold = ships[i].visited[ships[i].visited.len() - 1];
        ships[i].npc_visit(&mut places);
        let x = ships[i].visited[ships[i].visited.len() - 1];
        assert_ne!(x, xold);
    }
}
#[test]
fn npc_ship_exist() {
    let mut ships = bootstraps("ships.txt".to_string());
    assert_ne!(ships.len(), 0);
}
#[test]
fn npc_ship_sell() {
    let mut ships = bootstraps("ships.txt".to_string());

    let mut places = bootstrap("places.txt".to_string());
    for i in 0..ships.len() {
        let x = ships[i].visited[ships[i].visited.len() - 1];
        ships[i].sell_all(&mut places[x]);
        assert_eq!(ships[i].hold[0].getVal(), 0);
    }
}
#[test]
fn npc_ship_buy() {
    let mut ships = bootstraps("ships.txt".to_string());

    let mut places = bootstrap("places.txt".to_string());
    for i in 0..ships.len() {
        let x = ships[i].visited[ships[i].visited.len() - 1];
        ships[i].sell_all(&mut places[x]);
        ships[i].npc_buy(&mut places[x]);
        let mut v = 0;
        for j in &ships[i].hold {
            v = v + j.getVal();
        }
        assert_eq!(100, v);
    }
}
