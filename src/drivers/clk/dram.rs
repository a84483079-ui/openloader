use derive_ctor::ctor;

use crate::drivers::{
    Driver,
    clk::{Mux, mux, parents, soc::MATRIX_BASE},
    dram::DramSize,
};

const MATRIX_DDR_SEL: usize = MATRIX_BASE + 0x50;

#[derive(ctor)]
pub struct DramClk {
    size: DramSize,
}

impl Driver for DramClk {
    unsafe fn init(&self) {
        unsafe {
            match self.size {
                DramSize::Dram32M => DDRTopMux::set_parent(DDRTopClk::Clk200m),
                _ => DDRTopMux::set_parent(DDRTopClk::Clk156m),
            }
        }
    }
}

parents!(DDRTopClk: Clk156m, Clk200m, Clk104m, Clk78m);
mux!(DDRTopMux, MATRIX_DDR_SEL, 0, 2, DDRTopClk);
