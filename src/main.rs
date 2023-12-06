use nix::fcntl::fcntl;
use nix::fcntl::FcntlArg;
use nix::sys::memfd::memfd_create;
use nix::sys::memfd::MemFdCreateFlag;
use nix::sys::mman::mmap;
use nix::sys::mman::MapFlags;
use nix::sys::mman::ProtFlags;
use std::num::NonZeroUsize;
use std::ffi::CString;
use std::os::unix::io::AsRawFd;

fn main() {
    let s = CString::new("foo");
    let fd = memfd_create(
        s.unwrap().as_c_str(),
        MemFdCreateFlag::MFD_CLOEXEC | MemFdCreateFlag::MFD_ALLOW_SEALING,
    )
    .unwrap();
    dbg!(fcntl(fd.as_raw_fd(), FcntlArg::F_GETFD));
    let descriptor = fcntl(fd.as_raw_fd(), FcntlArg::F_DUPFD_CLOEXEC(0)).unwrap();
    dbg!(fcntl(descriptor, FcntlArg::F_GETFD));
    dbg!(fcntl(fd.as_raw_fd(), FcntlArg::F_GETFD));

    dbg!(unsafe { mmap(
        None,
        NonZeroUsize::new(4096).unwrap(),
        ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
        MapFlags::MAP_SHARED,
        Some(fd),
        0
    )});
    println!("Hello, world!");
}
