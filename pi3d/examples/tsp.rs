extern crate pi3d;
extern crate rand;
#[macro_use(s)]
extern crate ndarray as nd;

use rand::{thread_rng, Rng};
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use std::{collections::HashMap,
          cmp::Ordering,
          io::prelude::*,
          fs::File};

const POP_N: usize = 800;
const N_CITIES: usize = 80; // with cities from file this need to be mutated, i.e. use cities.len()
const W: f32 = 800.0;
const H: f32 = 800.0;

fn main() {
  let mut display = pi3d::display::create("TSP window", W, H, "GL", 2, 1).unwrap();
          display.set_background(&[0.2, 0.2, 0.6, 1.0]);
          display.set_target_fps(1000.0);
  let flatsh = pi3d::shader::Program::from_res(
        &display, "mat_flat").unwrap();
  let pointsh = pi3d::shader::Program::from_res(
        &display, "mat_pointsprite").unwrap();
  let textsh = pi3d::shader::Program::from_res(
        &display, "uv_pointsprite").unwrap();
  let mut camera = pi3d::camera::create(&display);
          camera.set_3d(false);

  let mut rng = thread_rng(); // one instance of random number gen passed as argument to methods
  //parsing text from file
  let mut cities: Vec<Point> = vec![];
  match File::open("cities/cities80.txt") { // NB needs to be in step with N_CITIES define above
    Ok(mut f) => {
          let mut buffer = String::new();
          match f.read_to_string(&mut buffer) {
                Ok(_n) => {
                      // i.e. "1.5, 2.75\n6.5, 5.75\n7.5, 8.75";
                      let b: Vec<&str> = buffer.lines().collect();
                      for i in b {
                        let pos: Vec<f32> = i.split(",").filter_map(|s| s.trim().parse().ok()).collect();
                        if pos.len() >= 2 {
                          cities.push(Point::new(pos[0], pos[1]));
                        }
                      }
                },
                Err(e) => {println!("{:?}", e);}
          }
    },
    Err(e) => {
      println!("{:?} - generate some random cities", e);
      for _i in 0..N_CITIES {
        cities.push(Point::new(rng.gen_range(-0.45 * W, 0.45 * W),
                               rng.gen_range(-0.45 * H, 0.45 * H)));
      }
      for c in cities.iter() {println!("Point::new({:.1}, {:.1}),", c.x, c.y);}
    }
  }

  let mut start_verts = Vec::<f32>::new();
  for i in 0..N_CITIES {
      start_verts.push(cities[i].x);
      start_verts.push(cities[i].y);
      start_verts.push(2.0);
  }

  let mut route = pi3d::shapes::lines::create(&start_verts, 4.0, true);
          route.set_shader(&flatsh);
          route.set_material(&[0.5, 0.1, 0.4]);

  let mut points = pi3d::shapes::points::create(&start_verts, 30.0);
          points.set_shader(&pointsh);
          points.buf[0].array_buffer.slice_mut(s![.., 2..7])
                                .assign(&nd::arr1(&[1.99, 1.0, 0.5, 0.0, 1.0]));
          points.buf[0].re_init();
          points.buf[0].unib[[0, 0]] = 1.0;

  let font = pi3d::util::font::create(&display, "fonts/NotoSans-Regular.ttf", "", "", 64.0);
  let mut labels = pi3d::shapes::point_text::create(&font, 600, 24.0);
          labels.set_shader(&textsh);
  for i in 0..N_CITIES {
    let blk = labels.add_text_block(&font, &[cities[i].x - 5.0, cities[i].y + 5.0, 0.0], 3, &format!("{}", i));
    labels.set_size(&font, blk, 0.6);
    labels.set_rgba(&font, blk, &[0.0, 1.0, 1.0, 1.0]);
  }

  let mut score = pi3d::shapes::point_text::create(&font, 16, 48.0);
          score.set_shader(&textsh);
  let score_blk = score.add_text_block(&font, &[-W * 0.5 + 10.0, H * 0.5 - 50.0, 0.0], 15, "----------");

  let mut population: Vec<Organism> = vec![];
  for _i in 0..POP_N {population.push(Organism::new(cities.len(), &mut rng));}
  let mut dist_table: HashMap<(usize, usize), f32> = HashMap::new();
  let mut best_organism = population[0].clone();
  let mut best_dist = best_organism.calc_length(&cities, &mut dist_table);
  let first_best = best_dist;
  let mut best_final = best_dist;
  let mut best_nine: Vec<Organism> = vec![];
  let mut last_improve = 0;
  let mut gen: usize = 0;
  let mut recalc = true;

    while display.loop_running() { // escape key and exit included by default
          route.draw(&mut camera);
          points.draw(&mut camera);
          labels.draw(&mut camera);
          score.draw(&mut camera);
          if recalc {
            gen += 1;
          //for gen in 0..5000 {
            let mut mating_pool = population.to_vec(); // clone vec
            for i in 0..POP_N { // recalc path lengths
              mating_pool[i].calc_length(&cities, &mut dist_table);
            }
            mating_pool.sort_unstable_by(Organism::partial_cmp); // shortest first
            if mating_pool[0].length < best_dist {
              best_organism = mating_pool[0].clone();
              best_dist = best_organism.length;
              last_improve = gen;
              for i in 0..N_CITIES {
                  route.buf[0].array_buffer[[i, 0]] = cities[best_organism.genes[i]].x;
                  route.buf[0].array_buffer[[i, 1]] = cities[best_organism.genes[i]].y;
              }
              route.buf[0].re_init();
              score.set_text(&font, score_blk, &format!("{:8.1}", best_dist));
            }

            for i in 3..15 { // mutate some of the good results shuffling 2 to 5 genes 
              mating_pool[i].mutate(i / 3 + 1, &mut rng)
            }

            if gen > (last_improve + 25) {
              if best_nine.len() < 12 {
                println!("stalled at {:?}", best_dist);
                best_nine.push(best_organism.clone());
                for i in 0..POP_N {
                  mating_pool[i].shuffle(&mut rng);
                }
                best_dist = first_best;
              } else {
                for i in 0..POP_N {
                  mating_pool[i] = best_nine[i % 12].clone(); // clone best genes over population
                  if i % 2 == 0 {
                    mating_pool[i].genes.reverse();
                  }
                }
                if best_dist == best_final {
                  println!("finished at {:.1}", best_dist);
                  println!("{:?}", best_organism.genes);
                  recalc = false; //stop recalculationg best route
                }
                best_final = best_dist;
                last_improve += 50; // check for final stall every 50 generations
              }
            }

            for i in 0..POP_N {
              //choose 2 organisms from mating pool using a distribution biased to start:
              let parent_a = &mating_pool[rng.gen_range(0, 5)];
              let parent_b = &mating_pool[rng.gen_range(0, POP_N - 1)];
              //reproduce:
              population[i] = parent_a.crossover(parent_b, &mut rng);
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////
struct Point {
  x: f32,
  y: f32,
}

impl Point {
  fn new(xi: f32, yi: f32) -> Point {
      Point {x: xi, y: yi}
  }
  ////////////////////////////
  fn dist(&self, p: &Point) -> f32 {
    let u = self.x - p.x;
    let v = self.y - p.y;
    (u * u + v * v).sqrt()
  }
}

////////////////////////////////////////////////////////////////////////
#[derive(Clone)] // need to be clonable for copying to mating_pool
struct Organism {
  genes: Vec<usize>,
  length: f32,
}

impl Organism {
  fn new(n_cities: usize, rng: &mut ThreadRng) -> Organism {
    let mut orgm = Organism {genes: (0..n_cities).collect(),
                             length: -1.0f32,};
    orgm.shuffle(rng);
    orgm
  }
  ////////////////////////////
  fn partial_cmp(first: &Organism, second: &Organism) -> Ordering {
    first.length.partial_cmp(&second.length).unwrap() // TODO cope with different results
  }
  ////////////////////////////
  fn shuffle(&mut self, rng: &mut ThreadRng) {
    self.genes.shuffle(rng);
    self.length = -1.0f32;
  }
  ////////////////////////////
  fn calc_length(&mut self, cities: &Vec<Point>, dist_table: &mut HashMap<(usize, usize), f32>) -> f32 {
    if self.length > 0.0f32 {
      return self.length;
    } else {
      self.length = 0.0f32;
      for i in 0..self.genes.len() {
        let c = self.genes[i]; // alias for tidyness below
        let cn = if i == self.genes.len() - 1 {  // wrap back to start
          self.genes[0] // cn is city next
        } else {
          self.genes[i + 1]
        };
        let d_key = if cn > c {
          (c, cn)
        } else {
          (cn, c)
        }; // reduce size of lookup table as a->b same as b->a
        self.length += match dist_table.get(&d_key) {
          Some(&dist) => dist,
          _ => { // not in table so add it
            let x = cities[c].dist(&cities[cn]);
            dist_table.insert(d_key, x);
            x // then return value
          }
        }
      }
    }
    self.length
  }
  ////////////////////////////
  fn crossover(&self, partner: &Organism, rng: &mut ThreadRng) -> Organism {
    //splice together their genes
    let n_cities = self.genes.len();
    let mut child = Organism::new(n_cities, rng);
    let index = rng.gen_range(0, n_cities - 1); //start of slice
    if rng.gen_range(0, 5) == 0 {
      let offset: i32 = n_cities as i32 + rng.gen_range(0, n_cities as i32 - 1); //shift 
      let inv = if rng.gen_range(0, 2) == 0 {-1} else {1};
      for i in 0..index {
        child.genes[i] = self.genes[((offset + i as i32 * inv) % n_cities as i32) as usize];
      }
    } else {
      for i in 0..index {
        child.genes[i] = self.genes[i];
      }
    }
    // add partner genes not in slice
    let mut k = index;
    for i in 0..partner.genes.len() {
      let mut flg = false; // flg if this number already in child.genes
      for j in 0..index {
        if partner.genes[i] == child.genes[j] {
          flg = true;
          break;
        }
      }
      if !flg {
        child.genes[k] = partner.genes[i];
        k += 1;
      }
    }
    child
  }
  ////////////////////////////
  fn mutate(&mut self, num: usize, rng: &mut ThreadRng) {
    let n = self.genes.len();
    if num < 2 || num >= n {println!("error"); return;}
    let mut indices: Vec<usize> = (0..n).collect();
    indices.shuffle(rng);
    for i in 0..num {
      let tmp: usize = self.genes[indices[i]];
      self.genes[indices[i]] = self.genes[indices[(i + 1) % num]];
      self.genes[indices[(i + 1) % num]] = tmp;
    }
    self.length = -1.0f32;
  }
}
