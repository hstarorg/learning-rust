#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod hi {
    pub struct User {
        pub name: String,
    }
}

pub mod utils;

pub mod dal;