#include <modbuspp.h>

#include <string>

int main(void) {
  std::string port("/dev/ttyUSB0");

  Modbus::Master mb (Modbus::Rtu, port, "9600N1");
  return 0;
}
