// PlayLayer::~PlayLayer = base + 0x1FAFC0
// PlayLayer::onEnterTransitionDidFinish = base + 0x20DBD0
// PlayLayer::onExit = base + 0x20DC00
// PlayLayer::draw = base + 0x208870
// PlayLayer::visit = base + 0x200020
// PlayLayer::updateTweenAction = base + 0x20D1F0
// PlayLayer::create = base + 0x1FB6D0
// PlayLayer::init = base + 0x01FB780
// PlayLayer::update = base + 0x2029C0
// PlayLayer::updateReplay = base + 0x20AF40
// PlayLayer::releaseButton = base + 0x111660
// PlayLayer::pushButton = base + 0x111500
// PlayLayer::onQuit = base + 0x20D810
// PlayLayer::levelComplete = base + 0x1FD3D0
// PlayLayer::timeForXPos2 = base + 0x1FD3D0
// PlayLayer::togglePracticeMode = base + 0x20D0D1
// PlayLayer::destroyPlayer = base + 0x20A1A0
// PlayLayer::markCheckpoint = base + 0x25FB60
// PlayLayer::createCheckpoint = base + 0x20B050
// PlayLayer::removeLastCheckpoint = base + 0x20B830
// PlayLayer::getCapacityString = base + 0x10C9B0
// PlayLayer::updateAttempts = base + 0x20CED0
// PlayLayer::resetLevel = base + 0x20BF00
// PlayLayer::setupLevelStart = base + 0x1FB780
// PlayLayer::flipGravity = base + 0x1F59D0
// bool PlayLayer.isDead = PlayLayer + 0x39C
// float PlayLayer.levelLength = PlayLayer + 0x3B4
// bool PlayLayer.isPractice = PlayLayer + 0x495
// int PlayLayer.currentAttempt = PlayLayer + 0x4A8
// PlayerObject* PlayLayer.PlayerObject2 = PlayLayer + 0x228
// PlayerObject* PlayLayer.PlayerObject1 = PlayLayer + 0x224
// double PlayLayer.time = PlayLayer + 0x450

use crate::{
    get_base, impl_get_set, read_mem, read_ptr, AddressUtils, LevelSettings, PlayerObject, Ptr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PlayLayer {
    address: Ptr,
}

macro_rules! make_func_wrapper {
    ($addr:expr, $comment:literal, $fnname:ident($($($n:ident: $t:ty),+)?) $(-> $ret:ty)?) => {
        #[doc = $comment]
        #[inline(always)]
        pub fn $fnname(&self, $($($n: $t),+)?) $(-> $ret)? {
            unsafe {
                std::mem::transmute::<_, extern "fastcall" fn(Ptr, $($($t),+)?) $(-> $ret)?>(
                    get_base() + $addr,
                )(self.address, $($($n),+)?)
            }
        }
    };
}

impl PlayLayer {
    #[inline(always)]
    pub fn from_address(address: Ptr) -> Self {
        Self { address }
    }

    #[inline(always)]
    pub fn create(level: Ptr) -> Self {
        unsafe {
            let address = std::mem::transmute::<_, extern "fastcall" fn(Ptr) -> Ptr>(
                get_base() + 0x1FB6D0, // PlayLayer::create
            )(level);
            Self::from_address(address)
        }
    }

    /// Returns true if the player is dead. Has an extra check for the player position.
    #[inline(always)]
    pub fn is_dead(&self) -> bool {
        unsafe { *read_mem(self.address + 0x39C) && self.player1().x() != 0.0 }
    }

    // https://github.com/maxnut/gd.h/blob/436af2c6440a7efd1ba4720e48429fddaaf298e4/layers_scenes_transitions_nodes/PlayLayer.h#L60-L190C16

    impl_get_set!(is_dead_raw, set_is_dead_raw, bool, 0x39C);
    impl_get_set!(level_length, set_level_length, f32, 0x3B4);
    impl_get_set!(is_practice_mode, set_is_practice_mode, bool, 0x495);
    impl_get_set!(is_test_mode, set_is_test_mode, bool, 0x494);
    impl_get_set!(current_attempt, set_current_attempt, bool, 0x4A8);
    impl_get_set!(time, set_time, f64, 0x450);
    impl_get_set!(
        has_level_complete_menu,
        set_has_level_complete_menu,
        bool,
        0x4BD
    );
    impl_get_set!(has_completed_level, set_has_completed_level, bool, 0x4BE);
    impl_get_set!(jump_count, set_jump_count, i32, 0x4AC);
    impl_get_set!(attempt_jump_count, set_attempt_jump_count, i32, 0x4B8);
    impl_get_set!(last_death_percent, set_last_death_percent, i32, 0x4C0);
    impl_get_set!(camera_x, set_camera_x, f32, 0x48C);
    impl_get_set!(camera_y, set_camera_y, f32, 0x490);

    #[inline(always)]
    pub fn player1(&self) -> PlayerObject {
        PlayerObject::from_address(unsafe { read_ptr(self.address + 0x224) })
    }

    #[inline(always)]
    pub fn player2(&self) -> PlayerObject {
        PlayerObject::from_address(unsafe { read_ptr(self.address + 0x228) })
    }

    #[inline(always)]
    pub fn set_player1(&self, player1: PlayerObject) {
        unsafe { *read_mem(self.address + 0x224) = player1.ptr() }
    }

    #[inline(always)]
    pub fn set_player2(&self, player2: PlayerObject) {
        unsafe { *read_mem(self.address + 0x228) = player2.ptr() }
    }

    #[inline(always)]
    pub fn level_settings(&self) -> LevelSettings {
        LevelSettings::from_address(unsafe { read_ptr(self.address + 0x22C) })
    }

    make_func_wrapper!(0x2087D0, "Gets the time (in seconds) for a given X position.", time_for_xpos(xpos: f32) -> f32);
    make_func_wrapper!(0x20D0D0, "Toggles practice mode.", toggle_practice_mode(on: bool));
    make_func_wrapper!(
        0x20B830,
        "Removes the last practice checkpoint.",
        remove_last_checkpoint()
    );
    make_func_wrapper!(0x203CD0, "Checks for player collisions.", check_collisions(player: PlayerObject) -> bool);
    make_func_wrapper!(0x20D3C0, "Pauses the game.", pause_game(unk: bool));
    make_func_wrapper!(0x20BF00, "Restarts the level.", reset_level());
}

crate::impl_addr_funcs!(PlayLayer);
