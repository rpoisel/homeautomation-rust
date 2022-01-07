#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use tokio::time::{self, sleep, Duration};
    use tokio_modbus::prelude::*;
    use tokio_serial::SerialStream;

    let tty_path = "/dev/ttyUSB0";
    let slave = Slave(0x01);

    let builder = tokio_serial::new(tty_path, 9600);
    let port = SerialStream::open(&builder).unwrap();

    let mut ctx_do = rtu::connect_slave(port, slave).await?;
    let mut interval = time::interval(Duration::from_millis(500));

    loop {
        for cur in 0..8 {
            // println!("Reading a sensor value");
            // let rsp = ctx_di.read_holding_registers(0x082B, 2).await?;
            // println!("Sensor value is: {:?}", rsp);
            ctx_do.write_single_coil(cur, true).await?;
            sleep(Duration::from_millis(250)).await;
            ctx_do.write_single_coil(cur, false).await?;

            interval.tick().await;
        }
    }
}
