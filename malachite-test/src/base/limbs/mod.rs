use common::DemoBenchRegistry;

pub mod limbs_delete_left;
pub mod limbs_leading_zero_limbs;
pub mod limbs_move_left;
pub mod limbs_pad_left;
pub mod limbs_set_zero;
pub mod limbs_test_zero;
pub mod limbs_trailing_zero_limbs;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    limbs_delete_left::register(registry);
    limbs_leading_zero_limbs::register(registry);
    limbs_move_left::register(registry);
    limbs_pad_left::register(registry);
    limbs_set_zero::register(registry);
    limbs_test_zero::register(registry);
    limbs_trailing_zero_limbs::register(registry);
}
