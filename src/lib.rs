#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));