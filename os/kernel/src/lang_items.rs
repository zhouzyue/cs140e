use core::panic::PanicInfo;

#[panic_handler] #[no_mangle] pub extern fn panic(_info: &PanicInfo) -> ! {
    loop { unsafe { asm!("wfe") } }
}

use core::alloc::Layout;

#[alloc_error_handler]
pub fn rust_oom(layout: Layout) -> ! {
//    let hook = HOOK.load(Ordering::SeqCst);
//    let hook: fn(Layout) = if hook.is_null() {
//        default_alloc_error_hook
//    } else {
//        unsafe { mem::transmute(hook) }
//    };
//    hook(layout);
//    unsafe { crate::sys::abort_internal(); }
    loop {unsafe { asm!("wfe")}}
}

use console::kprintln;
use pi::timer::spin_sleep_ms;

pub extern fn panic_fmt(fmt: ::std::fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {
    // FIXME: Print `fmt`, `file`, and `line` to the console.
    spin_sleep_ms(3000);
    let r = r#"
                (
           (      )     )
             )   (    (
            (          `
        .-""^"""^""^"""^""-.
      (//\\//\\//\\//\\//\\//)
       ~\^^^^^^^^^^^^^^^^^^/~
         `================`

        The pi is overdone.
    "#;
    kprintln!("{}", r);
    kprintln!("---------- PANIC ----------");
    kprintln!("FILE: {}", file);
    kprintln!("LINE: {}", line);
    kprintln!("COL: {}", col);
    kprintln!("");

    kprintln!("{}", fmt);
    loop { unsafe { asm!("wfe") } }
}

//#[cfg(not(test))] #[lang = "eh_personality"] pub extern fn eh_personality() {}
