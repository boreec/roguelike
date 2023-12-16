use crate::map::*;

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

pub const fn can_move_left(player_position: &MapPosition) -> bool {
    player_position.x > 0
}

pub const fn can_move_right(
    player_position: &MapPosition,
    map_size: &MapSize,
) -> bool {
    player_position.x < map_size.width - 1
}

pub const fn can_move_up(player_position: &MapPosition) -> bool {
    player_position.y > 0
}

pub const fn can_move_down(
    player_position: &MapPosition,
    map_size: &MapSize,
) -> bool {
    player_position.y < map_size.height - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    // const _MAP_FIVE_BY_FIVE: MapSize = MapSize::new(5, 5);
    const POSITION_MIDDLE: MapPosition = MapPosition { x: 2, y: 2 };
    const POSITION_TOP_LEFT: MapPosition = MapPosition { x: 0, y: 0 };
    const POSITION_TOP_RIGHT: MapPosition = MapPosition { x: 4, y: 0 };
    const POSITION_BOTTOM_LEFT: MapPosition = MapPosition { x: 0, y: 4 };
    const POSITION_BOTTOM_RIGHT: MapPosition = MapPosition { x: 4, y: 4 };

    #[test]
    fn test_can_move_left() {
        assert!(!can_move_left(&POSITION_TOP_LEFT));
        assert!(!can_move_left(&POSITION_BOTTOM_LEFT));
        assert!(can_move_left(&POSITION_TOP_RIGHT));
        assert!(can_move_left(&POSITION_BOTTOM_RIGHT));
        assert!(can_move_left(&POSITION_MIDDLE));
    }
}
