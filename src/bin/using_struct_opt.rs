use kvs::KvStore;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kvstore", about = "In memory kv store")]
pub struct Opts {
    #[structopt(subcommand)]
    commands: Option<Kvs>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs", about = "get/set/rm")]
//#[allow(non_camel_case_types)]
enum Kvs {
    #[structopt(name = "set")]
    Set(SetOpts),

    #[structopt(name = "rm")]
    Rm(RemoveOpts),

    #[structopt(name = "get")]
    Get(GetOpts),
}

#[derive(StructOpt, Debug)]
struct SetOpts {
    #[structopt(required = true)]
    key: String,
    #[structopt(required = true)]
    value: String,
}

#[derive(Debug, StructOpt)]
pub struct RemoveOpts {
    #[structopt(required = true)]
    key: String,
}

#[derive(Debug, StructOpt)]
pub struct GetOpts {
    #[structopt(required = true)]
    key: String,
}

fn main() {
    let opt = Opts::from_args();

    if let Some(subcommand) = opt.commands {
        let mut store = KvStore::new();
        match subcommand {
            Kvs::Set(key_value) => {
                println!("{:?} ", key_value);
                store.set(key_value.key, key_value.value);
                println!("The store is now {:?} ", store);
            }
            Kvs::Rm(key) => {
                store.remove(key.key);
            }
            Kvs::Get(key) => {
                store.get(key.key);
            }
        }
    }
}
