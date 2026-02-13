# Openloader

Open source implementation of the blob running on the Cortex M0.

## Status
### Working
 - [X] PLL, very basic clocks
 - [X] UART
 - [X] Efuse
 - [X] DRAM init (tested on 32, 64 and 128 MB)
 - [X] USB
 - [X] Download protocol for stage 2

### TODO
 - [ ] NAND/NOR
 - [ ] microSD (probably)

### Not planned
- Pinctrl, non-basic clocks

## Credits
- [stefand](https://github.com/stefand) - lots of reverse engineering for this SoC; testing (64 MB)
- [Mio-sha512](https://github.com/Mio-sha512) - DRAM & USB & protocol drivers; testing (32 MB)
- [jschwartzenberg](https://github.com/jschwartzenberg) - testing (64MB)
- [exp-3](https://github.com/exp-3) - testing (32MB)
