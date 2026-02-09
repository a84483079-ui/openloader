use derive_ctor::ctor;

use crate::drivers::{
    Driver, bit,
    delay::nsdelay,
    dram::{DramSize, MATRIX_DDR_RESET},
    dram_control::{
        DDR_CONTROL_DFIMISC, DDR_CONTROL_PCTRL_0, DDR_CONTROL_PCTRL_1, DDR_CONTROL_PCTRL_2,
        DDR_CONTROL_PCTRL_3, DDR_CONTROL_SWCTL,
    },
    readl, writel,
};

const DDR_PHY_BASE: usize = 0x154000;
const DDR_PHY_DRAM_WIDTH: usize = DDR_PHY_BASE + 0x000;
const FLAG_16BIT: usize = 0x3f;

const DDR_PHY_DRAM_TYPE: usize = DDR_PHY_BASE + 0x004;
const FLAG_LPDDR2: usize = 3;
const FLAG_TYPE_RESERVED: usize = bit(2);

const DDR_PHY_TRAINING_CTRL: usize = DDR_PHY_BASE + 0x008;

const DDR_PHY_DRAM_RLRW: usize = DDR_PHY_BASE + 0x02c;
const DDR_PHY_DRAM_WLRW: usize = DDR_PHY_BASE + 0x030;
const DDR_PHY_FBDIV: usize = DDR_PHY_BASE + 0x3b0;
const FLAG_FBDIV: usize = 0x10;

const DDR_PHY_PLLOUT: usize = DDR_PHY_BASE + 0x3b4;
const FLAG_PLL_PULL_DOWN: usize = bit(1);
const FLAG_PHY_PLLOUT_RESERVED: usize = bit(3);
const FLAG_PLL_CLOCK_OUTPUT_ENABLE: usize = bit(4);

const DDR_PHY_DIV: usize = DDR_PHY_BASE + 0x3b8;
const FLAG_PREDIV: usize = bit(1);
const FLAG_POSTDIV: usize = bit(5);

const DDR_PHY_CMD_DRIVE_STRENGTH: usize = DDR_PHY_BASE + 0x44;
const DDR_PHY_CK_DRIVE_STRENGTH: usize = DDR_PHY_BASE + 0x58;
const DDR_PHY_CK_PULL_UP: usize = DDR_PHY_BASE + 0x64;
const DDR_PHY_CMD_PULL_UP: usize = DDR_PHY_BASE + 0x68;

const DDR_PHY_A_DQ0_7_PULL_DOWN_DS: usize = DDR_PHY_BASE + 0x080;
const DDR_PHY_A_DQ0_7_PULL_DOWN_ODT: usize = DDR_PHY_BASE + 0x084;
const DDR_PHY_A_DQ0_7_PULL_UP_DS: usize = DDR_PHY_BASE + 0x0b8;
const DDR_PHY_A_DQ0_7_PULL_UP_ODT: usize = DDR_PHY_BASE + 0x0bc;

const DDR_PHY_A_DQ8_15_PULL_DOWN_DS: usize = DDR_PHY_BASE + 0x0c0;
const DDR_PHY_A_DQ8_15_PULL_DOWN_ODT: usize = DDR_PHY_BASE + 0x0c4;
const DDR_PHY_A_DQ8_15_PULL_UP_DS: usize = DDR_PHY_BASE + 0x0f8;
const DDR_PHY_A_DQ8_15_PULL_UP_ODT: usize = DDR_PHY_BASE + 0x0fc;

const DDR_PHY_B_DQ0_7_PULL_DOWN_DS: usize = DDR_PHY_BASE + 0x100;
const DDR_PHY_B_DQ0_7_PULL_DOWN_ODT: usize = DDR_PHY_BASE + 0x104;
const DDR_PHY_B_DQ0_7_PULL_UP_DS: usize = DDR_PHY_BASE + 0x138;
const DDR_PHY_B_DQ0_7_PULL_UP_ODT: usize = DDR_PHY_BASE + 0x13c;

const DDR_PHY_B_DQ8_15_PULL_DOWN_DS: usize = DDR_PHY_BASE + 0x140;
const DDR_PHY_B_DQ8_15_PULL_DOWN_ODT: usize = DDR_PHY_BASE + 0x144;
const DDR_PHY_B_DQ8_15_PULL_UP_DS: usize = DDR_PHY_BASE + 0x178;
const DDR_PHY_B_DQ8_15_PULL_UP_ODT: usize = DDR_PHY_BASE + 0x17c;

const DDR_PHY_TRAINING_RESULT_0: usize = DDR_PHY_BASE + 0x3ec;
const DDR_PHY_TRAINING_RESULT_1: usize = DDR_PHY_BASE + 0x3f0;
const DDR_PHY_TRAINING_RESULT_2: usize = DDR_PHY_BASE + 0x3f4;
const DDR_PHY_TRAINING_RESULT_3: usize = DDR_PHY_BASE + 0x3f8;

const TRAINING_MAX_ATTEMPTS: u32 = 100;
const TRAINING_TOLERANCE: usize = 4;

#[inline(always)]
fn abs_diff(a: usize, b: usize) -> usize {
    if a > b { a - b } else { b - a }
}

#[derive(ctor)]
pub struct DramPhy {
    size: DramSize,
}

