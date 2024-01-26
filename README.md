# :radio: Remote ID Module

Remote ID is a form of digital identification for drones (>= 250g) operating in the European Union.

The objective of this module is to communicate Remote ID information to the FlyArrow network[^flyarrow].

[^flyarrow]: [FlyArrow B.V.](https://github.com/FlyArrow-BV) is an open-source software developer seeking certification as a U-space service provider.

This module can be connected to a drone via CAN. The drone platform will communicate formatted telemetry data over the bus. The module will convert the data into remote ID format and transmit over bluetooth and 4G/5G.

"Direct" (a.k.a. "broadcast") Remote ID is transmitted over Bluetooth/WiFi as advertisement packets containing remote ID data. "Network" remote ID is transmitted over cellular networks to the cloud storage of a U-space[^uspace] service provider (USSP).

[^uspace]: :eu: *"U-space is a set of specific services and procedures designed to ensure safe and efficient access to airspace for a large number of drones, and which are based on high levels of digitalisation and automation. The purpose of U-space is therefore to achieve automated UAS management and integration, allowing for a large series of operations, many of them even simultaneous, and all of this in harmonious coexistence with the current ATM system."* - [ECAC CEAC](https://www.ecac-ceac.org/activities/unmanned-aircraft-systems/uas-bulletin/22-uas-bulletin/504-uas-bulletin-2-what-is-u-space)

## :wrench: Installation

```bash
rustup update stable
rustup target add thumbv7em-none-eabihf

# install probe
# https://probe.rs/
curl -LsSf https://github.com/probe-rs/probe-rs/releases/download/v0.22.0/probe-rs-installer.sh | sh
source $HOME/.cargo/env
```

## Hardware

### Mk.0

Targeting KPN [M2M](https://m2m.kpn.com/en) or [LTE-M](https://www.kpn.com/zakelijk/internet-of-things/en/ltem-connectivity.htm) connectivity.

*Note: TBD. Options Provided.*

| Item | Make | Model(s) | Notes
| --- | ---- | --- | --- |
| *Microcontroller* | NXP[^nxp] | [i.MX RT1020](https://www.nxp.com/docs/en/fact-sheet/IMXRTPORTFS.pdf) | [Rust embedded-hal support](https://github.com/imxrt-rs/imxrt-hal) <br>2x FlexCAN controllers<br>500MHz
| | NXP | [KW45](https://www.nxp.com/products/wireless-connectivity/bluetooth-low-energy/kw45-32-bit-bluetooth-5-3-long-range-mcus-with-can-fd-and-lin-bus-options-arm-cortex-m33-core:KW45) | Integrated Bluetooth<br>1x FlexCAN Controller<br>96MHz |
| *WiFi/Bluetooth Module* | u-blox[^ublox] | [NINA-W101-00B](https://www.tme.eu/nl/en/details/nina-w101-00b/iot-wifi-bluetooth-modules/u-blox/) | Or NXP KW45 with integrated bluetooth
| *4G LTE Module* | u-blox | [SARA-R510S-01B](https://www.tme.eu/nl/en/details/sara-r510s-01b/m2m-gprs-lte-5g-modules/u-blox/) | 5G Ready |
| | Sequans[^sequans] | [Monarch 2 GM02S](https://sequans.com/products/monarch-2-gm02s/) | LTE-M

[^nxp]: :netherlands: *"NXP Semiconductors N.V. is a Dutch semiconductor designer and manufacturer with headquarters in Eindhoven, Netherlands."* - [Wikipedia](https://en.wikipedia.org/wiki/NXP_Semiconductors)
[^ublox]: :switzerland: *"u-blox is a Swiss company that creates wireless semiconductors and modules for consumer, automotive and industrial markets. They operate as a fabless IC and design house.* - [Wikipedia](https://en.wikipedia.org/wiki/U-blox)
[^sequans]: :fr: *"Sequans Communications is a fabless semiconductor company that designs, develops, and markets integrated circuits and modules for 4G and 5G cellular IoT devices. The company is based in Paris, France with offices in the United States, United Kingdom, Israel, Hong Kong, Singapore, Taiwan, South Korea, Finland and China."* - [Wikipedia](https://en.wikipedia.org/wiki/Sequans)

## :sunrise_over_mountains: Roadmap

### :aerial_tramway: MK.0 (Concept)

Breadboard:
- Direct Remote ID over WiFi/Bluetooth
- Network Remote ID over 4G w/ data-only physical SIM card
- CAN Interface

### :mountain_cableway: MK.1 (Prototype)

PCB:
- All features of MK.0
- Aviation connectors

### :suspension_railway: MK.2 (Expansion)

- 5G
- Redundant CAN interface
- eSIM
- USB Debug
