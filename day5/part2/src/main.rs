use std::sync::{Arc, Mutex};

use rayon::prelude::*;
fn build_map<'a>(iter: &mut impl Iterator<Item = &'a str>, map: &mut Vec<(u64, u64, u64)>) {
    iter.next();
    for line in iter.by_ref() {
        if line.is_empty() {
            break;
        }
        let triplets: Vec<u64> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        map.push((triplets[0], triplets[1], triplets[2]));
    }
}

fn get_pos(point: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
    for triplets in map {
        if point >= triplets.1 && point < triplets.1 + triplets.2 {
            return triplets.0 + (point - triplets.1);
        }
    }
    point
}
fn main() {
    let input = include_str!("input");
    let mut seed_to_soil = Vec::new();
    let mut soil_to_fertilizer = Vec::new();
    let mut fertilizer_to_water = Vec::new();
    let mut water_to_light = Vec::new();
    let mut light_to_temperature = Vec::new();
    let mut temperature_to_humidity = Vec::new();
    let mut humidity_to_location = Vec::new();

    let mut iter = input.lines();
    let line = iter.next().unwrap();
    let seeds: Vec<u64> = line[7..].split(' ').map(|x| x.parse().unwrap()).collect();

    //empty line
    iter.next();

    build_map(&mut iter, &mut seed_to_soil);

    build_map(&mut iter, &mut soil_to_fertilizer);
    build_map(&mut iter, &mut fertilizer_to_water);
    build_map(&mut iter, &mut water_to_light);
    build_map(&mut iter, &mut light_to_temperature);
    build_map(&mut iter, &mut temperature_to_humidity);
    build_map(&mut iter, &mut humidity_to_location);

    let calculated_locations = Arc::new(Mutex::new(u64::MAX));
    let mut seed_iter = seeds.iter();
    while let Some(start) = seed_iter.next() {
        if let Some(range) = seed_iter.next() {
            println!("looping");
            (*start..*start + *range).into_par_iter().for_each(|x| {
                let soil = get_pos(x, &seed_to_soil);
                let fertilizer = get_pos(soil, &soil_to_fertilizer);
                let water = get_pos(fertilizer, &fertilizer_to_water);
                let light = get_pos(water, &water_to_light);
                let temperature = get_pos(light, &light_to_temperature);
                let humidity = get_pos(temperature, &temperature_to_humidity);
                let location = get_pos(humidity, &humidity_to_location);
                if *calculated_locations.lock().unwrap() > location {
                    // probablistically speaking it can appear before program exit right? RIGHT?
                    println!("Maybe: {}", location);
                    *calculated_locations.lock().unwrap() = location;
                }
            });
        }
    }

    println!("Output: {}", calculated_locations.lock().unwrap());
}
