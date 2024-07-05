// bruteforce
// 20/02/24
// Neil Crago <n.j.crago@gmail.com>
//
// A program to explore Combinatorics using a Brute Force Algorithm
// to solve the TSP (Travelling Salesman Problem)
//

use std::f64;

// This struct represents a city with its coordinates
#[derive(Debug)]
struct City {
    x: f64,
    y: f64,
}

// Function to calculate the distance between two cities
fn distance(city1: &City, city2: &City) -> f64 {
    let (x1, y1) = (city1.x, city1.y);
    let (x2, y2) = (city2.x, city2.y);
    f64::sqrt((x2 - x1).powi(2) + (y2 - y1).powi(2))
}

// This function calculates the total distance of a given tour
fn tour_length(cities: &[City], order: &[usize]) -> f64 {
    
    let mut total_distance = 0.0;
    
    for i in 0..cities.len() - 1 {
        let current_city = &cities[order[i]];
        let next_city = &cities[order[i + 1]];
        total_distance += distance(current_city, next_city);
    }
    
    // Add distance from last city back to first city to complete the loop
    total_distance += distance(&cities[order[cities.len() - 1]], &cities[order[0]]);
    total_distance
}

// This function implements a simple brute-force approach to find the shortest tour
fn brute_force_tsp(cities: &[City]) -> (f64, Vec<usize>) {
    
    let mut shortest_tour_length = f64::INFINITY;
    let mut shortest_tour: Vec<usize> = Vec::new();

    // Generate all possible permutations of city indexes
    let permutations = permutations(cities.len());

    // Iterate through all permutations and find the shortest tour
    for permutation in permutations {
        let tour_length = tour_length(cities, &permutation);
        if tour_length < shortest_tour_length {
            shortest_tour_length = tour_length;
            shortest_tour = permutation.clone();
        }
    }

    (shortest_tour_length, shortest_tour)
}

// Helper function to generate all permutations of a list (recursive)
fn permutations(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut permutations_vec = Vec::new();
    let sub_perms = permutations(n - 1);
    
    for i in 0..n {
        for perm in &sub_perms {
            let mut new_perm = perm.clone();
            new_perm.insert(i, n - 1);
            permutations_vec.push(new_perm);
        }
    }
    permutations_vec
}


fn main() {
    
    // Sample set of city coordinates
    let cities = vec![
        City { x: 1.0, y: 2.0 },
        City { x: 7.0, y: 11.0 },
        City { x: 3.0, y: 23.0 },
        City { x: 2.0, y: 80.0 },
        City { x: 9.0, y: 2.0 },
        City { x: 19.0, y: 11.0 },
        City { x: 29.0, y: 23.0 },
        City { x: 1.0, y: 80.0 },
    ];

    // Find the shortest tour using brute force
    let (shortest_distance, tour) = brute_force_tsp(&cities);

    println!("Shortest tour distance: {}", shortest_distance);
    println!("Tour order: {:?}", tour);
}
