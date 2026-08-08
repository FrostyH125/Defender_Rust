use std::{
    cmp::Ordering,
    collections::{BinaryHeap, VecDeque},
    f32::consts::SQRT_2,
};

use raylib::math::Vector2;

use crate::{
    map::{
        tile::TileType,
        tile_map::{MapDimensions, TileMap},
    },
    utils::{
        directional_deltas::ORTHOGONAL_DELTAS,
        map_cord::MapCord,
        map_utils::{self, cords_to_index, get_tile_at_cord, is_tile_in_bounds},
        vector2_utils,
    },
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
    NoPath,
    Success { path: VecDeque<Vector2> },
    TooLong,
    NoRoute,
}

/// generation: the current generation being used for data comparisons
/// open: the min-heap sorting the nodes by f value, as to get the cheapest first
/// parents: a vector of mapcords, indexed by tile index, returning the map cord that this tile considers the previous one for itself
/// g_score: a vector of g_score values indexed by tile index
/// g_score_generation: used to determine whether or not a g_score value is applicable to the current search or not using '==', indexed by tile index
pub struct PathFinder {
    generation: u32,

    open: BinaryHeap<Node>,

    parents: Vec<Option<MapCord>>,

    // if you didnt know, g_score is basically just how many tiles it took to get to this tile
    // each tile in this algorithm costs 1.0g to traverse and the distance algo calulates
    // the h (heuristic) value (which is basically just a lowball estimate on how far you
    // have to travel to get to the goal) which gets added to the g value for the final
    // f value per node. the nodes are sorted by f value in the min heap (smallest first)
    // in order to continuously pop the cheapest tile, check if the tile is the goal,
    // check if the g value is a genuine improvement or not, and add its neighbors when applicable to the min heap
    g_score: Vec<f32>,
    g_score_generation: Vec<u32>,
}

impl PathFinder {
    pub fn new(map_width: u16, map_height: u16) -> Self {
        let num_of_tiles = map_width as usize * map_height as usize;

        Self {
            generation: 0,
            open: BinaryHeap::with_capacity(10000),

            parents: vec![None; num_of_tiles],

            g_score: vec![f32::INFINITY; num_of_tiles],
            g_score_generation: vec![0; num_of_tiles],
        }
    }

