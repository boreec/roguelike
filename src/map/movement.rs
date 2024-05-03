use crate::prelude::*;

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

pub fn can_move_left(
    player_position: &MapPosition,
    map: &Map,
    occupied_positions: &Vec<MapPosition>,
) -> bool {
    if player_position.x > 0 {
        map.tiles[player_position.x + player_position.y * map.width - 1]
            .clone()
            .is_walkable()
            && !occupied_positions.contains(&MapPosition {
                x: player_position.x - 1,
                y: player_position.y,
            })
    } else {
        false
    }
}

pub fn can_move_right(
    player_position: &MapPosition,
    map: &Map,
    occupied_positions: &Vec<MapPosition>,
) -> bool {
    if player_position.x < map.width - 1 {
        map.tiles[player_position.x + player_position.y * map.width + 1]
            .clone()
            .is_walkable()
            && !occupied_positions.contains(&MapPosition {
                x: player_position.x + 1,
                y: player_position.y,
            })
    } else {
        false
    }
}

pub fn can_move_up(
    player_position: &MapPosition,
    map: &Map,
    occupied_positions: &Vec<MapPosition>,
) -> bool {
    if player_position.y > 0 {
        map.tiles[player_position.x + (player_position.y - 1) * map.width]
            .clone()
            .is_walkable()
            && !occupied_positions.contains(&MapPosition {
                x: player_position.x,
                y: player_position.y - 1,
            })
    } else {
        false
    }
}

pub fn can_move_down(
    player_position: &MapPosition,
    map: &Map,
    occupied_positions: &Vec<MapPosition>,
) -> bool {
    if player_position.y < map.height - 1 {
        map.tiles[player_position.x + (player_position.y + 1) * map.width]
            .clone()
            .is_walkable()
            && !occupied_positions.contains(&MapPosition {
                x: player_position.x,
                y: player_position.y + 1,
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
            tiles: vec![TileType::Grass; 3 * 3],
            exits: vec![],
        }
    }

    fn create_stone_map() -> Map {
        Map {
            width: 3,
            height: 3,
            tiles: vec![
                TileType::Grass,
                TileType::GrassWithStone,
                TileType::Grass,
                TileType::GrassWithStone,
                TileType::Grass,
                TileType::GrassWithStone,
                TileType::Grass,
                TileType::GrassWithStone,
                TileType::Grass,
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
    const POSITION_BOTTOM_MIDDLE: MapPosition = MapPosition { x: 0, y: 1 };
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
}
