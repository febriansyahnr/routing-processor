use core::prelude::*;

pub trait TSeeder {
    fn execute(&self) -> impl std::future::Future<Output = Result<()>>;
}
