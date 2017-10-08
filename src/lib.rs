#[cfg(test)]
extern crate hello;

pub fn min() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() { ::hello::world(); }
}
