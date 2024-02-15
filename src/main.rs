pub mod sphere_helper;
pub mod search_box;
pub mod kd_tree;
pub mod geo_point;

use std::f64::consts::PI;
use rand::distributions::{Distribution, Uniform};
use std::time::Instant;
use geo_point::Point;
use kd_tree::KDTree;

fn main() {

    // generate points
    let amount = 10_000 as usize;

    let lat_generator = Uniform::new(-PI / 2.0 + 0.2, PI / 2.0 - 0.2);
    let lon_generator = Uniform::new(-PI + 0.2, PI - 0.2);
    let mut rng = rand::thread_rng();

    let points: Vec::<Point> = (0..amount).into_iter().map(|idx| {
        Point{id: idx, lat: lat_generator.sample(&mut rng), lon: lon_generator.sample(&mut rng)}
    }).collect();

    // create a KDTree
    let radius = 1.0;
    let n_stop = 300 as usize;
    let kdtree = KDTree::new(&points, n_stop, radius);


    // check the speed !!!

    let mut tree_res = Vec::<f64>::new();
    let mut simple_res = Vec::<f64>::new();

    let n_const = 100;
    for _ in 0..n_const {
        let target = Point {
            id: amount + 1, 
            lat: lat_generator.sample(&mut rng), 
            lon: lon_generator.sample(&mut rng)
        };
        let distance = f64::abs(lon_generator.sample(&mut rng)) / 100.0;

        // search points in three
        let start_tree = Instant::now();
        let mut results = kdtree.search_by_distance(&target, distance);
        let duration = start_tree.elapsed().as_secs_f64();

        tree_res.push(duration);

        results.sort_by(|&p1, &p2| p1.id.cmp(&p2.id));

        // simple search
        let start_search = Instant::now();
        let simple_search: Vec::<Point> = points.clone().into_iter().filter(|&point| {
            sphere_helper::SphereHelper::distance(&target, &point, radius) <= distance
        }).collect();
        let duration = start_search.elapsed().as_secs_f64();

        simple_res.push(duration);

        std::iter::zip(results, simple_search).for_each(|(left, right)| {
            if left.id != right.id {
                println!("{:?}, {:?}, {:?}, {:?}", target, distance, left, right);
            }
        });
    }

    println!("Average time for tree = {:?}", tree_res.iter().sum::<f64>() / tree_res.len() as f64);
    println!("Average time for simple search = {:?}", simple_res.iter().sum::<f64>() / tree_res.len() as f64);

}