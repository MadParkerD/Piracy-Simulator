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

    #[derive(Debug, Serialize, Deserialize)]
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
        pub fn update(&mut self, manifest: &mut [goods], buy: bool) {
            for i in manifest {
                for j in 0..self.economy.len() - 1 {
                    if i.same(&self.economy[j]) {
                        println!(
                            "{:?}{:?}{:?}",
                            i,
                            &self.economy[j],
                            i.same(&self.economy[j])
                        );
                        let v;
                        if buy {
                            v = &self.economy[j].getVal() + (i.getVal() / 5);
                            i.swap(&mut self.economy, j, v);
                        } else {
                            v = &self.economy[j].getVal() - (i.getVal() / 5);
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