    /// a, dare i say, optimized a* algorithm
    /// one caveat, this algorithm will just happen to a return a
    /// PathResult::NoRoute if the goal is technically within
    /// the max distance but the path youll have to travel is
    /// further than the max distance to get there, such as in having to
    /// go around something large, for example. I pondered if this was the
    /// correct choice but ultimately it't not an incorrect description of
    /// what happened. The path does not, in fact, have an available
    /// route to it, even though the path is still technically open for
    /// traversal.
    pub fn a_star(
        &mut self,
        start: MapCord,
        goal: MapCord,
        tile_map: &TileMap,
        max_radius_for_path: f32,
    ) -> PathResult {
        // base case handling
        if goal.dist_to(start) >= max_radius_for_path {
            println!("current pathfinding goal too far away");
            return PathResult::TooLong;
        }

        if !is_tile_in_bounds(tile_map.map_dimensions, goal) {
            println!("current pathfinding goal out of bounds");
            return PathResult::NoRoute;
        }

        if !is_tile_in_bounds(tile_map.map_dimensions, start) {
            println!("current pathfinding start out of bounds");
            return PathResult::NoRoute;
        }

        if get_tile_at_cord(&tile_map.map_tile_grid, tile_map.map_dimensions, goal)
            != TileType::Grass
        {
            println!("current pathfinding goal is not grass");
            return PathResult::NoRoute;
        }

        if get_tile_at_cord(&tile_map.map_tile_grid, tile_map.map_dimensions, start)
            != TileType::Grass
        {
            println!("current pathfinding start is not grass");
            return PathResult::NoRoute;
        }

        // handle overflow on generation
        if self.generation == u32::MAX {
            self.generation = 0;
            self.g_score_generation.fill(0);
            self.parents.fill(None);
        }

        // generation counters make the closed and g-score arrays not needing to be upkept
        // on each a_star algorithm run, so only the priority queue needs to be manually cleared.
        // This avoids clearing/filling several large collections each run
        self.generation += 1;
        self.open.clear();

        let start_index = cords_to_index(tile_map.map_dimensions, start);

        // set g
        let start_g = 0.0;
        self.g_score[start_index] = start_g;
        self.g_score_generation[start_index] = self.generation;

        // set the starting node up,
        // this parent will be in the first node, and at the time of reconstruction,
        // the starting node is never actually added to the path.
        // this is on purpose, since the implementation of move_to() using
        // this a_star algorithm will actually insert a final position before running, which is the
        // target pos Vector2, so it'll still go to the target
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
            let current_index = cords_to_index(tile_map.map_dimensions, current.cord);

            // current g value for the current tile is higher cost than whats already there,
            // scrap it
            if current.g > self.g_score[current_index] {
                continue;
            }

            self.parents[current_index] = Some(current.parent);

            // goal found, go home
            if current.cord == goal {
                return PathResult::Success {
                    path: reconstruct_path(&self.parents, tile_map.map_dimensions, start, goal),
                };
            }

            // check all neighbors in 8 directions, this is where tiles get added to open if applicable
            for i in 0..ORTHOGONAL_DELTAS.len() {
                let check_tile = current.cord + ORTHOGONAL_DELTAS[i];

                // tile is too far away, don't add it
                if check_tile.dist_to(start) > max_radius_for_path {
                    continue;
                }

                if !map_utils::is_tile_in_bounds(tile_map.map_dimensions, check_tile) {
                    continue;
                }

                if map_utils::get_tile_at_cord(
                    &tile_map.map_tile_grid,
                    tile_map.map_dimensions,
                    check_tile,
                ) != TileType::Grass
                {
                    continue;
                }

                let check_index = cords_to_index(tile_map.map_dimensions, check_tile);

                // see what the g_score would be if you went with this tile
                let tentative_g = if i % 2 == 0 {
                    current.g + 1.0
                } else {
                    current.g + SQRT_2
                };

                // if a g score already exists for this tile, use that for the comparison, otherwise,
                // let this win because g_score hasnt been assigned for this tile yet
                let existing_g = if self.g_score_generation[check_index] == self.generation {
                    self.g_score[check_index]
                } else {
                    f32::INFINITY
                };

                // if the g thats already there (or isnt there) is greater than the g_score thats tentatively being tried
                // originally i had a closed array as well, but i realized eventually that it wasnt necessary, and id
                // rather save the memory usage (4b * total tiles) and just do slightly more computations for neighbor checking
                // originally it would continue early right before the tentative_g calculation if the tile was marked closed
                // already. this comparison legitimately guarantees that only improved path tiles are added
                if existing_g > tentative_g {
                    // set the g_score for this tile to tentative_g, because a lower score means its a cheaper cost
                    self.g_score[check_index] = tentative_g;
                    self.g_score_generation[check_index] = self.generation;

                    // calculate the final f value so this tile can be sorted appropriately for
                    // efficient check
                    let f = tentative_g + octile_dist(check_tile, goal);

                    // finally, push the node to be properly evaluated by the first half of this algorithm
                    self.open.push(Node {
                        cord: check_tile,
                        parent: current.cord,
                        g: tentative_g,
                        f,
                    });
                }
            }
        }

        // if you run out of open, that means that all available tiles within the
        // constraints were explored, and the goal was never found
        return PathResult::NoRoute;
    }
}

fn reconstruct_path(
    parents: &[Option<MapCord>],
    map_dimensions: MapDimensions,
    start: MapCord,
    goal: MapCord,
) -> VecDeque<Vector2> {
    let mut path: VecDeque<Vector2> = VecDeque::new();
    let mut current = goal;

    while current != start {
        path.push_front(vector2_utils::cord_to_v2(current));

        let index = cords_to_index(map_dimensions, current);

        current = parents[index].unwrap();
    }

    return path;
}

fn octile_dist(p1: MapCord, p2: MapCord) -> f32 {
    let dx = (p1.x - p2.x).abs() as f32;
    let dy = (p1.y - p2.y).abs() as f32;

    return f32::max(dx, dy) + (SQRT_2 - 1.0) * f32::min(dx, dy);
}

fn manhattan_dist(p1: MapCord, p2: MapCord) -> f32 {
    let dx = (p1.x - p2.x).abs() as f32;
    let dy = (p1.y - p2.y).abs() as f32;

    dx + dy
}

fn pythagorean_dist(p1: MapCord, p2: MapCord) -> f32 {
    let dx = (p2.x - p1.x) as f32;
    let dy = (p2.y - p1.y) as f32;

    (dx * dx + dy * dy).sqrt()
}

fn chebyshev_dist(p1: MapCord, p2: MapCord) -> f32 {
    let dx = (p1.x - p2.x).abs() as f32;
    let dy = (p1.y - p2.y).abs() as f32;

    dx.max(dy)
}
