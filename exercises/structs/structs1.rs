// structs1.rs
// Address all the TODOs to make the tests pass!

struct ColorClassicStruct {
    // TODO: Something goes here
}

struct ColorTupleStruct(/* TODO: Something goes here */);

struct ColorUnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        // let green =

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        // For more fun, use the field initialization shorthand.
        // let green =

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        // let green =

        if let ColorUnitStruct = green {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
