use bluer::{gatt::remote::Characteristic, Address, Session};
use std::env;
use std::str::FromStr;

const SERVICE_UUID: &str = "0000180f-0000-1000-8000-00805f9b34fb";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mac: &String = &args[1];
    let _ = print_batt(mac).await;
}

async fn read_char(char: &Characteristic) -> u8 {
    char.read().await.unwrap().get(0).unwrap().clone()
}

async fn print_batt(mac: &String) -> bluer::Result<()> {
    let adapter = Session::new().await?.default_adapter().await?;
    let address: Address = Address::from_str(&mac.as_str()).unwrap();

    let device = adapter.device(address).unwrap();
    let _ = device.connect().await;

    let mut levels: [u8; 2] = [0; 2];
    for service in device.services().await? {
        let uuid = service.uuid().await?;
        if uuid.to_string() == SERVICE_UUID {
            for char in service.characteristics().await? {
                match char.id() {
                    17 => levels[0] = read_char(&char).await,
                    22 => levels[1] = read_char(&char).await,
                    _ => (),
                }
            }
        }
    }

    println!("[{},{}]", levels[0], levels[1]);
    Ok(())
}
