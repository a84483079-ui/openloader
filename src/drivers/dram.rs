use derive_ctor::ctor;
use derive_more::IsVariant;
use ufmt::{uDisplay, uwrite};

use crate::drivers::Driver;
use crate::drivers::clk::{dram::DramClk, soc::MATRIX_BASE};
use crate::drivers::delay::nsdelay;
use crate::drivers::dram_control::DramControl;
use crate::drivers::dram_phy::DramPhy;

use super::writel;

pub(super) const MATRIX_DDR_RESET: usize = MATRIX_BASE + 0x100;

#[derive(Clone, Copy, Default, IsVariant)]
pub enum DramSize {
    #[default]
    Dram32M,
    Dram64M,
    Dram128M,
    Dram256M,
    Dram512M,
}

impl uDisplay for DramSize {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match self {
            Self::Dram32M => uwrite!(f, "32 MB"),
            Self::Dram64M => uwrite!(f, "64 MB"),
            Self::Dram128M => uwrite!(f, "128 MB"),
            Self::Dram256M => uwrite!(f, "256 MB"),
            Self::Dram512M => uwrite!(f, "512 MB"),
        }
    }
}

#[derive(ctor)]
pub struct Dram {
    size: DramSize,
}

impl Driver for Dram {
    unsafe fn init(&self) {
        unsafe {
            writel(MATRIX_DDR_RESET, 0x0affe000);
            nsdelay(200000);
            writel(MATRIX_DDR_RESET, 0x0affe400);
            nsdelay(200000);

            DramClk::new(self.size).init();
            let phy = DramPhy::new(self.size);
            phy.init();

            DramControl::new(self.size).init();

            phy.train();
        }
    }
}
