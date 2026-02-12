use simpleport::{SimpleRead, SimpleWrite};
use ufmt::uwriteln;

use crate::drivers::DriverMut;
use crate::drivers::regs::register;
use crate::drivers::uart::Serial;
use crate::err::USBError;

const TYPE_BULK: usize = 2;

const USB_BASE: usize = 0x01500000;

register!(gahbcfg, USB_BASE + 0x008, [
    bit: NPTXFEMPLVL, offset: 5, width: 1;
]);

register!(gintsts, USB_BASE + 0x014, [
    bit: RXFLVL, offset: 4;
    bit: OEPINT, offset: 19;
]);

register!(grxstsp, USB_BASE + 0x020);

register!(dctl, USB_BASE + 0x804, [
    bit: SOFT_RESET1, offset: 8;
    bit: SOFT_RESET2, offset: 10;
]);

register!(dsts, USB_BASE + 0x808);

register!(doepctl1, USB_BASE + 0xb20, [
    field: MPS, offset: 0, width: 11;
    bit: USB_ACTIVE_EP, offset: 15;
    field: EP_TYPE, offset: 18, width: 2;
    bit: CNAK, offset: 26;
    bit: EPENA, offset: 31;
]);

register!(diepint1, USB_BASE + 0x928, [
    bit: XFERCOMPL, offset: 0;
]);

register!(dieptsiz1, USB_BASE + 0x930, [
    field: XFERSIZE, offset: 0, width: 7;
    field: PKTCNT, offset: 19, width: 2;
]);

register!(diepctl1, USB_BASE + 0x920, [
    field: MPS, offset: 0, width: 11;
    bit: USB_ACTIVE_EP, offset: 15;
    field: EP_TYPE, offset: 18, width: 2;
    field: TXFNUM, offset: 22, width: 4;
    bit: CNAK, offset: 26;
    bit: EPENA, offset: 31;
]);

register!(doepint1, USB_BASE + 0xb28, [
    bit: XFERCOMPL, offset: 0;
    bit: SETUP_COMPLETED, offset: 3;
]);

register!(doeptsiz1, USB_BASE + 0xb30, [
    field: SPEED, offset: 0, width: 2;
    bit: PKTCNT, offset: 19;
]);

register!(rx_fifo, USB_BASE + 0x1000);
register!(tx_fifo, USB_BASE + 0x2000);

pub struct Usb {
    rx_buf: [u8; 512],
    rx_ptr: usize,
    rx_cnt: usize,
    ep_mps: usize,
}

impl DriverMut for Usb {
    unsafe fn init(&mut self) {
        unsafe {
            gahbcfg::read_modify_write(|r| {
                use gahbcfg::*;

                if r.is_set_bit(NPTXFEMPLVL) {
                    r.clear_bit(NPTXFEMPLVL);
                }
            });

            dctl::read_modify_write(|r| {
                r.set_bit(dctl::SOFT_RESET1).set_bit(dctl::SOFT_RESET2);
            });

            let speed = (dsts::read() >> 1) & 0x3;

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

            doeptsiz1::read_modify_write(|r| {
                use doeptsiz1::*;

                r.set_bit(PKTCNT).set_field(SPEED, speed);
            });

            doepctl1::new_scope(|r| {
                use doepctl1::*;

                r.set_bit(EPENA)
                    .set_bit(CNAK)
                    .set_field(EP_TYPE, 2)
                    .set_bit(USB_ACTIVE_EP)
                    .set_field(MPS, self.ep_mps);
            });

            diepctl1::new_scope(|r| {
                use diepctl1::*;

                r.set_bit(CNAK)
                    .set_field(TXFNUM, 1)
                    .set_field(EP_TYPE, 2)
                    .set_bit(USB_ACTIVE_EP)
                    .set_field(MPS, self.ep_mps);
            });
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

    unsafe fn read_u8(&mut self) -> Result<u8, USBError> {
        let mut hang_ctr = 0;
        loop {
            if self.rx_ptr < self.rx_cnt {
                let b = self.rx_buf[self.rx_ptr];
                self.rx_ptr += 1;
                break Ok(b);
            }

            let status = unsafe { gintsts::read() };

            if status.is_set_bit(gintsts::RXFLVL) {
                let rx_status = unsafe { grxstsp::read() };
                let packet_status = (rx_status >> 17) & 0xf;
                let byte_count = ((rx_status >> 4) & 0x7ff) as usize;

                // 2 = OUT received, 6 = Setup received
                if (packet_status == 2 || packet_status == 6) && byte_count > 0 {
                    self.rx_ptr = 0;
                    self.rx_cnt = 0;

                    let words = (byte_count + 3) / 4;
                    for _ in 0..words {
                        let val = unsafe { rx_fifo::read() };
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

            if status.is_set_bit(gintsts::OEPINT) {
                unsafe {
                    doepint1::read_modify_write(|r| {
                        use doepint1::*;

                        doepint1::write_raw(r.raw());

                        if r.is_set_bit(XFERCOMPL) || r.is_set_bit(SETUP_COMPLETED) {
                            doeptsiz1::read_modify_write(|r| {
                                r.set_bit(doeptsiz1::PKTCNT)
                                    .set_field(doeptsiz1::SPEED, self.ep_mps);
                            });

                            doepctl1::new_scope(|r| {
                                use doepctl1::*;

                                r.set_bit(EPENA)
                                    .set_bit(CNAK)
                                    .set_field(EP_TYPE, TYPE_BULK)
                                    .set_bit(USB_ACTIVE_EP)
                                    .set_field(MPS, self.ep_mps);
                            });
                        }
                    });
                }
            }

            hang_ctr += 1;
            if hang_ctr > 1_000_000 {
                break Err(USBError::Timeout);
            }
        }
    }

    unsafe fn write_u8(&mut self, b: u8) -> Result<(), USBError> {
        unsafe {
            dieptsiz1::new_scope(|r| {
                use dieptsiz1::*;

                r.set_field(PKTCNT, 1).set_field(XFERSIZE, 1);
            });

            diepctl1::new_scope(|r| {
                use diepctl1::*;

                r.set_bit(EPENA)
                    .set_bit(CNAK)
                    .set_field(EP_TYPE, TYPE_BULK)
                    .set_bit(USB_ACTIVE_EP)
                    .set_field(MPS, self.ep_mps);
            });

            tx_fifo::write(b as usize);

            let mut timeout = 10_000_000;
            loop {
                let intr = diepint1::read();
                if intr.is_set_bit(diepint1::XFERCOMPL) {
                    diepint1::write_raw(1);
                    break Ok(());
                }

                timeout -= 1;
                if timeout == 0 {
                    break Err(USBError::Timeout);
                }
            }
        }
    }
}

impl SimpleRead for Usb {
    type Error = USBError;

    fn read(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        for i in 0..buf.len() {
            buf[i] = unsafe { self.read_u8()? };
        }

        Ok(())
    }
}

impl SimpleWrite for Usb {
    type Error = USBError;

    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        for b in buf {
            unsafe { self.write_u8(*b)? };
        }

        Ok(())
    }
}
