#![allow(non_snake_case)]
pub mod start_econ {
    use serde_json::to_string;
    use serde_json::*;
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::Path;

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ptype {
        large,
        meduim,
        small,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
    pub enum goods {
        sugar(u64),
        ice(u64),
        cotton(u64),
        gold(u64),
        iron(u64),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct port {
        pub name: String,
        pub sizeof: ptype,
        pub economy: Vec<goods>,
        pub route: Vec<usize>,
    }

    pub fn bootstrap(filename: String) -> Vec<port> {
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

    pub fn get_js() -> String {
        let mut toj: port = port {
            name: "Port Royal".to_string(),
            sizeof: ptype::large,
            economy: vec![goods::iron(1), goods::sugar(100), goods::cotton(50)],
            route: vec![0, 1],
        };
        let s = to_string(&toj);
        s.unwrap()
    }

    impl goods {
        fn same(&self, other: &goods) -> bool {
            let x = match &self {
                goods::sugar(u64) => 1,
                goods::ice(u64) => 2,
                goods::cotton(u64) => 3,
                goods::gold(u64) => 4,
                goods::iron(u64) => 5,
            };
            let y = match other {
                goods::sugar(u64) => 1,
                goods::ice(u64) => 2,
                goods::cotton(u64) => 3,
                goods::gold(u64) => 4,
                goods::iron(u64) => 5,
            };
            let mut ret = false;
            if x == y {
                ret = true;
            }
            ret
        }
        pub fn getVal(&self) -> u64 {
            match &self {
                goods::sugar(val) => val + 0,
                goods::ice(val) => val + 0,
                goods::cotton(val) => val + 0,
                goods::gold(val) => val + 0,
                goods::iron(val) => val + 0,
            }
        }
        //I hate myself for implimenting this but boy does it ever work
        pub fn swap(&self, tofill: &mut Vec<goods>, index: usize, vax: u64) {
            tofill.remove(index);

            let x = match &self {
                goods::sugar(val) => goods::sugar(vax),
                goods::ice(val) => goods::ice(vax),
                goods::cotton(val) => goods::cotton(vax),
                goods::gold(val) => goods::gold(vax),
                goods::iron(val) => goods::iron(vax),
            };
            tofill.push(x);
        }
    }

    impl port {
        pub fn update(&mut self, manifest: &mut [goods], buy: bool) -> u64 {
            let mut mcopy: Vec<goods> = vec![goods::sugar(1); manifest.len()];
            mcopy.clone_from_slice(manifest);

            for i in manifest {
                for j in 0..self.economy.len() - 1 {
                    if i.same(&self.economy[j]) {
                        println!(
                            "{:?}{:?}{:?}",
                            i,
                            &self.economy[j],
                            i.same(&self.economy[j])
                        );
                        let mut v;
                        if buy {
                            //inflation occurs because prices keep going up
                            v = &self.economy[j].getVal() + (i.getVal() / 5) + 1;
                            i.swap(&mut self.economy, j, v);
                        } else {
                            v = &self.economy[j].getVal() - (i.getVal() / 5);
                            if v < 0 || v == 0 {
                                v = 1;
                            }
                            i.swap(&mut self.economy, j, v);
                        }
                        println!(
                            "{:?}{:?}{:?}",
                            i,
                            &self.economy[&self.economy.len() - 1],
                            i.same(&self.economy[&self.economy.len() - 1])
                        );
                    }
                }
            }
            let mut total = 0;
            if buy {
                for i in mcopy {
                    total = total + i.getVal() * match i {
                        goods::sugar(val) => val + 0,
                        goods::ice(val) => val + 0,
                        goods::cotton(val) => val + 0,
                        goods::gold(val) => val + 0,
                        goods::iron(val) => val + 0,
                    };
                }
            } else {
                for i in mcopy {
                    total = total + i.getVal() * match i {
                        goods::sugar(val) => val + 0,
                        goods::ice(val) => val + 0,
                        goods::cotton(val) => val + 0,
                        goods::gold(val) => val + 0,
                        goods::iron(val) => val + 0,
                    };
                }
            }
            total
        }
        pub fn getmax(&self) -> goods {
            let mut max = goods::cotton(0);
            for i in &self.economy {
                let x = i.getVal();
                if x > max.getVal() {
                    max = i.clone();
                }
            }
            max
        }
        pub fn buy_menu(&self, capacity: usize) {
            let x = 0;
            for i in &place.economy {
                x = x + 1;
                println!("{}: {:?}", x, i);
            }
            for i in &place.economy {
                println!("How many {:?}", i);
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
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
