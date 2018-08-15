#![allow(non_snake_case)]
pub mod start_econ {
    use serde_json::to_string;
    use serde_json::*;
    use std::error::Error;
    use std::fmt;
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

    impl goods {
        pub fn same(&self, other: &goods) -> bool {
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
        pub fn setVal(&self, j: u64) -> goods {
            let mut single: goods = goods::sugar(100000000);
            single = match &self {
                goods::sugar(val) => goods::sugar(j as u64),
                goods::ice(val) => goods::ice(j as u64),
                goods::cotton(val) => goods::cotton(j as u64),
                goods::gold(val) => goods::gold(j as u64),
                goods::iron(val) => goods::iron(j as u64),
            };

            single
        }
        pub fn combine(&self, j: goods) -> goods {
            let mut single: goods = goods::sugar(100000000);
            single = match &self {
                goods::sugar(val) => goods::sugar(val + j.getVal()),
                goods::ice(val) => goods::ice(val + j.getVal()),
                goods::cotton(val) => goods::cotton(val + j.getVal()),
                goods::gold(val) => goods::gold(val + j.getVal()),
                goods::iron(val) => goods::iron(val + j.getVal()),
            };

            single
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
        //Basically an unsafe swap_remove() macro to replace an enum in a vec
        pub fn swap(&self, tofill: &mut Vec<goods>, index: usize, vax: u64) {
            let x = match &self {
                goods::sugar(val) => goods::sugar(vax),
                goods::ice(val) => goods::ice(vax),
                goods::cotton(val) => goods::cotton(vax),
                goods::gold(val) => goods::gold(vax),
                goods::iron(val) => goods::iron(vax),
            };
            tofill.push(x);
            tofill.swap_remove(index);
        }
    }

    impl port {
        pub fn update(&mut self, manifest: &mut [goods], buy: bool) -> u64 {
            let mut mcopy: Vec<goods> = vec![goods::sugar(1); manifest.len()];
            mcopy.clone_from_slice(manifest);

            let mut total = 0;
            for i in manifest {
                for j in 0..self.economy.len() {
                    if i.same(&self.economy[j]) {
                        let mut v: i64;
                        total = total + (i.getVal() * &self.economy[j].getVal());
                        if i.getVal() != 0 {
                            if buy {
                                //inflation occurs because prices keep going up
                                v = *(&self.economy[j].getVal()) as i64
                                    + ((i.getVal() / 5) + 1) as i64;
                                i.swap(&mut self.economy, j, v as u64);
                            } else {
                                v = *(&self.economy[j].getVal()) as i64 - (i.getVal() / 5) as i64;
                                if v < 0 || v == 0 {
                                    v = 1;
                                }
                                i.swap(&mut self.economy, j, v as u64);
                            }
                        }
                    }
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
    }

    impl fmt::Display for port {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Name: {} | Size: {:?} \n    Prices: {:?}",
                self.name, self.sizeof, self.economy
            )
        }
    }

    #[test]
    fn get_max_of_port() {
        let p = port {
            name: "a".to_string(),
            sizeof: ptype::large,
            economy: vec![goods::sugar(100), goods::ice(20)],
            route: vec![],
        };
        assert_eq!(100, p.getmax().getVal());
    }
    #[test]
    fn swap_goods() {
        let mut x = vec![goods::sugar(10)];
        goods::sugar(100).swap(&mut x, 0, 100);
        assert_eq!(x[0].getVal(), 100);
    }
    #[test]
    fn goods_setval() {
        let mut x = goods::sugar(10);
        x = x.setVal(100);
        assert_eq!(x.getVal(), 100);
    }
    #[test]
    fn goods_getval() {
        let mut x = goods::sugar(10);
        assert_eq!(x.getVal(), 10);
    }

    #[test]
    fn goods_combine() {
        let mut x = goods::sugar(10);
        x = x.combine(goods::sugar(5));
        assert_eq!(15, x.getVal());
    }

    #[test]
    fn goods_same() {
        let mut x = goods::sugar(10);
        let v = x.same(&goods::sugar(5));
        assert!(v);
    }
    #[test]
    fn update_alters_values_only_as_passed() {
        let mut p = port {
            name: "a".to_string(),
            sizeof: ptype::large,
            economy: vec![goods::sugar(100), goods::ice(20)],
            route: vec![],
        };
        let q = p.economy[0].getVal();
        let q2 = p.economy[1].getVal();
        p.update(vec![goods::sugar(100), goods::ice(0)].as_mut_slice(), true);
        assert_ne!(p.economy[0].getVal(), q);
        assert_eq!(p.economy[1].getVal(), q2);
    }
}
