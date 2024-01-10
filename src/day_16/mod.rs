use std::collections::HashSet;

use aoc_2023_rust_flupke::Problem;

use crate::common::{
    array::{parse, Array},
    vector::Vector,
};

pub struct Day16;

const LEFT: Vector = Vector { x: -1, y: 0 };
const RIGHT: Vector = Vector { x: 1, y: 0 };
const UP: Vector = Vector { x: 0, y: -1 };
const DOWN: Vector = Vector { x: 0, y: 1 };

#[derive(Debug, Default, Eq, PartialEq, Clone, Hash)]
struct Ray {
    origin: Vector,
    direction: Vector,
}

impl Ray {
    #[allow(dead_code)]
    fn from_direction(direction: Vector) -> Self {
        Self {
            direction,
            ..Self::default()
        }
    }

    fn walk(&self) -> Self {
        Self {
            origin: self.origin + self.direction,
            ..*self
        }
    }

    fn orient(&self, direction: Vector) -> Self {
        Self { direction, ..*self }
    }

    fn format_direction(&self) -> char {
        match self.direction {
            UP => '^',
            DOWN => 'v',
            LEFT => '<',
            RIGHT => '>',
            _ => panic!("invalid direction {:?}", self.direction),
        }
    }
}

pub struct MirrorMap {
    tiles: Array<char>,
}

struct PhotonMap {
    photons: Array<usize>,
}

impl PhotonMap {
    fn new(width: usize, height: usize) -> Self {
        Self {
            photons: Array::new(width, height),
        }
    }

    fn add_photon(&mut self, at: Vector) {
        self.photons.set(at, self.photons.get(at) + 1);
    }

    fn score(&self) -> usize {
        self.photons
            .data
            .iter()
            .filter(|&&photons| photons > 0)
            .count()
    }

    #[allow(dead_code)]
    fn format(&self) -> String {
        Array::from_data(
            self.photons
                .data
                .iter()
                .map(|p| if *p > 0 { '#' } else { '.' })
                .collect(),
            self.photons.width,
            self.photons.height,
        )
        .format()
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("{}", self.format());
    }
}

#[allow(dead_code)]
fn wait_for_keypress() {
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");
}

impl MirrorMap {
    fn from_str(input: &str) -> Self {
        MirrorMap {
            tiles: parse(input),
        }
    }

    fn trace(&self) -> PhotonMap {
        let mut photon_map = PhotonMap::new(self.tiles.width, self.tiles.height);
        let ray = Ray {
            origin: Vector { x: 0, y: 0 },
            direction: Vector { x: 1, y: 0 },
        };
        self.do_trace(ray, &mut photon_map, &mut HashSet::new());
        photon_map
    }

    fn do_trace(&self, ray: Ray, photon_map: &mut PhotonMap, seen: &mut HashSet<Ray>) {
        let mut ray = ray;
        loop {
            if ray.origin.x < 0
                || ray.origin.x as usize >= self.tiles.width
                || ray.origin.y < 0
                || ray.origin.y as usize >= self.tiles.height
                || seen.contains(&ray)
            {
                break;
            }
            let tile = *self.tiles.get(ray.origin);
            photon_map.add_photon(ray.origin);
            seen.insert(ray.clone());

            // self.print_debug(&ray);
            // wait_for_keypress();

            if tile == '.' {
                ray = ray.walk();
            } else if tile == '/' || tile == '\\' {
                ray = reflect(ray, tile)
            } else if tile == '|' || tile == '-' {
                let refracted = refract(&ray, tile);
                ray = refracted[0].clone();
                if refracted.len() == 2 {
                    self.do_trace(refracted[1].clone(), photon_map, seen);
                }
            }
        }
    }

    #[allow(dead_code)]
    fn print_debug(&self, ray: &Ray) {
        let mut data = Vec::new();
        for coords in self.tiles.iter_vec_coords() {
            let tile = if coords == ray.origin {
                ray.format_direction()
            } else {
                *self.tiles.get(coords)
            };
            data.push(tile);
        }
        Array::from_data(data, self.tiles.width, self.tiles.height).print();
    }
}

