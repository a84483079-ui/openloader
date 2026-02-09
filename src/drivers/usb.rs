use simpleport::{SimpleRead, SimpleWrite};
use ufmt::uwriteln;

use crate::drivers::uart::Serial;
use crate::drivers::{DriverMut, bit, readl, shift, writel};

const TYPE_BULK: usize = 2;

const USB_BASE: usize = 0x01500000;

const USB_GAHBCFG: usize = USB_BASE + 0x008;
const FLAG_NPTXFEMPLVL: usize = bit(5);

const USB_GUSBCFG: usize = USB_BASE + 0x00c;
const USB_GINTSTS: usize = USB_BASE + 0x014;
const FLAG_RXFLVL: usize = bit(4);
const FLAG_OEPINT: usize = bit(19);

const USB_GRXSTSP: usize = USB_BASE + 0x020;
const USB_DCTL: usize = USB_BASE + 0x804;
const FLAG_SOFT_RESET1: usize = bit(8);
const FLAG_SOFT_RESET2: usize = bit(10);

const USB_DSTS: usize = USB_BASE + 0x808;
const USB_DTXFSIZ1: usize = USB_BASE + 0x104;

const USB_DIEPCTL1: usize = USB_BASE + 0x920;
shift!(SHIFT_TXFNUM, 22);

const USB_DIEPINT1: usize = USB_BASE + 0x928;
const USB_DIEPTSIZ1: usize = USB_BASE + 0x930;
shift!(SHIFT_XFER_COUNT, 0);

const USB_DOEPCTL1: usize = USB_BASE + 0xb20;
const FLAG_USB_ACTIVE_EP: usize = bit(15);
shift!(SHIFT_EP_TYPE, 18);
const FLAG_CNAK: usize = bit(26);
const FLAG_EPENA: usize = bit(31);

const USB_DOEPINT1: usize = USB_BASE + 0xb28;
const FLAG_XFERCOMPL: usize = bit(0);
const FLAG_SETUP_COMPLETED: usize = bit(3);

const USB_DOEPTSIZ1: usize = USB_BASE + 0xb30;
shift!(SHIFT_PKTCNT, 19);

const USB_RX_FIFO: usize = USB_BASE + 0x1000;
const USB_TX_FIFO: usize = USB_BASE + 0x2000;

pub struct Usb {
    rx_buf: [u8; 512],
    rx_ptr: usize,
    rx_cnt: usize,
    ep_mps: usize,
}

impl DriverMut for Usb {
    unsafe fn init(&mut self) {
        unsafe {
            let mut gahbcfg = readl(USB_GAHBCFG);
            let gusbcfg = readl(USB_GUSBCFG);
            let dtxfsiz1 = readl(USB_DTXFSIZ1);

            uwriteln!(
                &mut Serial,
                "USB: Config - GAHBCFG=0x{:x} GUSBCFG=0x{:x} DTXFSIZ1=0x{:x}",
                gahbcfg,
                gusbcfg,
                dtxfsiz1
            );

            if (gahbcfg & FLAG_NPTXFEMPLVL) != 0 {
                gahbcfg &= !FLAG_NPTXFEMPLVL;
                writel(USB_GAHBCFG, gahbcfg);
            }

            let mut dctl = readl(USB_DCTL);
            if (dctl & FLAG_SOFT_RESET1) != 0 || (dctl & FLAG_SOFT_RESET2) != 0 {
                dctl |= FLAG_SOFT_RESET1 | FLAG_SOFT_RESET2;
                writel(USB_DCTL, dctl);
            }

            let dsts = readl(USB_DSTS);
            let speed = (dsts >> 1) & 0x3;

            self.ep_mps = if speed == 0 {
                uwriteln!(
                    &mut Serial,
                    "USB: Using USB High Speed Mode (MPS=512 bytes)"
                );
                512
            } else {
                uwriteln!(&mut Serial, "USB: Using USB Full Speed Mode (MPS=64 bytes)");
                64
            };

            writel(USB_DOEPTSIZ1, SHIFT_PKTCNT(1) | self.ep_mps);
            writel(
                USB_DOEPCTL1,
                FLAG_EPENA
                    | FLAG_CNAK
                    | SHIFT_EP_TYPE(TYPE_BULK)
                    | FLAG_USB_ACTIVE_EP
                    | self.ep_mps,
            );

            writel(
                USB_DIEPCTL1,
                FLAG_CNAK
                    | SHIFT_TXFNUM(1)
                    | SHIFT_EP_TYPE(TYPE_BULK)
                    | FLAG_USB_ACTIVE_EP
                    | self.ep_mps,
            );
        }
    }
}

