use core::prelude::*;
use prelude::TSeeder;

mod processor_seeder;
mod prelude;
mod transfer_config_seeder;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Run Seeders");
    let should_run = false; // set true to run seeder
    if should_run {
        let conf = core::config::get_config();
        let poll = core::utils::connection::get_mysql_pool(&conf.database_url).await?;
    
        // Processor Seeder
        processor_seeder::ProcessorSeeder::new("processor_seeder", &poll)
        .execute()
        .await?;
    
        // Transfer Config Seeder
        transfer_config_seeder::TransferConfigSeeder::new("transfer_config_seeder", &poll)
        .execute()
        .await?;
    }

    Ok(())
}
