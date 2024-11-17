use core::prelude::*;

use prelude::TSeeder;

mod processor_seeder;
mod prelude;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Run Seeders");
    let conf = core::config::Config::new()?;

    let poll = core::utils::connection::get_mysql_pool(&conf.database_url).await?;

    // Processor Seeder
    processor_seeder::ProcessorSeeder::new("processor_seeder", &poll)
    .execute()
    .await?;

    Ok(())
}