fn reflect(ray: Ray, tile: char) -> Ray {
    match ray.direction {
        LEFT | RIGHT => ray.orient(if tile == '/' {
            ray.direction.rotate_counterclockwise()
        } else {
            ray.direction.rotate_clockwise()
        }),
        UP | DOWN => ray.orient(if tile == '/' {
            ray.direction.rotate_clockwise()
        } else {
            ray.direction.rotate_counterclockwise()
        }),
        _ => panic!("invalid ray direction {:?}", ray.direction),
    }
    .walk()
}

fn refract(ray: &Ray, tile: char) -> Vec<Ray> {
    match ray.direction {
        LEFT | RIGHT => {
            if tile == '|' {
                vec![ray.orient(UP).walk(), ray.orient(DOWN).walk()]
            } else {
                vec![ray.walk()]
            }
        }
        UP | DOWN => {
            if tile == '-' {
                vec![ray.orient(LEFT).walk(), ray.orient(RIGHT).walk()]
            } else {
                vec![ray.walk()]
            }
        }
        _ => panic!("invalid ray direction {:?}", ray.direction),
    }
}

fn solve(input: &str) -> usize {
    let map = MirrorMap::from_str(input);
    let photon_map = map.trace();
    photon_map.score()
}

impl Problem for Day16 {
    fn check(&self) {
        println!("{}", solve(include_str!("example.txt")));
    }

    fn solve(&self) {
        println!("{}", solve(include_str!("input.txt")));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflect() {
        assert_eq!(
            reflect(Ray::from_direction(RIGHT), '/'),
            Ray::from_direction(UP).walk()
        );
        assert_eq!(
            reflect(Ray::from_direction(LEFT), '/'),
            Ray::from_direction(DOWN).walk()
        );
        assert_eq!(
            reflect(Ray::from_direction(UP), '/'),
            Ray::from_direction(RIGHT).walk()
        );
        assert_eq!(
            reflect(Ray::from_direction(DOWN), '/'),
            Ray::from_direction(LEFT).walk()
        );

        assert_eq!(
            reflect(Ray::from_direction(RIGHT), '\\'),
            Ray::from_direction(DOWN).walk()
        );
        assert_eq!(
            reflect(Ray::from_direction(LEFT), '\\'),
            Ray::from_direction(UP).walk()
        );
        assert_eq!(
            reflect(Ray::from_direction(UP), '\\'),
            Ray::from_direction(LEFT).walk()
        );
        assert_eq!(
            reflect(Ray::from_direction(DOWN), '\\'),
            Ray::from_direction(RIGHT).walk()
        );
    }

    #[test]
    fn test_refract() {
        assert_eq!(
            refract(&Ray::from_direction(RIGHT), '|'),
            vec![
                Ray::from_direction(UP).walk(),
                Ray::from_direction(DOWN).walk()
            ]
        );
        assert_eq!(
            refract(&Ray::from_direction(LEFT), '|'),
            vec![
                Ray::from_direction(UP).walk(),
                Ray::from_direction(DOWN).walk()
            ]
        );
        assert_eq!(
            refract(&Ray::from_direction(UP), '-'),
            vec![
                Ray::from_direction(LEFT).walk(),
                Ray::from_direction(RIGHT).walk()
            ]
        );
        assert_eq!(
            refract(&Ray::from_direction(DOWN), '-'),
            vec![
                Ray::from_direction(LEFT).walk(),
                Ray::from_direction(RIGHT).walk()
            ]
        );

        assert_eq!(
            refract(&Ray::from_direction(RIGHT), '-'),
            vec![Ray::from_direction(RIGHT).walk()]
        );
        assert_eq!(
            refract(&Ray::from_direction(LEFT), '-'),
            vec![Ray::from_direction(LEFT).walk()]
        );
        assert_eq!(
            refract(&Ray::from_direction(UP), '|'),
            vec![Ray::from_direction(UP).walk()]
        );
        assert_eq!(
            refract(&Ray::from_direction(DOWN), '|'),
            vec![Ray::from_direction(DOWN).walk()]
        );
    }
}
