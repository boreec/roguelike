use crate::prelude::*;

/// Move mob actors to a random reachable position.
pub fn move_randomly(
    mut q_actors: Query<(&mut MapPosition, &Actor), With<OnDisplay>>,
    mut q_map: Query<&mut Map, With<OnDisplay>>,
) {
    let mut map = q_map.single_mut();

    for (mut pos_mob, actor) in q_actors.iter_mut() {
        if !actor.is_player() {
            let pos_reachable =
                enumerate_reachable_positions(&pos_mob.clone(), &map);

            if !pos_reachable.is_empty() {
                let pos_random = pos_reachable
                    [rand::thread_rng().gen_range(0..pos_reachable.len())];
                let tile_pos_old = map.as_tile_index(&pos_mob).unwrap();
                let tile_pos_new = map.as_tile_index(&pos_random).unwrap();
                map.tiles[tile_pos_new].actor =
                    map.tiles[tile_pos_old].actor.take();
                pos_mob.x = pos_random.x;
                pos_mob.y = pos_random.y;
            }
        }
    }
}

pub fn move_left(position: &mut MapPosition) {
    position.x -= 1;
}

pub fn move_right(position: &mut MapPosition) {
    position.x += 1;
}

pub fn move_up(position: &mut MapPosition) {
    position.y -= 1;
}

pub fn move_down(position: &mut MapPosition) {
    position.y += 1;
}

/// Returns a vector of reachable positions from a specific map position.
pub fn enumerate_reachable_positions(
    position: &MapPosition,
    map: &Map,
) -> Vec<MapPosition> {
    let mut reachable_positions: Vec<MapPosition> = vec![];

    if can_move_left(position, map) {
        reachable_positions.push(MapPosition {
            x: position.x - 1,
            y: position.y,
        });
    }
    if can_move_right(position, map) {
        reachable_positions.push(MapPosition {
            x: position.x + 1,
            y: position.y,
        });
    }
    if can_move_up(position, map) {
        reachable_positions.push(MapPosition {
            x: position.x,
            y: position.y - 1,
        });
    }
    if can_move_down(position, map) {
        reachable_positions.push(MapPosition {
            x: position.x,
            y: position.y + 1,
        });
    }
    return reachable_positions;
}

pub fn can_move_left(pos: &MapPosition, map: &Map) -> bool {
    if pos.x > 0 {
        map.tiles[pos.x + pos.y * map.width - 1].is_walkable()
    } else {
        false
    }
}

pub fn can_move_right(pos: &MapPosition, map: &Map) -> bool {
    if pos.x < map.width - 1 {
        map.tiles[pos.x + pos.y * map.width + 1].is_walkable()
    } else {
        false
    }
}

pub fn can_move_up(pos: &MapPosition, map: &Map) -> bool {
    if pos.y > 0 {
        map.tiles[pos.x + (pos.y - 1) * map.width].is_walkable()
    } else {
        false
    }
}

