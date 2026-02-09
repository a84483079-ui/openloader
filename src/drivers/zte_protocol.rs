use core::slice;
use derive_ctor::ctor;
use ufmt::uwriteln;

use crate::drivers::{uart::Serial, usb::Usb, writel};
use simpleport::{SimpleRead, SimpleWrite};

const SYNC_FLAG: u8 = 0x5a;
const DOWNLOAD_FLAG: u8 = 0x7a;
const RUN_FLAG: u8 = 0x8a;

const SYNC_ACK: u8 = 0xa5;
const DOWNLOAD_HEADER_ACK: u8 = 0xa1;
const DOWNLOAD_COMPLETE_ACK: u8 = 0xa7;
const RUN_ACK: u8 = 0xa8;

const IRAM1_BASE: usize = 0x100000;
const A53_SUBSYS_CFG: usize = 0x013b138;
const A53_SW_RSTEN: usize = 0xf;

#[derive(ctor)]
pub struct ZteProtocol {
    usb: Usb,
}

impl ZteProtocol {
    pub unsafe fn dispatch(&mut self) -> Result<(), simpleport::err::Error> {
        loop {
            let cmd = self.usb.read_u8()?;

            match cmd {
                SYNC_FLAG => self.usb.write_u8(SYNC_ACK)?,
                DOWNLOAD_FLAG => unsafe {
                    let addr = self.usb.read_u32_be()?;
                    let size = self.usb.read_u32_be()?;

                    self.usb.write_u8(DOWNLOAD_HEADER_ACK)?;

                    self.usb
                        .read(slice::from_raw_parts_mut(addr as *mut u8, size as usize))?;

                    self.usb.write_u8(DOWNLOAD_COMPLETE_ACK)?;
                },
                RUN_FLAG => unsafe {
                    let addr = self.usb.read_u32_be()?;
                    Self::boot_ap(addr as usize);

                    self.usb.write_u8(RUN_ACK)?;

                    break Ok(());
                },
                _ => {
                    uwriteln!(&mut Serial, "Unknown command: {:#x}", cmd);
                }
            }
        }
    }

    unsafe fn boot_ap(uboot_entry: usize) {
        unsafe {
            writel(IRAM1_BASE, 0xe59ff000);
            writel(IRAM1_BASE + 8, uboot_entry);
            writel(A53_SUBSYS_CFG, A53_SW_RSTEN);
        }
    }
}
