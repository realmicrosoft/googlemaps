/// Linux-Based Operating System
#[derive(Copy, Clone, Debug)]
pub enum LinuxKernel {
    NormalLinuxGnu,
    NormalLinuxMusl,
    Android,
    ChromeOS,
}

/// Darwin-Based Operating System
#[derive(Copy, Clone, Debug)]
pub enum DarwinKernel {
    MacOSGreaterThan9, // we can't call it MacOsX, because apple decided to make a version 11!
    IOS,
    IPadOs,
    WatchOS,
    TVOS,
}

/// An operating system that is in some way "Unix-like"
/// Usually means that it will use forward slashes for paths and be in some way better than windows
#[derive(Copy, Clone, Debug)]
pub enum UnixLike {
    Linux(LinuxKernel),
    Darwin(DarwinKernel),
    BSD,
    SolarisOrUhOopsIMeanIllumos,
}

/// An operating system that runs off of the NT kernel
/// WARNING! This will not detect Windows Servers yet, so don't add cases for `NTKernel::WindowsServer`
/// until i stop being lazy and add support for it
#[derive(Copy, Clone, Debug)]
pub enum NTKernel {
    Windows,
    WindowsServer,
}

/// Operating Systems are what makes your computer do things without you having to manually connect pins on your CPU
#[derive(Copy, Clone, Debug)]
pub enum OperatingSystem {
    UnixLike(UnixLike),
    Windows(NTKernel),
    DOS,
    Unknown,
}

/// The x86 Architecture
/// WARNING! I don't think this will actually properly detect 8086 cpus yet
/// so you might not want to use this crate for DOS programming until i fix this
#[derive(Copy, Clone, Debug)]
pub enum X86 {
    AMD64,
    I386,
    I486,
    I586,
    I686,
    EightyEightySix, // rust doesn't like me using 8086 as an identifier
}

// todo! research into what all of this actually means
/// The "Arm" Architecture
/// I don't actually know much about this one, so it may get incorrect results!
#[derive(Copy, Clone, Debug)]
pub enum ARM {
    AArch32,
    AArch64,
    AppleSilicon,
}

// i love mips so much 🥵
/// The Best or uh I mean The MIPS Architecture
/// WARNING! this doesn't actually detect MIPSI, MIPSII, MIPSIII, or MIPSIV yet
/// you can only detect MIPS32 or MIPS64
#[derive(Copy, Clone, Debug)]
pub enum MIPS {
    MipsI,
    MipsII,
    MipsIII,
    MipsIV,
    MipsV,
    Mips32,
    Mips64,
}

/// CPUs make your computer a "computer"
#[derive(Copy, Clone, Debug)]
pub enum CPU {
    X86(X86),
    ARM(ARM),
    MIPS(MIPS),
    PowerPC,
    SPARC,
    RISC,
    RISCV,
    Alpha,
    IA64,
    HPPA,
    S390,
    S390X,
    SuperH,
    SystemZ,
    XCore,
    Other,
}

/// Window Systems let you see things on your screen that aren't just text
/// I mean I guess there are also text based window systems but we aren't detecting those yet
#[derive(Copy, Clone, Debug)]
pub enum WindowSystem {
    X11,
    Wayland,
    ExplorerDotExe,
    Quartz, // apple's window system
    Unknown,
    None,
}

