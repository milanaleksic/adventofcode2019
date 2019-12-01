pub fn fuel(f: i32) -> i32 {
    f / 3 - 2
}

pub fn fuel2(f: i32) -> i32 {
    let mut fr = 0;
    let mut ft = f;
    loop {
        ft = fuel(ft);
        if ft > 0 {
            fr += ft;
        } else {
            break;
        }
    }
    fr
}

mod tests {
    use super::*;

    #[test]
    fn test12() {
        assert_eq!(fuel(12), 2);
    }

    #[test]
    fn test14() {
        assert_eq!(fuel(14), 2);
    }

    #[test]
    fn test1969() {
        assert_eq!(fuel(1969), 654);
    }

    #[test]
    fn test100756() {
        assert_eq!(fuel(100756), 33583);
    }

    #[test]
    fn test2_14() {
        assert_eq!(fuel2(12), 2);
    }

    #[test]
    fn test2_1969() {
        assert_eq!(fuel2(1969), 966);
    }

    #[test]
    fn test2_50346() {
        assert_eq!(fuel2(100756), 50346);
    }

}