pub fn can_move_down(pos: &MapPosition, map: &Map) -> bool {
    if pos.y < map.height - 1 {
        map.tiles[pos.x + (pos.y + 1) * map.width].is_walkable()
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::*;

    fn create_plain_map() -> Map {
        Map {
            width: 3,
            height: 3,
            tiles: vec![Tile::default(); 3 * 3],
            exits: vec![],
        }
    }

    fn create_stone_map() -> Map {
        Map {
            width: 3,
            height: 3,
            tiles: vec![
                Tile::default(),
                Tile::from_kind(TileKind::GrassWithStone),
                Tile::default(),
                Tile::from_kind(TileKind::GrassWithStone),
                Tile::default(),
                Tile::from_kind(TileKind::GrassWithStone),
                Tile::default(),
                Tile::from_kind(TileKind::GrassWithStone),
                Tile::default(),
            ],
            exits: vec![],
        }
    }

    const POSITION_MIDDLE_LEFT: MapPosition = MapPosition { x: 0, y: 1 };
    const POSITION_MIDDLE: MapPosition = MapPosition { x: 1, y: 1 };
    const POSITION_MIDDLE_RIGHT: MapPosition = MapPosition { x: 2, y: 1 };
    const POSITION_TOP_LEFT: MapPosition = MapPosition { x: 0, y: 0 };
    const POSITION_TOP_MIDDLE: MapPosition = MapPosition { x: 1, y: 0 };
    const POSITION_TOP_RIGHT: MapPosition = MapPosition { x: 2, y: 0 };
    const POSITION_BOTTOM_LEFT: MapPosition = MapPosition { x: 0, y: 2 };
    const POSITION_BOTTOM_MIDDLE: MapPosition = MapPosition { x: 1, y: 2 };
    const POSITION_BOTTOM_RIGHT: MapPosition = MapPosition { x: 2, y: 2 };

    #[test]
    fn test_can_move_left_without_actors() {
        let map_plain = create_plain_map();
        assert!(!can_move_left(&POSITION_TOP_LEFT, &map_plain));
        assert!(!can_move_left(&POSITION_BOTTOM_LEFT, &map_plain));
        assert!(can_move_left(&POSITION_TOP_RIGHT, &map_plain));
        assert!(can_move_left(&POSITION_BOTTOM_RIGHT, &map_plain));
        assert!(can_move_left(&POSITION_MIDDLE, &map_plain));

        let map_stone = create_stone_map();
        assert!(!can_move_left(&POSITION_TOP_LEFT, &map_stone));
        assert!(!can_move_left(&POSITION_TOP_RIGHT, &map_stone));
        assert!(!can_move_left(&POSITION_BOTTOM_LEFT, &map_stone));
        assert!(!can_move_left(&POSITION_BOTTOM_RIGHT, &map_stone));
        assert!(!can_move_left(&POSITION_MIDDLE, &map_stone));
    }

    #[test]
    fn test_can_move_left_with_actors() {
        let map_plain = create_plain_map();

        assert!(!can_move_left(&POSITION_MIDDLE_RIGHT, &map_plain,));
        assert!(can_move_left(&POSITION_TOP_RIGHT, &map_plain,));
        assert!(can_move_left(&POSITION_BOTTOM_RIGHT, &map_plain,));
    }

    #[test]
    fn test_can_move_right_without_actors() {
        let map_plain = create_plain_map();
        assert!(!can_move_right(&POSITION_TOP_RIGHT, &map_plain));
        assert!(!can_move_right(&POSITION_BOTTOM_RIGHT, &map_plain));
        assert!(can_move_right(&POSITION_TOP_LEFT, &map_plain));
        assert!(can_move_right(&POSITION_BOTTOM_LEFT, &map_plain));
        assert!(can_move_right(&POSITION_MIDDLE, &map_plain));

        let map_stone = create_stone_map();
        assert!(!can_move_right(&POSITION_TOP_LEFT, &map_stone));
        assert!(!can_move_right(&POSITION_TOP_RIGHT, &map_stone));
        assert!(!can_move_right(&POSITION_BOTTOM_LEFT, &map_stone));
        assert!(!can_move_right(&POSITION_BOTTOM_RIGHT, &map_stone));
        assert!(!can_move_right(&POSITION_MIDDLE, &map_stone));
    }

    #[test]
    fn test_can_move_right_with_actors() {
        let map_plain = create_plain_map();

        assert!(!can_move_right(&POSITION_MIDDLE_LEFT, &map_plain,));
        assert!(can_move_right(&POSITION_TOP_LEFT, &map_plain,));
        assert!(can_move_right(&POSITION_BOTTOM_LEFT, &map_plain,));
    }

    #[test]
    fn test_can_move_up_without_actors() {
        let map_plain = create_plain_map();
        assert!(!can_move_up(&POSITION_TOP_LEFT, &map_plain));
        assert!(!can_move_up(&POSITION_TOP_RIGHT, &map_plain));
        assert!(can_move_up(&POSITION_BOTTOM_LEFT, &map_plain));
        assert!(can_move_up(&POSITION_BOTTOM_RIGHT, &map_plain));
        assert!(can_move_up(&POSITION_MIDDLE, &map_plain));

        let map_stone = create_stone_map();
        assert!(!can_move_up(&POSITION_TOP_LEFT, &map_stone));
        assert!(!can_move_up(&POSITION_TOP_RIGHT, &map_stone));
        assert!(!can_move_up(&POSITION_BOTTOM_LEFT, &map_stone));
        assert!(!can_move_up(&POSITION_BOTTOM_RIGHT, &map_stone));
        assert!(!can_move_up(&POSITION_MIDDLE, &map_stone));
    }

    #[test]
    fn test_can_move_up_with_actors() {
        let map_plain = create_plain_map();

        assert!(can_move_up(&POSITION_BOTTOM_RIGHT, &map_plain,));
        assert!(can_move_up(&POSITION_BOTTOM_LEFT, &map_plain,));
        assert!(!can_move_up(&POSITION_BOTTOM_MIDDLE, &map_plain,));
    }

    #[test]
    fn test_can_move_down_without_actors() {
        let map_plain = create_plain_map();
        assert!(!can_move_down(&POSITION_BOTTOM_LEFT, &map_plain));
        assert!(!can_move_down(&POSITION_BOTTOM_RIGHT, &map_plain));
        assert!(can_move_down(&POSITION_TOP_LEFT, &map_plain));
        assert!(can_move_down(&POSITION_TOP_RIGHT, &map_plain));
        assert!(can_move_down(&POSITION_MIDDLE, &map_plain));

        let map_stone = create_stone_map();
        assert!(!can_move_down(&POSITION_TOP_LEFT, &map_stone));
        assert!(!can_move_down(&POSITION_TOP_RIGHT, &map_stone));
        assert!(!can_move_down(&POSITION_BOTTOM_LEFT, &map_stone));
        assert!(!can_move_down(&POSITION_BOTTOM_RIGHT, &map_stone));
        assert!(!can_move_down(&POSITION_MIDDLE, &map_stone));
    }

    #[test]
    fn test_can_move_down_with_actors() {
        let map_plain = create_plain_map();

        assert!(can_move_down(&POSITION_TOP_RIGHT, &map_plain,));
        assert!(can_move_down(&POSITION_TOP_LEFT, &map_plain));
        assert!(!can_move_down(&POSITION_TOP_MIDDLE, &map_plain));
    }

    #[test]
    fn test_enumerate_reachable_positions() {
        let map_plain = create_plain_map();

        let reachable_positions =
            enumerate_reachable_positions(&POSITION_MIDDLE, &map_plain);

        assert_eq!(4, reachable_positions.len())
    }
}
