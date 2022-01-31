#!/usr/bin/python3

"""
Inputs: N4DIH32 PNP Input Module
Outputs: RS485 8 Channel Relay Controller, ModBus RTU
"""

from pymodbus.client.sync import ModbusSerialClient as ModbusClient
import time

import logging
FORMAT = ('%(asctime)-15s %(threadName)-15s '
          '%(levelname)-8s %(module)-15s:%(lineno)-8s %(message)s')
logging.basicConfig(format=FORMAT)
log = logging.getLogger()
log.setLevel(logging.DEBUG)

DEVICE='/dev/ttyUSB0'

def main():
    client = ModbusClient(method='rtu', port=DEVICE, baudrate=9600, bytesize=8, stopbits=1, parity='N')
    client.connect()

    relay_states = 0
    log.debug("Reading Holding Registers")
    rr = client.read_discrete_inputs(0x0000, 0x0010, unit=1)
    log.debug(rr)
    last_input_states = rr.getBit(0)
    while True:
        log.debug("Reading Holding Registers")
        rr = client.read_discrete_inputs(0x0000, 0x0010, unit=1)
        log.debug(rr)
        if rr.isError():
            continue
        input_states = rr.getBit(0)
        for cur in range(0, 8):
            mask = 0x01 << cur
            if last_input_states & mask == 0 and input_states & mask == mask:
                relay_states ^= mask
                client.write_coil(cur, relay_states & mask == mask, unit=1)

        last_input_states = input_states

        time.sleep(.02)

if __name__ == "__main__":
    main()
