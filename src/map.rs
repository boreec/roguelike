use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn generate_random_spawning_position(&self) -> MapPosition {
        let spawnable_positions: Vec<_> = self
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.is_walkable())
            .map(|(index, _)| index)
            .collect();

        if spawnable_positions.is_empty() {
            panic!("There are no spawnable positions");
        }

        let mut rng = rand::thread_rng();
        let index = *spawnable_positions.choose(&mut rng).unwrap();

        MapPosition::new(index % self.width, index / self.height)
    }

    // collect von neumann neighbours of a position
    pub fn collect_neighbours_from(&self, p: &MapPosition) -> Vec<MapPosition> {
        let mut neighbours = Vec::new();
        // left neighbour
        if p.x > 0 {
            neighbours.push(MapPosition::new(p.x - 1, p.y));
        }
        // right neighbour
        if p.x < self.width - 1 {
            neighbours.push(MapPosition::new(p.x + 1, p.y));
        }
        // top neighbour
        if p.y > 0 {
            neighbours.push(MapPosition::new(p.x, p.y - 1));
        }
        // bottom neighbour
        if p.y < self.height - 1 {
            neighbours.push(MapPosition::new(p.x, p.y + 1));
        }
        // top right neighbour
        if p.x < self.width - 1 && p.y > 0 {
            neighbours.push(MapPosition::new(p.x + 1, p.y - 1));
        }
        // top left neighbour
        if p.x > 0 && p.y > 0 {
            neighbours.push(MapPosition::new(p.x - 1, p.y - 1));
        }
        // bottom right neighbour
        if p.x < self.width - 1 && p.y < self.height - 1 {
            neighbours.push(MapPosition::new(p.x + 1, p.y + 1));
        }
        // bottom left neighbour
        if p.x > 0 && p.y < self.height - 1 {
            neighbours.push(MapPosition::new(p.x - 1, p.y + 1));
        }

        neighbours
    }
}

#[derive(Component, Debug)]
pub struct MapPosition {
    pub x: usize,
    pub y: usize,
}

impl MapPosition {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_plain_map(width: usize, height: usize) -> Map {
        Map {
            width,
            height,
            tiles: vec![TileType::Grass; width * height],
        }
    }

    #[test]
    fn test_collect_neighbours() {
        let map1x1 = create_plain_map(1, 1);
        let map1x2 = create_plain_map(1, 2);
        let map3x3 = create_plain_map(3, 3);
        let pos00 = MapPosition::new(0, 0);
        let pos01 = MapPosition::new(0, 1);
        let pos02 = MapPosition::new(0, 2);
        let pos10 = MapPosition::new(1, 0);
        let pos11 = MapPosition::new(1, 1);
        let pos12 = MapPosition::new(1, 2);
        let pos20 = MapPosition::new(2, 0);
        let pos21 = MapPosition::new(2, 1);
        let pos22 = MapPosition::new(2, 2);

        let pos00_neighbours = map3x3.collect_neighbours_from(&pos00);
        let pos01_neighbours = map3x3.collect_neighbours_from(&pos01);
        let pos02_neighbours = map3x3.collect_neighbours_from(&pos02);
        let pos10_neighbours = map3x3.collect_neighbours_from(&pos10);
        let pos11_neighbours = map3x3.collect_neighbours_from(&pos11);
        let pos12_neighbours = map3x3.collect_neighbours_from(&pos12);
        let pos20_neighbours = map3x3.collect_neighbours_from(&pos20);
        let pos21_neighbours = map3x3.collect_neighbours_from(&pos21);
        let pos22_neighbours = map3x3.collect_neighbours_from(&pos22);

        assert_eq!(pos00_neighbours.len(), 3);
        assert_eq!(pos01_neighbours.len(), 5);
        assert_eq!(pos02_neighbours.len(), 3);
        assert_eq!(pos10_neighbours.len(), 5);
        assert_eq!(pos11_neighbours.len(), 8);
        assert_eq!(pos12_neighbours.len(), 5);
        assert_eq!(pos20_neighbours.len(), 3);
        assert_eq!(pos21_neighbours.len(), 5);
        assert_eq!(pos22_neighbours.len(), 3);

        let pos00_neighbours = map1x2.collect_neighbours_from(&pos00);
        let pos01_neighbours = map1x2.collect_neighbours_from(&pos01);
        assert_eq!(pos00_neighbours.len(), 1);
        assert_eq!(pos01_neighbours.len(), 1);

        let pos00_neighbours = map1x1.collect_neighbours_from(&pos00);
        assert_eq!(pos00_neighbours.len(), 0);
    }
}