/// Detects what operating system the program is running on, and returns the `OperatingSystem` enum
pub fn detect_os() -> OperatingSystem {
    match std::env::consts::OS {
        "linux" => {
            // figure out whether we're on gnu or musl
            let kernel = if cfg!(target_env = "gnu") {
                LinuxKernel::NormalLinuxGnu
            } else if cfg!(target_env = "musl") {
                LinuxKernel::NormalLinuxMusl
            } else {
                // todo! figure out what to do here
                LinuxKernel::NormalLinuxGnu
            };

            OperatingSystem::UnixLike(UnixLike::Linux(kernel))
        }
        "macos" => {
            // figure out whether we're on macos or ios
            let kernel = if cfg!(target_os = "ios") {
                DarwinKernel::IOS
            } else if cfg!(target_os = "macos") {
                DarwinKernel::MacOSGreaterThan9
            } else if cfg!(target_os = "ipados") {
                DarwinKernel::IPadOs
            } else if cfg!(target_os = "watchos") {
                DarwinKernel::WatchOS
            } else if cfg!(target_os = "tvos") {
                DarwinKernel::TVOS
            } else {
                // todo! figure out what to do here
                DarwinKernel::MacOSGreaterThan9
            };

            OperatingSystem::UnixLike(UnixLike::Darwin(kernel))
        }
        "windows" => {
            // todo! check if we're on windows server

            OperatingSystem::Windows(NTKernel::Windows)
        }
        "freebsd" | "netbsd" | "openbsd" | "dragonfly" => OperatingSystem::UnixLike(UnixLike::BSD),
        "solaris" | "illumos" => OperatingSystem::UnixLike(UnixLike::SolarisOrUhOopsIMeanIllumos),
        "dos" => OperatingSystem::DOS,
        "android" => OperatingSystem::UnixLike(UnixLike::Linux(LinuxKernel::Android)),
        "ios" => OperatingSystem::UnixLike(UnixLike::Darwin(DarwinKernel::IOS)),
        _ => OperatingSystem::Unknown,
    }
}

/// Detects what CPU Architecture the program is running on, and returns the `CPU` enum
pub fn detect_architecture() -> CPU {
    match std::env::consts::ARCH {
        "x86_64" => CPU::X86(X86::AMD64),
        "i386" => CPU::X86(X86::I386),
        "i486" => CPU::X86(X86::I486),
        "i586" => CPU::X86(X86::I586),
        "i686" => CPU::X86(X86::I686),
        "8086" => CPU::X86(X86::EightyEightySix),
        "arm" => CPU::ARM(ARM::AArch32),
        "aarch64" => CPU::ARM(ARM::AArch64),
        "mips" => CPU::MIPS(MIPS::Mips32),
        "mips64" => CPU::MIPS(MIPS::Mips64),
        "powerpc" => CPU::PowerPC,
        "sparc" => CPU::SPARC,
        "risc" => CPU::RISC,
        "riscv" => CPU::RISCV,
        "alpha" => CPU::Alpha,
        "ia64" => CPU::IA64,
        "hppa" => CPU::HPPA,
        "s390" => CPU::S390,
        "s390x" => CPU::S390X,
        "sh" => CPU::SuperH,
        "systemz" => CPU::SystemZ,
        "xcore" => CPU::XCore,
        _ => CPU::Other,
    }
}

/// Detects what window system the program is running under, and returns the `WindowSystem` enum
pub fn detect_windowsystem() -> WindowSystem {
    match std::env::consts::OS {
        "linux" => {
            // check if we're actually on a tty
            if !std::env::var("DISPLAY").is_ok() && !std::env::var("WAYLAND_DISPLAY").is_ok() {
                return WindowSystem::None;
            }

            // check $XDG_SESSION_TYPE
            // if it's "wayland", we're on wayland
            // if it's "x11", we're on x11
            // if neither, assume x11 because hey at least wayland has xwayland!
            let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or("x11".to_string());

            if session_type == "wayland" {
                WindowSystem::Wayland
            } else {
                WindowSystem::X11
            }
        }
        "macos" => WindowSystem::Quartz, // I MEAN I GUESS YOU COULD BE USING XQUARTZ BUT I DON'T CARE ENOUGH TO CHECK
        "windows" => WindowSystem::ExplorerDotExe, // like someone would port x11 to windows lol, wait that exists?
        _ => WindowSystem::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_os() {
        let os = detect_os();

        println!("{:?}", os);
    }

    #[test]
    fn test_detect_architecture() {
        let arch = detect_architecture();

        println!("{:?}", arch);
    }

    #[test]
    fn test_detect_windowsystem() {
        let ws = detect_windowsystem();

        println!("{:?}", ws);
    }
}
