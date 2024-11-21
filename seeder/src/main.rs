use core::prelude::*;
use env_logger::Env;
use log::info;
use prelude::TSeeder;

mod processor_seeder;
mod prelude;
mod transfer_config_seeder;

use clap::Parser;

/// Program to run database manipulation like seeding, and migration
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// run migration
    #[arg(short, long, default_value_t = false)]
    migrate: bool,

    /// Run Seeders: processor, transfer_config
    #[arg(short, long)]
    seeds: Vec<String>,
}

impl Args {
    fn validate_seed(&self) -> bool {
        let seeds = vec!["processor", "transfer_config"];
        for seed in self.seeds.iter() {
            if seeds.contains(&seed.as_str()) {
                return true;
            }
        }
        false
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    if args.validate_seed() {
        let conf = core::config::get_config();
        let poll = core::utils::connection::get_mysql_pool(&conf.database_url).await?;

        for seed in args.seeds.iter() {
            match seed.as_str() {
                "processor" => {
                    info!("Running Processor Seeder");
                    processor_seeder::ProcessorSeeder::new("processor_seeder", poll.clone())
                        .execute()
                        .await?;
                    
                }
                "transfer_config" => {
                    info!("Running Transfer Config Seeder");
                    transfer_config_seeder::TransferConfigSeeder::new("transfer_config_seeder", poll.clone())
                        .execute()
                        .await?;
                }
                _ => {}
                
            }
        }
    }

    Ok(())
}
