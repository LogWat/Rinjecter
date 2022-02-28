use crate process::Process;

#[repr(C)]
pub struct Debugger {
    pub process: Process,
    pub isDebuggerAttached: bool,
}

impl Debugger {
    // Constructor ( return empty Process )
    pub fn new() -> Debugger {
        Debugger {
            process: Process::empty(),
            isDebuggerAttached: false,
        }
    }
}