impl Driver for DramPhy {
    unsafe fn init(&self) {
        unsafe {
            if self.size.is_dram_32_m() {
                writel(DDR_PHY_DRAM_WIDTH, FLAG_16BIT);
            }

            writel(DDR_PHY_DRAM_TYPE, FLAG_LPDDR2 | FLAG_TYPE_RESERVED);
            writel(DDR_PHY_DRAM_RLRW, 8 << 4);
            writel(DDR_PHY_DRAM_WLRW, 4);
            writel(DDR_PHY_FBDIV, FLAG_FBDIV);
            writel(
                DDR_PHY_PLLOUT,
                FLAG_PLL_PULL_DOWN | FLAG_PHY_PLLOUT_RESERVED | FLAG_PLL_CLOCK_OUTPUT_ENABLE,
            );
            writel(DDR_PHY_DIV, FLAG_PREDIV | FLAG_POSTDIV);

            writel(DDR_PHY_CMD_DRIVE_STRENGTH, 0x08);
            writel(DDR_PHY_CK_DRIVE_STRENGTH, 0x08);
            writel(DDR_PHY_CK_PULL_UP, 0x08);
            writel(DDR_PHY_CMD_PULL_UP, 0x08);

            writel(DDR_PHY_A_DQ0_7_PULL_DOWN_DS, 0x08);
            writel(DDR_PHY_A_DQ0_7_PULL_DOWN_ODT, 0x04);
            writel(DDR_PHY_A_DQ0_7_PULL_UP_DS, 0x88);
            writel(DDR_PHY_A_DQ0_7_PULL_UP_ODT, 0x84);

            writel(DDR_PHY_A_DQ8_15_PULL_DOWN_DS, 0x88);
            writel(DDR_PHY_A_DQ8_15_PULL_DOWN_ODT, 0x84);
            writel(DDR_PHY_A_DQ8_15_PULL_UP_DS, 0x08);
            writel(DDR_PHY_A_DQ8_15_PULL_UP_ODT, 0x04);

            writel(DDR_PHY_B_DQ0_7_PULL_DOWN_DS, 0x08);
            writel(DDR_PHY_B_DQ0_7_PULL_DOWN_ODT, 0x04);
            writel(DDR_PHY_B_DQ0_7_PULL_UP_DS, 0x88);
            writel(DDR_PHY_B_DQ0_7_PULL_UP_ODT, 0x84);

            writel(DDR_PHY_B_DQ8_15_PULL_DOWN_DS, 0x88);
            writel(DDR_PHY_B_DQ8_15_PULL_DOWN_ODT, 0x84);
            writel(DDR_PHY_B_DQ8_15_PULL_UP_DS, 0x08);
            writel(DDR_PHY_B_DQ8_15_PULL_UP_ODT, 0x24);
        }
    }
}

impl DramPhy {
    unsafe fn do_train(&self) {
        if self.size.is_dram_32_m() {
            for _ in 0..TRAINING_MAX_ATTEMPTS {
                unsafe {
                    writel(DDR_PHY_TRAINING_CTRL, 0x01);
                    nsdelay(200000);
                    writel(DDR_PHY_TRAINING_CTRL, 0x00);
                    nsdelay(200000);
                }

                let fb = unsafe { readl(DDR_PHY_TRAINING_RESULT_0) };
                let fc = unsafe { readl(DDR_PHY_TRAINING_RESULT_1) };

                if abs_diff(fb, fc) <= TRAINING_TOLERANCE && (fb & 0x80) == 0 {
                    break;
                }
            }
        } else {
            for _ in 0..TRAINING_MAX_ATTEMPTS {
                unsafe {
                    writel(DDR_PHY_TRAINING_CTRL, 0x01);
                    nsdelay(200000);
                    writel(DDR_PHY_TRAINING_CTRL, 0x00);
                    nsdelay(200000);
                }

                let fb = unsafe { readl(DDR_PHY_TRAINING_RESULT_0) };
                let fc = unsafe { readl(DDR_PHY_TRAINING_RESULT_1) };
                let fd = unsafe { readl(DDR_PHY_TRAINING_RESULT_2) };
                let fe = unsafe { readl(DDR_PHY_TRAINING_RESULT_3) };

                let training_ok = abs_diff(fb, fc) <= TRAINING_TOLERANCE
                    && abs_diff(fb, fd) <= TRAINING_TOLERANCE
                    && abs_diff(fb, fe) <= TRAINING_TOLERANCE
                    && abs_diff(fc, fd) <= TRAINING_TOLERANCE
                    && abs_diff(fc, fe) <= TRAINING_TOLERANCE
                    && abs_diff(fd, fe) <= TRAINING_TOLERANCE
                    && (fb & 0x80) == 0;

                if training_ok {
                    break;
                }
            }
        }
    }

    pub unsafe fn train(&self) {
        unsafe {
            nsdelay(200000);

            writel(MATRIX_DDR_RESET, 0x0affffc0);
            nsdelay(200000);

            writel(DDR_CONTROL_SWCTL, 0x00);
            nsdelay(200000);

            writel(DDR_PHY_PLLOUT, 0x18);
            nsdelay(200000);

            writel(DDR_PHY_B_DQ8_15_PULL_UP_ODT, 0x04);
            nsdelay(200000);

            writel(DDR_CONTROL_DFIMISC, 0x01);
            nsdelay(200000);
            writel(DDR_CONTROL_DFIMISC, 0x00);

            self.do_train();

            writel(DDR_CONTROL_PCTRL_0, 0x01);
            writel(DDR_CONTROL_PCTRL_1, 0x01);
            writel(DDR_CONTROL_PCTRL_2, 0x01);
            writel(DDR_CONTROL_PCTRL_3, 0x01);
        }
    }
}
