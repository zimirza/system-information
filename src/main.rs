use std::arch::asm;
use std::collections::HashMap;
use std::env;
use std::ffi::CStr;

use libc::c_char;

#[repr(C)]
#[derive(Debug)]
pub struct Utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}

impl Utsname {
    fn new() -> Self {
        Self {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            domainname: [0; 65],
        }
    }
}

#[derive(Debug)]
pub struct Uname {
    pub sysname: String,
    pub nodename: String,
    pub release: String,
    pub version: String,
    pub machine: String,
    pub domainname: String,
}

impl From<Utsname> for Uname {
    fn from(utsname: Utsname) -> Self {
        Self {
            sysname: { unsafe { CStr::from_ptr(utsname.sysname.as_ptr()) } }
                .to_string_lossy()
                .to_string(),
            nodename: { unsafe { CStr::from_ptr(utsname.nodename.as_ptr()) } }
                .to_string_lossy()
                .to_string(),
            release: { unsafe { CStr::from_ptr(utsname.release.as_ptr()) } }
                .to_string_lossy()
                .to_string(),
            version: { unsafe { CStr::from_ptr(utsname.version.as_ptr()) } }
                .to_string_lossy()
                .to_string(),
            machine: { unsafe { CStr::from_ptr(utsname.machine.as_ptr()) } }
                .to_string_lossy()
                .to_string(),
            domainname: { unsafe { CStr::from_ptr(utsname.domainname.as_ptr()) } }
                .to_string_lossy()
                .to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut os = HashMap::new();
    os.insert("linux", "Линукс");
    os.insert("solaris", "солярис");
    os.insert("android", "андроид");

    let operating_system = os.get(env::consts::OS);

    let utsname = Utsname::new();

    unsafe {
        asm!(
            "mov rax, 63",
            "syscall",
            in("rdi") &utsname as *const _,
        );
    }
    let uname = Uname::from(utsname);

    println!(
        "\t\t\t{}@{}",
        env::home_dir().unwrap().to_string_lossy().to_string(),
        uname.nodename
    );
    println!(
        "Операционная система:\t{} {}",
        operating_system.expect("неизвестно операционная система"),
        env::consts::ARCH
    );
    println!("Ядро:\t\t\t{}", uname.release);

    Ok(())
}
