use crate::{impl_addr_funcs, impl_get_set, Ptr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameObject {
    address: Ptr,
}

impl GameObject {
    #[inline(always)]
    pub fn from_address(address: Ptr) -> Self {
        Self { address }
    }

    // https://github.com/maxnut/gd.h/blob/436af2c6440a7efd1ba4720e48429fddaaf298e4/sprite_nodes/GameObject.h#L62-L194

    impl_get_set!(is_object_rect_dirty, set_is_object_rect_dirty, bool, 0x2C8);
    impl_get_set!(
        is_oriented_rect_dirty,
        set_is_oriented_rect_dirty,
        bool,
        0x2C9
    );
    impl_get_set!(has_been_activated, set_has_been_activated, bool, 0x2CA);
    impl_get_set!(
        has_been_activated_p2,
        set_has_been_activated_p2,
        bool,
        0x2CB
    );
}

impl_addr_funcs!(GameObject);
