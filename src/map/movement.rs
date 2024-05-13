use crate::prelude::*;

/// Move mob actors to a random reachable position.
pub fn move_randomly(
    mut query_actors: Query<(&mut MapPosition, &Actor)>,
    query_map: Query<&Map>,
    current_map_number: Res<CurrentMapNumber>,
) {
    let pos_occupied: Vec<MapPosition> = query_actors
        .iter_mut()
        .filter(|(_, a)| a.map_number == current_map_number.0)
        .map(|(p, _)| *p)
        .collect();

    let map = query_map
        .iter()
        .filter(|m| m.number == current_map_number.0)
        .last()
        .expect("no map found");

    for (mut pos_mob, actor) in query_actors.iter_mut() {
        if actor.map_number == current_map_number.0 && !actor.is_player() {
            let pos_reachable = enumerate_reachable_positions(
                &pos_mob.clone(),
                map,
                &pos_occupied,
            );

            if !pos_reachable.is_empty() {
                let pos_random = pos_reachable
                    [rand::thread_rng().gen_range(0..pos_reachable.len())];
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
    occupied_positions: &Vec<MapPosition>,
) -> Vec<MapPosition> {
    let mut reachable_positions: Vec<MapPosition> = vec![];

    if can_move_left(position, map, occupied_positions) {
        reachable_positions.push(MapPosition {
            x: position.x - 1,
            y: position.y,
        });
    }
    if can_move_right(position, map, occupied_positions) {
        reachable_positions.push(MapPosition {
            x: position.x + 1,
            y: position.y,
        });
    }
    if can_move_up(position, map, occupied_positions) {
        reachable_positions.push(MapPosition {
            x: position.x,
            y: position.y - 1,
        });
    }
    if can_move_down(position, map, occupied_positions) {
        reachable_positions.push(MapPosition {
            x: position.x,
            y: position.y + 1,
        });
    }
    return reachable_positions;
}

pub fn can_move_left(
    position: &MapPosition,
    map: &Map,
    occupied_positions: &Vec<MapPosition>,
) -> bool {
    if position.x > 0 {
        map.tiles[position.x + position.y * map.width - 1]
            .clone()
            .is_walkable()
            && !occupied_positions.contains(&MapPosition {
                x: position.x - 1,
                y: position.y,
            })
    } else {
        false
    }
}

pub fn can_move_right(
    position: &MapPosition,
    map: &Map,
    occupied_positions: &Vec<MapPosition>,
) -> bool {
    if position.x < map.width - 1 {
        map.tiles[position.x + position.y * map.width + 1]
            .clone()
            .is_walkable()
            && !occupied_positions.contains(&MapPosition {
                x: position.x + 1,
                y: position.y,
            })
    } else {
        false
    }
}

pub fn can_move_up(
    position: &MapPosition,
    map: &Map,
    occupied_positions: &Vec<MapPosition>,
) -> bool {
    if position.y > 0 {
        map.tiles[position.x + (position.y - 1) * map.width]
            .clone()
            .is_walkable()
            && !occupied_positions.contains(&MapPosition {
                x: position.x,
                y: position.y - 1,
            })
    } else {
        false
    }
}

pub fn can_move_down(
    position: &MapPosition,
    map: &Map,
    occupied_positions: &Vec<MapPosition>,
) -> bool {
    if position.y < map.height - 1 {
        map.tiles[position.x + (position.y + 1) * map.width]
            .clone()
            .is_walkable()
            && !occupied_positions.contains(&MapPosition {
                x: position.x,
                y: position.y + 1,
            })
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
            tiles: vec![TileKind::Grass; 3 * 3],
            exits: vec![],
            number: 0,
        }
    }

    fn create_stone_map() -> Map {
        Map {
            width: 3,
            height: 3,
            tiles: vec![
                TileKind::Grass,
                TileKind::GrassWithStone,
                TileKind::Grass,
                TileKind::GrassWithStone,
                TileKind::Grass,
                TileKind::GrassWithStone,
                TileKind::Grass,
                TileKind::GrassWithStone,
                TileKind::Grass,
            ],
            exits: vec![],
            number: 0,
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
        assert!(!can_move_left(&POSITION_TOP_LEFT, &map_plain, &vec![]));
        assert!(!can_move_left(&POSITION_BOTTOM_LEFT, &map_plain, &vec![]));
        assert!(can_move_left(&POSITION_TOP_RIGHT, &map_plain, &vec![]));
        assert!(can_move_left(&POSITION_BOTTOM_RIGHT, &map_plain, &vec![]));
        assert!(can_move_left(&POSITION_MIDDLE, &map_plain, &vec![]));

        let map_stone = create_stone_map();
        assert!(!can_move_left(&POSITION_TOP_LEFT, &map_stone, &vec![]));
        assert!(!can_move_left(&POSITION_TOP_RIGHT, &map_stone, &vec![]));
        assert!(!can_move_left(&POSITION_BOTTOM_LEFT, &map_stone, &vec![]));
        assert!(!can_move_left(&POSITION_BOTTOM_RIGHT, &map_stone, &vec![]));
        assert!(!can_move_left(&POSITION_MIDDLE, &map_stone, &vec![]));
    }

    #[test]
    fn test_can_move_left_with_actors() {
        let map_plain = create_plain_map();
        let actor_positions = vec![POSITION_MIDDLE];

        assert!(!can_move_left(
            &POSITION_MIDDLE_RIGHT,
            &map_plain,
            &actor_positions,
        ));
        assert!(can_move_left(
            &POSITION_TOP_RIGHT,
            &map_plain,
            &actor_positions,
        ));
        assert!(can_move_left(
            &POSITION_BOTTOM_RIGHT,
            &map_plain,
            &actor_positions,
        ));
    }

    #[test]
    fn test_can_move_right_without_actors() {
        let map_plain = create_plain_map();
        assert!(!can_move_right(&POSITION_TOP_RIGHT, &map_plain, &vec![]));
        assert!(!can_move_right(&POSITION_BOTTOM_RIGHT, &map_plain, &vec![]));
        assert!(can_move_right(&POSITION_TOP_LEFT, &map_plain, &vec![]));
        assert!(can_move_right(&POSITION_BOTTOM_LEFT, &map_plain, &vec![]));
        assert!(can_move_right(&POSITION_MIDDLE, &map_plain, &vec![]));

        let map_stone = create_stone_map();
        assert!(!can_move_right(&POSITION_TOP_LEFT, &map_stone, &vec![]));
        assert!(!can_move_right(&POSITION_TOP_RIGHT, &map_stone, &vec![]));
        assert!(!can_move_right(&POSITION_BOTTOM_LEFT, &map_stone, &vec![]));
        assert!(!can_move_right(&POSITION_BOTTOM_RIGHT, &map_stone, &vec![]));
        assert!(!can_move_right(&POSITION_MIDDLE, &map_stone, &vec![]));
    }

    #[test]
    fn test_can_move_right_with_actors() {
        let map_plain = create_plain_map();
        let actor_positions = vec![POSITION_MIDDLE];

        assert!(!can_move_right(
            &POSITION_MIDDLE_LEFT,
            &map_plain,
            &actor_positions,
        ));
        assert!(can_move_right(
            &POSITION_TOP_LEFT,
            &map_plain,
            &actor_positions,
        ));
        assert!(can_move_right(
            &POSITION_BOTTOM_LEFT,
            &map_plain,
            &actor_positions,
        ));
    }

    #[test]
    fn test_can_move_up_without_actors() {
        let map_plain = create_plain_map();
        assert!(!can_move_up(&POSITION_TOP_LEFT, &map_plain, &vec![]));
        assert!(!can_move_up(&POSITION_TOP_RIGHT, &map_plain, &vec![]));
        assert!(can_move_up(&POSITION_BOTTOM_LEFT, &map_plain, &vec![]));
        assert!(can_move_up(&POSITION_BOTTOM_RIGHT, &map_plain, &vec![]));
        assert!(can_move_up(&POSITION_MIDDLE, &map_plain, &vec![]));

        let map_stone = create_stone_map();
        assert!(!can_move_up(&POSITION_TOP_LEFT, &map_stone, &vec![]));
        assert!(!can_move_up(&POSITION_TOP_RIGHT, &map_stone, &vec![]));
        assert!(!can_move_up(&POSITION_BOTTOM_LEFT, &map_stone, &vec![]));
        assert!(!can_move_up(&POSITION_BOTTOM_RIGHT, &map_stone, &vec![]));
        assert!(!can_move_up(&POSITION_MIDDLE, &map_stone, &vec![]));
    }

    #[test]
    fn test_can_move_up_with_actors() {
        let map_plain = create_plain_map();
        let actor_positions = vec![POSITION_MIDDLE];

        assert!(can_move_up(
            &POSITION_BOTTOM_RIGHT,
            &map_plain,
            &actor_positions,
        ));
        assert!(can_move_up(
            &POSITION_BOTTOM_LEFT,
            &map_plain,
            &actor_positions,
        ));
        assert!(!can_move_up(
            &POSITION_BOTTOM_MIDDLE,
            &map_plain,
            &actor_positions,
        ));
    }

    #[test]
    fn test_can_move_down_without_actors() {
        let map_plain = create_plain_map();
        assert!(!can_move_down(&POSITION_BOTTOM_LEFT, &map_plain, &vec![]));
        assert!(!can_move_down(&POSITION_BOTTOM_RIGHT, &map_plain, &vec![]));
        assert!(can_move_down(&POSITION_TOP_LEFT, &map_plain, &vec![]));
        assert!(can_move_down(&POSITION_TOP_RIGHT, &map_plain, &vec![]));
        assert!(can_move_down(&POSITION_MIDDLE, &map_plain, &vec![]));

        let map_stone = create_stone_map();
        assert!(!can_move_down(&POSITION_TOP_LEFT, &map_stone, &vec![]));
        assert!(!can_move_down(&POSITION_TOP_RIGHT, &map_stone, &vec![]));
        assert!(!can_move_down(&POSITION_BOTTOM_LEFT, &map_stone, &vec![]));
        assert!(!can_move_down(&POSITION_BOTTOM_RIGHT, &map_stone, &vec![]));
        assert!(!can_move_down(&POSITION_MIDDLE, &map_stone, &vec![]));
    }

    #[test]
    fn test_can_move_down_with_actors() {
        let map_plain = create_plain_map();
        let actor_positions = vec![POSITION_MIDDLE];

        assert!(can_move_down(
            &POSITION_TOP_RIGHT,
            &map_plain,
            &actor_positions,
        ));
        assert!(can_move_down(
            &POSITION_TOP_LEFT,
            &map_plain,
            &actor_positions,
        ));
        assert!(!can_move_down(
            &POSITION_TOP_MIDDLE,
            &map_plain,
            &actor_positions,
        ));
    }

    #[test]
    fn test_enumerate_reachable_positions() {
        let map_plain = create_plain_map();
        let actor_positions = vec![];

        let reachable_positions = enumerate_reachable_positions(
            &POSITION_MIDDLE,
            &map_plain,
            &actor_positions,
        );

        assert_eq!(4, reachable_positions.len())
    }
}
