use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    f32::consts::SQRT_2,
};

use crate::{
    map::{
        tile::TileType,
        tile_map::{MapDimensions, MapTileGrid},
    },
    utils::{directional_deltas::ORTHOGONAL_DELTAS, map_cord::MapCord, map_utils},
};

struct Node {
    cord: MapCord,
    parent: MapCord,
    f: f32,
    g: f32,
}

impl Ord for Node {
    // sorts via min-heap rules
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.total_cmp(&self.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f.total_cmp(&other.f) == Ordering::Equal
    }
}

impl Eq for Node {}

pub enum PathResult {
    Success { path: Vec<MapCord> },
    TooLong,
    NoRoute,
}
pub struct PathFinder {
    open: BinaryHeap<Node>,
    closed: HashMap<MapCord, MapCord>,
    g_score: HashMap<MapCord, f32>,
}

impl PathFinder {
    pub fn new() -> Self {
        Self {
            open: BinaryHeap::with_capacity(10000),
            closed: HashMap::with_capacity(10000),
            g_score: HashMap::with_capacity(10000),
        }
    }

    pub fn a_star(
        &mut self,
        start: MapCord,
        goal: MapCord,
        grid: &MapTileGrid,
        max_check_tiles: usize,
        map_dimensions: MapDimensions,
    ) -> PathResult {
        self.open.clear();
        self.closed.clear();
        self.g_score.clear();

        self.g_score.insert(start, 0.0);
        let start_g = 0.0;
        let start_h = octile_dist(start, goal);
        let start_f = start_h + start_g;
        let parent = MapCord::new(i16::MAX, i16::MAX);

        self.open.push(Node {
            cord: start,
            parent,
            f: start_f,
            g: start_g,
        });

        while let Some(current) = self.open.pop() {
            if current.g > self.g_score[&current.cord] {
                continue;
            }

            self.closed.insert(current.cord, current.parent);

            if current.cord == goal {
                return PathResult::Success {
                    path: reconstruct_path(&self.closed, start, goal),
                };
            }

            for i in 0..ORTHOGONAL_DELTAS.len() {
                let check_tile = current.cord + ORTHOGONAL_DELTAS[i];

                if !map_utils::is_tile_in_bounds(map_dimensions, check_tile) {
                    continue;
                }

                if map_utils::get_tile_at_cord(&grid, map_dimensions, check_tile) != TileType::Grass
                {
                    continue;
                }

                if self.closed.contains_key(&check_tile) {
                    continue;
                }

                let tentative_g = if i % 2 == 0 {
                    current.g + 1.0
                } else {
                    current.g + SQRT_2
                };

                if self
                    .g_score
                    .get(&check_tile)
                    .is_none_or(|&existing_g| tentative_g < existing_g)
                {
                    self.g_score.insert(check_tile, tentative_g);

                    let f = tentative_g + octile_dist(check_tile, goal);

                    self.open.push(Node {
                        cord: check_tile,
                        parent: current.cord,
                        g: tentative_g,
                        f,
                    });
                }

                if self.closed.len() > max_check_tiles {
                    println!("Path too far away!");
                    return PathResult::TooLong;
                }
            }
        }
        return PathResult::NoRoute
    }
}

fn octile_dist(p1: MapCord, p2: MapCord) -> f32 {
    let abs_dist_x = f32::abs(p1.x as f32 - p2.x as f32);
    let abs_dist_y = f32::abs(p1.y as f32 - p2.y as f32);

    return f32::max(abs_dist_x, abs_dist_y) + (SQRT_2 - 1.0) * f32::min(abs_dist_x, abs_dist_y);
}

fn reconstruct_path(
    came_from: &HashMap<MapCord, MapCord>,
    start: MapCord,
    goal: MapCord,
) -> Vec<MapCord> {
    let mut path: Vec<MapCord> = Vec::new();
    let mut current = goal;

    while current != start {
        path.push(current);
        current = came_from[&current];
    }

    path.push(start);
    path.reverse();
    return path;
}
