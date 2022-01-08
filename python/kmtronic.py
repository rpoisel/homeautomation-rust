#!/usr/bin/python3

from pymodbus.client.sync import ModbusSerialClient as ModbusClient
import time

import logging
FORMAT = ('%(asctime)-15s %(threadName)-15s '
          '%(levelname)-8s %(module)-15s:%(lineno)-8s %(message)s')
logging.basicConfig(format=FORMAT)
log = logging.getLogger()
log.setLevel(logging.DEBUG)

DEVICE='/dev/ttyUSB0'

def set_relay(client, unit, relay, value):
    log.debug("Writing coil " + str(relay))
    rr = client.write_coil(relay, value, unit=unit)
    log.debug(rr)

def verify_relay(client, unit, relay):
    log.debug("Reading coil " + str(relay))
    rr = client.read_coils(relay, unit=unit)
    log.debug(rr)
    assert rr.isError() == False
    return rr.bits[0]

def test_client():
    client = ModbusClient(method='rtu', port=DEVICE, baudrate=9600, bytesize=8, stopbits=1, parity='N')
    client.connect()

    for relay in range(0, 8):
        set_relay(client, 1, relay, True)
        assert verify_relay(client, 1, relay) == True
        time.sleep(.2)
        set_relay(client, 1, relay, False)
        assert verify_relay(client, 1, relay) == False

if __name__ == "__main__":
    test_client()
