#[derive(Debug)]
struct City {
  name: char,
  population: u64
}

fn sort_pop(cities:&mut Vec<City>) {
  cities.sort_by_key(sort_city_helper);
}

fn sort_city_helper(cty: &City) -> u64 {
  cty.population
}

fn sort_city_closer(cities:&mut Vec<City>) {
  cities.sort_by_key(|p| p.population);
}


fn main() {
  let a = City {name:'A', population: 1000};
  let b = City {name:'B', population: 102};
  let c = City {name:'C', population: 32};
  let d = City {name:'D', population: 342};
  let e = City {name:'E', population: 10};
  let g = City {name:'G', population: 100};
  let f = City {name:'F', population: 1060};

  let mut vec: Vec<City> = Vec::new();
  vec.push(c);
  vec.push(b);
  vec.push(d);
  vec.push(e);
  vec.push(g);
  vec.push(f);
  vec.push(a);

  sort_pop(&mut vec);
  println!("{:?}", vec);
}
