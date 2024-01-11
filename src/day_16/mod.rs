use std::collections::HashSet;

use aoc_2023_rust_flupke::Problem;

use crate::common::{
    array::{parse_char, Array},
    vector::Vector,
};

type HereVector = Vector<i16>;

pub struct Day16;

const LEFT: HereVector = HereVector { x: -1, y: 0 };
const RIGHT: HereVector = HereVector { x: 1, y: 0 };
const UP: HereVector = HereVector { x: 0, y: -1 };
const DOWN: HereVector = HereVector { x: 0, y: 1 };

#[derive(Debug, Default, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
struct Ray {
    origin: HereVector,
    direction: HereVector,
}

impl Ray {
    #[allow(dead_code)]
    fn from_direction(direction: HereVector) -> Self {
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

    fn orient(&self, direction: HereVector) -> Self {
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

    fn add_photon(&mut self, at: HereVector) {
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
            tiles: parse_char(input),
        }
    }

    fn optimize(&self) -> usize {
        let mut best_score = 0_usize;

        for x in 0..self.tiles.width {
            best_score = best_score.max(
                self.trace(Ray {
                    direction: DOWN,
                    origin: HereVector { x: x as i16, y: 0 },
                })
                .score(),
            );
            best_score = best_score.max(
                self.trace(Ray {
                    direction: UP,
                    origin: HereVector {
                        x: x as i16,
                        y: self.tiles.height as i16 - 1,
                    },
                })
                .score(),
            );
        }

        for y in 0..self.tiles.height {
            best_score = best_score.max(
                self.trace(Ray {
                    direction: RIGHT,
                    origin: HereVector { x: 0, y: y as i16 },
                })
                .score(),
            );
            best_score = best_score.max(
                self.trace(Ray {
                    direction: LEFT,
                    origin: HereVector {
                        x: self.tiles.width as i16 - 1,
                        y: y as i16,
                    },
                })
                .score(),
            );
        }

        best_score
    }

    fn trace(&self, ray: Ray) -> PhotonMap {
        let mut ray = ray;
        let mut photon_map = PhotonMap::new(self.tiles.width, self.tiles.height);
        let mut ray_stack = Vec::new();
        let mut seen = HashSet::new();
        loop {
            if ray.origin.x < 0
                || ray.origin.x as usize >= self.tiles.width
                || ray.origin.y < 0
                || ray.origin.y as usize >= self.tiles.height
                || seen.contains(&ray)
            {
                if ray_stack.is_empty() {
                    break;
                }
                ray = ray_stack.pop().unwrap();
            } else {
                let tile = *self.tiles.get(ray.origin);
                photon_map.add_photon(ray.origin);
                seen.insert(ray.clone());

                if tile == '.' {
                    ray = ray.walk();
                } else if tile == '/' || tile == '\\' {
                    ray = reflect(ray, tile)
                } else if tile == '|' || tile == '-' {
                    let refracted = refract(&ray, tile);
                    ray = refracted[0].clone();
                    if refracted.len() == 2 {
                        ray_stack.push(refracted[1].clone());
                    }
                }
            }
        }
        photon_map
    }

    #[allow(dead_code)]
    fn print_debug(&self, ray: &Ray) {
        let mut data = Vec::new();
        for coords in self.tiles.iter_vec_coords() {
            let coords: HereVector = coords.into();
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
    map.optimize()
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

    #[ignore]
    #[test]
    fn test_part_two() {
        assert_eq!(solve(include_str!("example.txt")), 51);
        assert_eq!(solve(include_str!("input.txt")), 8163);
    }
}
