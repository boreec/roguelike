use crate::map::MapPosition;
use crate::MAP_HEIGHT;
use crate::MAP_WIDTH;

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

pub const fn can_move_right(player_position: &MapPosition) -> bool {
    player_position.x < MAP_WIDTH - 1
}

pub const fn can_move_up(player_position: &MapPosition) -> bool {
    player_position.y > 0
}

pub const fn can_move_down(player_position: &MapPosition) -> bool {
    player_position.y < MAP_HEIGHT - 1
}
