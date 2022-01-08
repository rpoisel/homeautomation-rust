#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use tokio::time::{self, Duration};
    use tokio_modbus::prelude::*;
    use tokio_serial::SerialStream;

    let tty_path = "/dev/ttyUSB0";
    let slave_do = Slave(0x01);
    let slave_di = Slave(0x02);

    let builder = tokio_serial::new(tty_path, 9600);
    let port_do = SerialStream::open(&builder).unwrap();
    let port_di = SerialStream::open(&builder).unwrap();

    let mut ctx_do = rtu::connect_slave(port_do, slave_do).await?;
    let mut ctx_di = rtu::connect_slave(port_di, slave_di).await?;
    let mut interval = time::interval(Duration::from_millis(50));

    let mut states_do: u8 = 0;
    let mut states_prev_di = ctx_di.read_holding_registers(0x0080, 2).await?[0] as u8;
    loop {
        let rsp = ctx_di.read_holding_registers(0x0080, 2).await?;
        let di_0_states: u8 = rsp[0] as u8;

        for cur in 0..8 {
            let mask: u8 = 0x01 << cur;

            let val_cur_byte = di_0_states & mask;
            let val_cur = val_cur_byte == mask;
            let val_prev_byte = states_prev_di & mask;
            if !(val_cur && val_prev_byte == 0) {
                continue;
            }

            states_do ^= mask;

            let val_do_byte = states_do & mask;
            let val_do_bit = val_do_byte == mask;
            ctx_do.write_single_coil(cur, val_do_bit).await?;
        }
        states_prev_di = di_0_states;

        interval.tick().await;
    }
}
