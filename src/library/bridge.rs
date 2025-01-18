#[link(name = "pkgmgr", kind = "static")]
extern "C" {
    fn add(left: isize, right: isize) -> isize;
    fn sub(left: isize, right: isize) -> isize;
    fn mul(left: isize, right: isize) -> isize;
    fn div(left: isize, right: isize) -> isize;
}