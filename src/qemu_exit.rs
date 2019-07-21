const PORT: u16 = 0xf4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

fn _exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(PORT);
        port.write(exit_code as u32);
    }
}

pub fn succeed() {
    _exit_qemu(QemuExitCode::Success)
}

pub fn fail() {
    _exit_qemu(QemuExitCode::Failed)
}
