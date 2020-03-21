#[cfg(test)]
mod tests {
    use super::*;
    use cute_c2_sys as ffi;
    #[test]
    fn it_works() {
        let a = ffi::c2Circle {
            p: ffi::c2v { x: 0.0, y: 0.0 },
            r: 5.0,
        };
        let b = ffi::c2Circle {
            p: ffi::c2v { x: 4.0, y: 0.0 },
            r: 5.0,
        };
        unsafe {
            let touching = ffi::c2CircletoCircle(a, b) != 0;
            assert_eq!(touching, true);
        }
    }
}
