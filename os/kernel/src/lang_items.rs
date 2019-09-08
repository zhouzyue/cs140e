use core::panic::PanicInfo;
use console::kprintln;
use pi::timer::spin_sleep_ms;

//#[panic_handler]
//#[no_mangle] pub extern fn panic(_info: &PanicInfo) -> ! {
//    spin_sleep_ms(3000);
//    let r = r#"
//                (
//           (      )     )
//             )   (    (
//            (          `
//        .-""^"""^""^"""^""-.
//      (//\\//\\//\\//\\//\\//)
//       ~\^^^^^^^^^^^^^^^^^^/~
//         `================`
//
//        The pi is overdone.
//    "#;
//    kprintln!("{}", r);
//    kprintln!("---------- PANIC ----------");
//    if let Some(location) = _info.location() {
//        kprintln!("FILE: {}", location.file());
//        kprintln!("LINE: {}", location.line());
//        kprintln!("COL: {}", location.column());
//    }
//    kprintln!("");
//    if let Some(message) = _info.message() {
//         kprintln!("{:?}", message);
//    }
//    loop { unsafe { asm!("wfe") } }
//}

use core::alloc::Layout;

//#[alloc_error_handler]
//pub fn rust_oom(layout: Layout) -> ! {
//    let hook = HOOK.load(Ordering::SeqCst);
//    let hook: fn(Layout) = if hook.is_null() {
//        default_alloc_error_hook
//    } else {
//        unsafe { mem::transmute(hook) }
//    };
//    hook(layout);
//    unsafe { crate::sys::abort_internal(); }
//    loop {unsafe { asm!("wfe")}}
//}

//#[cfg(not(test))] #[lang = "eh_personality"] pub extern fn eh_personality() {}

#[no_mangle]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    return dest;
}

#[no_mangle]
pub unsafe extern fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 { // copy from end
        let mut i = n;
        while i != 0 {
            i -= 1;
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    } else { // copy from beginning
        let mut i = 0;
        while i < n {
            *dest.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }
    return dest;
}

#[no_mangle]
pub unsafe extern fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    return s;
}

#[no_mangle]
pub unsafe extern fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32
        }
        i += 1;
    }
    return 0;
}
