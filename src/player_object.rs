// PlayerObject::init = base + 0x1E6DA0
// void __thiscall PlayerObject::pushButton(PlayerObject*, PlayerButton player_button) = base + 0x1F4E40;
// void __thiscall PlayerObject::releaseButton(PlayerObject*, PlayerButton player_button) = base + 0x1F4F70;
// void __thiscall PlayerObject::playDeathEffect(PlayerObject*) = base + 0x1EFBE0;
// void __thiscall PlayerObject::playerDestroyed(PlayerObject*, bool destroyed) = base + 0x1EFAA0;
// void __thiscall PlayerObject::flipGravity(PlayerObject*, bool m_unknown_1, bool m_unknown_2) = base + 0x1F59D0;
// void __thiscall PlayerObject::toggleFlyMode(PlayerObject*, bool toggle) = base + 0x1F5E40;
// void __thiscall PlayerObject::toggleBirdMode(PlayerObject*, bool toggle) = base + 0x1F6050;
// void __thiscall PlayerObject::toggleRollMode(PlayerObject*, bool toggle) = base + 0x1F68E0;
// void __thiscall PlayerObject::toggleDartMode(PlayerObject*, bool toggle) = base + 0x1F62C0;
// void __thiscall PlayerObject::toggleRobotMode(PlayerObject*, bool toggle) = base + 0x1F6A10;
// void __thiscall PlayerObject::toggleSpiderMode(PlayerObject*, bool toggle) = base + 0x1F94D1;
// void __thiscall PlayerObject::updateTimeMod(PlayerObject*, float time) = base + 0x1F94E0;
// void __thiscall PlayerObject::togglePlayerScale(PlayerObject*, bool toggle) = base + 0x1F9640;
// void __thiscall PlayerObject::setFlipX(PlayerObject*, bool flip) = base + 0x1FA690;
// void __thiscall PlayerObject::setFlipY(PlayerObject*, bool flip) = base + 0x1FA740;
// void __thiscall PlayerObject::resetObject(PlayerObject*) = base + 0x1EECD0;
// CCPoint __thiscall PlayerObject::getRealPosition(PlayerObject*) = base + 0x1F7E20;
// m_250h_undefined __thiscall PlayerObject::getOrientedBox(PlayerObject*) = base + 0x1F95D0;
// void __thiscall PlayerObject::fadeOutStreak2(PlayerObject*, float m_unknown_1) = base + 0x1F9110;
// void __thiscall PlayerObject::toggleGhostEffect(PlayerObject*, GhostType type) = base + 0x1F8930;
//
// If none of them are true, the player is in cube mode
// bool PlayerObject.isShip = PlayerObject + 0x638;
// bool PlayerObject.isBird = PlayerObject + 0x639;
// bool PlayerObject.isBall = PlayerObject + 0x63A;
// bool PlayerObject.isDart = PlayerObject + 0x63B;
// bool PlayerObject.isRobot = PlayerObject + 0x63C;
// bool PlayerObject.isSpider = PlayerObject + 0x63D;
// bool PlayerObject.isFlipped = PlayerObject + 0x63E;
//
// float PlayerObject.x = PlayerObject + 0x67C;
// float PlayerObject.y = PlayerObject + 0x680;
// double PlayerObject.y_acceleration = PlayerObject + 0x628;
// CCMotionStreak* PlayerObject.trail = PlayerObject + 0x510;
// HardStreak* PlayerObject.wave_trail = PlayerObject + 0x514;

use crate::{impl_get_set, read_mem, Ptr};

/// Player gamemode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    /// Cube gamemode.
    Cube,
    /// Ship gamemode.
    Ship,
    /// UFO (bird) gamemode.
    Ufo,
    /// Ball gamemode.
    Ball,
    /// Wave (dart) gamemode.
    Wave,
    /// Robot gamemode.
    Robot,
    /// Spider gamemode.
    Spider,
}

/// PlayerObject
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerObject {
    address: Ptr,
}

impl PlayerObject {
    #[inline(always)]
    pub fn from_address(address: Ptr) -> Self {
        Self { address }
    }

    // getters & setters, generated automatically by the macro

    impl_get_set!(x, set_x, f32, 0x67C);
    impl_get_set!(y, set_y, f32, 0x680);

    /// Get the X and Y position.
    #[inline]
    pub fn get_position(&self) -> (f32, f32) {
        (self.x(), self.y())
    }

    /// Set the X and Y position.
    #[inline]
    pub fn set_position(&self, position: (f32, f32)) {
        self.set_x(position.0);
        self.set_y(position.1);
    }

    impl_get_set!(x_accel, set_x_accel, f64, 0x518);
    impl_get_set!(y_accel, set_y_accel, f64, 0x628);
    impl_get_set!(jump_accel, set_jump_accel, f64, 0x520);
    impl_get_set!(is_holding, set_is_holding, bool, 0x611);
    impl_get_set!(has_just_held, set_has_just_held, bool, 0x612);
    impl_get_set!(is_holding2, set_is_holding2, bool, 0x613);
    impl_get_set!(has_just_held2, set_has_just_held2, bool, 0x614);
    impl_get_set!(can_robot_jump, set_can_robot_jump, bool, 0x624);
    impl_get_set!(is_upside_down, set_is_upside_down, bool, 0x63E);
    impl_get_set!(is_on_ground, set_is_on_ground, bool, 0x640);
    impl_get_set!(is_dashing, set_is_dashing, bool, 0x641);
    impl_get_set!(is_sliding, set_is_sliding, bool, 0x660);
    impl_get_set!(is_rising, set_is_rising, bool, 0x661);
    impl_get_set!(black_orb, set_black_orb, bool, 0x5FE);
    impl_get_set!(unk662, set_unk662, bool, 0x662);
    impl_get_set!(unk630, set_unk630, bool, 0x630);
    impl_get_set!(unk631, set_unk631, bool, 0x631);
    impl_get_set!(vehicle_size, set_vehicle_size, f32, 0x644);
    impl_get_set!(player_speed, set_player_speed, f32, 0x648);
    impl_get_set!(rotation_x, set_rotation_x, f32, 0x20);
    impl_get_set!(rotation_y, set_rotation_y, f32, 0x24);

    /// Returns the player's gamemode.
    #[inline]
    pub fn game_mode(&self) -> GameMode {
        unsafe {
            if *read_mem(self.address + 0x638) {
                GameMode::Ship
            } else if *read_mem(self.address + 0x639) {
                GameMode::Ufo
            } else if *read_mem(self.address + 0x63A) {
                GameMode::Ball
            } else if *read_mem(self.address + 0x63B) {
                GameMode::Wave
            } else if *read_mem(self.address + 0x63C) {
                GameMode::Robot
            } else if *read_mem(self.address + 0x63D) {
                GameMode::Spider
            } else {
                GameMode::Cube // if none are true, the player is in cube gamemode
            }
        }
    }

    /// Sets the player's gamemode.
    #[inline]
    pub fn set_game_mode(&self, gamemode: GameMode) {
        unsafe {
            *read_mem(self.address + 0x638) = gamemode == GameMode::Ship;
            *read_mem(self.address + 0x639) = gamemode == GameMode::Ufo;
            *read_mem(self.address + 0x63A) = gamemode == GameMode::Ball;
            *read_mem(self.address + 0x63B) = gamemode == GameMode::Wave;
            *read_mem(self.address + 0x63C) = gamemode == GameMode::Robot;
            *read_mem(self.address + 0x63D) = gamemode == GameMode::Spider;
        }
    }
}

crate::impl_addr_funcs!(PlayerObject);
