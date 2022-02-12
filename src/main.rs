#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use tokio::time::{self, Duration};
    use tokio_modbus::prelude::*;
    use tokio_serial::SerialStream;

    let tty_path = "/dev/ttyUSB0";
    let slave_do = Slave(0x01);
    let slave_di = Slave(0x01);

    let builder = tokio_serial::new(tty_path, 9600);
    let port_do = SerialStream::open(&builder).unwrap();
    let port_di = SerialStream::open(&builder).unwrap();

    let mut ctx_do = rtu::connect_slave(port_do, slave_do).await?;
    let mut ctx_di = rtu::connect_slave(port_di, slave_di).await?;
    let mut interval = time::interval(Duration::from_millis(50));

    let mut states_do = Vec::from([false, false, false, false, false, false, false, false]);
    let mut states_prev_di = ctx_di.read_discrete_inputs(0x0000, 8).await?;
    loop {
        let states_di = ctx_di.read_discrete_inputs(0x0000, 8).await?;

        for cur in 0..=7 {
            if !(states_di[cur] && !states_prev_di[cur]) {
                continue;
            }

            states_do[cur] = !states_do[cur];
            ctx_do.write_single_coil(cur as u16, states_do[cur]).await?;
        }
        states_prev_di = states_di;

        interval.tick().await;
    }
}
