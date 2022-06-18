#pragma once

#include <i2c_bus.hpp>

#include <bithelpers.hpp>

#include <cstdint>

namespace HomeAutomation {
namespace IO {
namespace I2C {

// https://datasheets.maximintegrated.com/en/ds/MAX7311.pdf
class MAX7311Output : public HomeAutomation::IO::I2C::OutputModule {
public:
  MAX7311Output(std::uint8_t address)
      : address(address), outputs{}, last_outputs(outputs) {}
  virtual void init(IReadWrite *io) override {
    // Port 1 configuration
    io->write(address, {REGISTER_PORT_1_CONFIGURATION, 0x00});

    io->write(address, {REGISTER_OUTPUT_PORT_1});
    auto bytes = io->read(address, 1);
    if (bytes.empty()) {
      // should not happen
      return;
    }
    outputs = BitHelpers::bitflip(bytes[0]); // bits are inverted
  }

  void setOutputs(std::uint8_t values) { outputs = values; }
  std::uint8_t getOutputs() const { return outputs; }
  void setOutput(std::uint8_t pos, bool value) {
    outputs = BitHelpers::bitset(outputs, pos, value);
  }
  bool getOutput(std::uint8_t pos) const {
    return BitHelpers::bitget(outputs, pos);
  }

  virtual void write(HomeAutomation::IO::I2C::IReadWrite *io) override {
    // writing is only necessary if there is change in outputs states
    if (outputs == last_outputs) {
      return;
    }

    io->write(address, {REGISTER_OUTPUT_PORT_1,
                        BitHelpers::bitflip(outputs)}); // bits are inverted

    last_outputs = outputs;
  }

private:
  static constexpr std::uint8_t const REGISTER_OUTPUT_PORT_1 = 0x02;
  static constexpr std::uint8_t const REGISTER_PORT_1_CONFIGURATION = 0x06;

  std::uint8_t const address;
  std::uint8_t outputs;
  std::uint8_t last_outputs;
};

// https://www.nxp.com/docs/en/data-sheet/PCF8574_PCF8574A.pdf
class PCF8574Input : public HomeAutomation::IO::I2C::InputModule {
public:
  PCF8574Input(std::uint8_t address) : address(address), inputs{0x00} {}
  virtual void init(IReadWrite *io) override { read(io); }
  std::uint8_t getInputs() const { return inputs; };
  bool getInput(std::uint8_t pos) const {
    return BitHelpers::bitget(inputs, pos);
  }

  virtual void read(HomeAutomation::IO::I2C::IReadWrite *io) override {
    auto bytes = io->read(address, 1);

    if (bytes.empty()) {
      // should not happen
      return;
    }
    inputs = BitHelpers::bitflip(bytes[0]); // bits are inverted
  }

private:
  std::uint8_t const address;
  std::uint8_t inputs;
};

} // namespace I2C
} // namespace IO
} // namespace HomeAutomation