impl Usb {
    pub fn new() -> Self {
        Self {
            rx_buf: [0; 512],
            rx_ptr: 0,
            rx_cnt: 0,
            ep_mps: 0,
        }
    }

    unsafe fn read_u8(&mut self) -> u8 {
        let mut hang_ctr = 0;
        loop {
            if self.rx_ptr < self.rx_cnt {
                let b = self.rx_buf[self.rx_ptr];
                self.rx_ptr += 1;
                return b;
            }

            let status = unsafe { readl(USB_GINTSTS) };

            if (status & FLAG_RXFLVL) != 0 {
                let rx_status = unsafe { readl(USB_GRXSTSP) };
                let packet_status = (rx_status >> 17) & 0xf;
                let byte_count = ((rx_status >> 4) & 0x7ff) as usize;

                // 2 = OUT received, 6 = Setup received
                if (packet_status == 2 || packet_status == 6) && byte_count > 0 {
                    self.rx_ptr = 0;
                    self.rx_cnt = 0;

                    let words = (byte_count + 3) / 4;
                    for _ in 0..words {
                        let val = unsafe { readl(USB_RX_FIFO) };
                        let bytes = val.to_le_bytes();
                        for k in 0..4 {
                            if self.rx_cnt < 512 && self.rx_cnt < byte_count {
                                self.rx_buf[self.rx_cnt] = bytes[k];
                                self.rx_cnt += 1;
                            }
                        }
                    }
                }
            }

            if (status & FLAG_OEPINT) != 0 {
                let doepint = unsafe { readl(USB_DOEPINT1) };
                unsafe { writel(USB_DOEPINT1, doepint) };

                if (doepint & FLAG_XFERCOMPL) != 0 || (doepint & FLAG_SETUP_COMPLETED) != 0 {
                    unsafe {
                        writel(USB_DOEPTSIZ1, FLAG_OEPINT | (self.ep_mps << 0));
                        writel(
                            USB_DOEPCTL1,
                            FLAG_EPENA
                                | FLAG_CNAK
                                | SHIFT_EP_TYPE(TYPE_BULK)
                                | FLAG_USB_ACTIVE_EP
                                | self.ep_mps,
                        );
                    }
                }
            }

            hang_ctr += 1;
            if hang_ctr > 1_000_000 {
                let doepctl = unsafe { readl(USB_DOEPCTL1) };
                if (doepctl & FLAG_EPENA) == 0 {
                    unsafe {
                        writel(USB_DOEPTSIZ1, FLAG_OEPINT | (self.ep_mps << 0));
                        writel(
                            USB_DOEPCTL1,
                            FLAG_EPENA
                                | FLAG_CNAK
                                | SHIFT_EP_TYPE(TYPE_BULK)
                                | FLAG_USB_ACTIVE_EP
                                | self.ep_mps,
                        );
                    }
                }
                hang_ctr = 0;
            }
        }
    }

    unsafe fn write_u8(&mut self, b: u8) {
        unsafe {
            writel(USB_DIEPTSIZ1, SHIFT_PKTCNT(1) | SHIFT_XFER_COUNT(1));

            let val = FLAG_EPENA
                | FLAG_CNAK
                | SHIFT_EP_TYPE(TYPE_BULK)
                | FLAG_USB_ACTIVE_EP
                | self.ep_mps;
            writel(USB_DIEPCTL1, val);

            writel(USB_TX_FIFO, b as usize);

            let mut timeout = 10_000_000;
            loop {
                let intr = readl(USB_DIEPINT1);
                if (intr & FLAG_XFERCOMPL) != 0 {
                    writel(USB_DIEPINT1, 1);
                    break;
                }

                timeout -= 1;
                if timeout == 0 {
                    uwriteln!(&mut Serial, "USB: Transmit timeout");
                    break;
                }
            }
        }
    }
}

impl SimpleRead for Usb {
    fn read(&mut self, buf: &mut [u8]) -> simpleport::Result<()> {
        for i in 0..buf.len() {
            buf[i] = unsafe { self.read_u8() };
        }

        Ok(())
    }
}

impl SimpleWrite for Usb {
    fn write(&mut self, buf: &[u8]) -> simpleport::Result<()> {
        for b in buf {
            unsafe { self.write_u8(*b) };
        }

        Ok(())
    }
}
