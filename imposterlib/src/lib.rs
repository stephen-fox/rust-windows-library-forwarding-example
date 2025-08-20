#![allow(non_snake_case)]

#[unsafe(no_mangle)]
extern "system" fn DllMain(_: isize, call_reason: u32, _: *mut ()) -> bool {
    // https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain
    const DLL_PROCESS_ATTACH: u32 = 1;

    if call_reason == DLL_PROCESS_ATTACH {
        attach();
    }

    true
}

fn attach() {
    eprintln!(
        ">:) imposter code loaded into: '{}'",
        std::env::args().collect::<Vec<_>>().join(" ")
    );
}

// Note: This function is only needed to generate the export
// symbol - it is never actually executed.
#[unsafe(no_mangle)]
fn add() {}
