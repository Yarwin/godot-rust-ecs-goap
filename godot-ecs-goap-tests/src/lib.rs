use gdnative::prelude::*;
use gdnative_core::godot_itest;

mod test_goap_planner;

#[no_mangle]
pub extern "C" fn run_tests(
    _data: *mut gdnative::libc::c_void,
    _args: *mut gdnative::sys::godot_array,
) -> gdnative::sys::godot_variant {
    let mut status = true;
    status &= test_it_works();
    status &= test_goap_planner::run_tests();
    Variant::new(status).leak()
}

godot_itest! { test_it_works {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}



fn init(handle: InitHandle) {
}

fn terminate(_term_info: &gdnative::init::TerminateInfo) {
    gdnative::tasks::terminate_runtime();
}

gdnative::init::godot_gdnative_init!();
gdnative::init::godot_nativescript_init!(init);
gdnative::init::godot_gdnative_terminate!(terminate);
