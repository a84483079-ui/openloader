use derive_ctor::ctor;

use crate::drivers::{Driver, bit, dram::DramSize, shift, writel};

const DDR_CONTROL_BASE: usize = 0x150000;
const DDR_CONTROL_MSTR: usize = DDR_CONTROL_BASE;
const FLAG_LPDDR2: usize = bit(2);
const FLAG_16BIT: usize = 2 << 11;
const FLAG_BURST_RDWR: usize = 4 << 16;
const FLAG_ACTIVE_RANKS: usize = bit(24);

const DDR_CONTROL_MRCTRL0: usize = DDR_CONTROL_BASE + 0x10;
const FLAG_MR_RANK: usize = 3 << 4;

const DDR_CONTROL_MRCTRL1: usize = DDR_CONTROL_BASE + 0x14;
const DDR_CONTROL_DERATEEN: usize = DDR_CONTROL_BASE + 0x20;
const DDR_CONTROL_DERATEINT: usize = DDR_CONTROL_BASE + 0x24;
const FLAG_READ_INTERVAL: usize = 0x800000;

const DDR_CONTROL_PWRCTL: usize = DDR_CONTROL_BASE + 0x30;
const DDR_CONTROL_PWRTMG: usize = DDR_CONTROL_BASE + 0x34;
const FLAG_POWER_DOWN_TO_X32: usize = 2;
const FLAG_DEEP_POWER_DOWN_TO_X1024: usize = 0x20 << 8;
const FLAG_SELF_REFRESH_TO_X32: usize = 5 << 16;

const DDR_CONTROL_HWLPCTL: usize = DDR_CONTROL_BASE + 0x38;
const FLAG_HW_LOW_POWER: usize = bit(0);
const FLAG_HW_EXIT_IDLE: usize = bit(1);
const FLAG_HW_LOW_POWER_IDLE_X32: usize = 4 << 16;

const DDR_CONTROL_RFSHCTL0: usize = DDR_CONTROL_BASE + 0x50;
const FLAG_REFRESH_TO_X32: usize = bit(16);
const FLAG_REFRESH_MARGIN: usize = 2 << 20;

const DDR_CONTROL_RFSHCTL1: usize = DDR_CONTROL_BASE + 0x54;
const DDR_CONTROL_RFSHCTL2: usize = DDR_CONTROL_BASE + 0x58;
const DDR_CONTROL_RFSHCTL3: usize = DDR_CONTROL_BASE + 0x60;

const DDR_CONTROL_RFSHTMG: usize = DDR_CONTROL_BASE + 0x64;
shift!(SHIFT_RFC_MIN, 0);
shift!(SHIFT_T_RFC_NOM_X32, 16);

const DDR_CONTROL_VALUE_INIT0: usize = DDR_CONTROL_BASE + 0xd0;
const FLAG_PRE_CKE_X1024: usize = 1;
shift!(SHIFT_POST_CKE_X1024, 16);

const DDR_CONTROL_VALUE_INIT1: usize = DDR_CONTROL_BASE + 0xd4;
const DDR_CONTROL_VALUE_INIT2: usize = DDR_CONTROL_BASE + 0xd8;
const FLAG_MIN_STABLE_CLOCK_X1: usize = 5;
shift!(SHIFT_IDLE_AFTER_RESET_X32, 8);

const DDR_CONTROL_VALUE_INIT3: usize = DDR_CONTROL_BASE + 0xdc;
const FLAG_EMR: usize = 6;
shift!(SHIFT_MR, 16);

const DDR_CONTROL_VALUE_INIT4: usize = DDR_CONTROL_BASE + 0xe0;
const FLAG_MR3: usize = 4 << 16; // 3 - 48 ohm, 4 - 60 ohm

const DDR_CONTROL_VALUE_INIT5: usize = DDR_CONTROL_BASE + 0xe4;
const FLAG_MAX_AUTO_INIT_X1024: usize = 2;
shift!(SHIFT_ZQINIT_X32, 16);

const DDR_CONTROL_RANKCTL: usize = DDR_CONTROL_BASE + 0xf4;
const FLAG_MAX_RANK_RD: usize = 0xf;
const FLAG_DIFF_RANK_RD_GAP: usize = 6 << 4;
const FLAG_RANK_WR_GAP: usize = 6 << 8;

const DDR_CONTROL_DRAMTMG0: usize = DDR_CONTROL_BASE + 0x100;
shift!(SHIFT_T_RAS_MIN, 0);
shift!(SHIFT_T_RAS_MAX, 8);
shift!(SHIFT_T_FAW, 16);
shift!(SHIFT_WR2PRE, 24);

const DDR_CONTROL_DRAMTMG1: usize = DDR_CONTROL_BASE + 0x104;
shift!(SHIFT_T_RC, 0);
const FLAG_RD2PRE: usize = 2 << 8;
const FLAG_T_XP: usize = 2 << 16;

const DDR_CONTROL_DRAMTMG2: usize = DDR_CONTROL_BASE + 0x108;
shift!(SHIFT_WR2RD, 0);
const FLAG_RD2WR: usize = 6 << 8;
const FLAG_READ_LATENCY: usize = 4 << 16;
const FLAG_WRITE_LATENCY: usize = 2 << 24;

const DDR_CONTROL_DRAMTMG3: usize = DDR_CONTROL_BASE + 0x10c;
const FLAG_T_MOD: usize = 0;
const FLAG_T_MRD: usize = 4 << 6;
const FLAG_T_MRW: usize = 5 << 20;

const DDR_CONTROL_DRAMTMG4: usize = DDR_CONTROL_BASE + 0x110;
shift!(SHIFT_T_RP, 0);
const FLAG_T_RRD: usize = 2 << 8;
const FLAG_T_CCD: usize = 1 << 16;
shift!(SHIFT_T_RCD, 24);

const DDR_CONTROL_DRAMTMG5: usize = DDR_CONTROL_BASE + 0x114;
const FLAG_T_CKE: usize = 3;
const FLAG_T_CKESR: usize = 3 << 8;
const FLAG_T_CKSRE: usize = 1 << 16;
const FLAG_T_CKSRX: usize = 1 << 24;

const DDR_CONTROL_DRAMTMG6: usize = DDR_CONTROL_BASE + 0x118;
const FLAG_T_CKCSX: usize = 3;
const FLAG_T_CKDPDX: usize = 2 << 16;
const FLAG_T_CKDPDE: usize = 2 << 24;

const DDR_CONTROL_DRAMTMG7: usize = DDR_CONTROL_BASE + 0x11c;
const FLAG_T_CKPDX: usize = 2;
const FLAG_T_CKPDE: usize = 2 << 8;

const DDR_CONTROL_DRAMTMG14: usize = DDR_CONTROL_BASE + 0x138;
shift!(SHIFT_T_XSR, 0);

const DDR_CONTROL_ZQCTL0: usize = DDR_CONTROL_BASE + 0x180;
shift!(SHIFT_T_ZQ_SHORT_NOP, 0);
shift!(SHIFT_T_ZQ_LONG_NOP, 16);
const FLAG_DIS_SRX_ZQCL: usize = bit(30);

const DDR_CONTROL_ZQCTL1: usize = DDR_CONTROL_BASE + 0x184;
const FLAG_T_ZQ_SHORT_INTERVAL_X1024: usize = 0x100;
shift!(SHIFT_T_ZQ_RESET_NOP, 20);

const DDR_CONTROL_ZQCTL2: usize = DDR_CONTROL_BASE + 0x188;
const DDR_CONTROL_DFITMG0: usize = DDR_CONTROL_BASE + 0x190;
const FLAG_DFI_TPHY_WRLAT: usize = 1;
const FLAG_DFI_T_RDDATA_EN: usize = 3 << 16;
const FLAG_DFI_T_CTRL_DELAY: usize = 4 << 24;

const DDR_CONTROL_DFITMG1: usize = DDR_CONTROL_BASE + 0x194;
const FLAG_DFI_T_DRAM_CLK_ENABLE: usize = 4;
const FLAG_DFI_T_DRAM_CLK_DISABLE: usize = 4 << 8;
const FLAG_DFI_T_WRDATA_DELAY: usize = 2 << 16;

const DDR_CONTROL_DFILPCFG0: usize = DDR_CONTROL_BASE + 0x198;
const FLAG_DFI_LP_EN_PD: usize = 1 << 0;
const FLAG_DFI_LP_WAKEUP_PD: usize = bit(4);
const FLAG_DFI_LP_EN_SR: usize = 1 << 8;
const FLAG_DFI_LP_WAKEUP_SR: usize = 1 << 12;
const FLAG_DFI_TLP_RESPONSE: usize = 9 << 24;

const DDR_CONTROL_DFIUPD0: usize = DDR_CONTROL_BASE + 0x1a0;
const FLAG_DFI_T_CTRLUP_MIN: usize = 3;
const FLAG_DFI_T_CTRLUP_MAX: usize = 0x40 << 16;
const FLAG_DIS_AUTO_CTRLUPD: usize = bit(31);

const DDR_CONTROL_DFIUPD1: usize = DDR_CONTROL_BASE + 0x1a4;
const DDR_CONTROL_DFIUPD2: usize = DDR_CONTROL_BASE + 0x1a8;
const FLAG_DFI_PHYUPD_TYPE0: usize = 0x10;
const FLAG_DFI_PHYUPD_TYPE1: usize = 0x10 << 16;
const FLAG_DFI_PHYUPD_EN: usize = bit(31);

pub const DDR_CONTROL_DFIMISC: usize = DDR_CONTROL_BASE + 0x1b0;
const DDR_CONTROL_ADDRMAP0: usize = DDR_CONTROL_BASE + 0x200;
const FLAG_ADDRMAP_CS_BIT0: usize = 0x1f;

const DDR_CONTROL_ADDRMAP1: usize = DDR_CONTROL_BASE + 0x204;
shift!(SHIFT_ADDRMAP_BANK_B0, 0);
shift!(SHIFT_ADDRMAP_BANK_B1, 8);
shift!(SHIFT_ADDRMAP_BANK_B2, 16);

const DDR_CONTROL_ADDRMAP2: usize = DDR_CONTROL_BASE + 0x208;
const DDR_CONTROL_ADDRMAP3: usize = DDR_CONTROL_BASE + 0x20c;
shift!(SHIFT_ADDRMAP_COL_B8, 16);
shift!(SHIFT_ADDRMAP_COL_B9, 24);

const DDR_CONTROL_ADDRMAP4: usize = DDR_CONTROL_BASE + 0x210;
shift!(SHIFT_ADDRMAP_COL_B10, 0);
shift!(SHIFT_ADDRMAP_COL_B11, 8);

const DDR_CONTROL_ADDRMAP5: usize = DDR_CONTROL_BASE + 0x214;
shift!(SHIFT_ADDRMAP_ROW_B0, 0);
shift!(SHIFT_ADDRMAP_ROW_B1, 8);
shift!(SHIFT_ADDRMAP_ROW_B2, 16);
shift!(SHIFT_ADDRMAP_ROW_B11, 24);

const DDR_CONTROL_ADDRMAP6: usize = DDR_CONTROL_BASE + 0x218;
shift!(SHIFT_ADDRMAP_ROW_B12, 0);
shift!(SHIFT_ADDRMAP_ROW_B13, 8);
shift!(SHIFT_ADDRMAP_ROW_B14, 16);
shift!(SHIFT_ADDRMAP_ROW_B15, 24);

const DDR_CONTROL_OTDCFG: usize = DDR_CONTROL_BASE + 0x240;
shift!(SHIFT_ODT_HOLD, 8);
shift!(SHIFT_WR_ODT_HOLD, 24);

const DDR_CONTROL_ODTMAP: usize = DDR_CONTROL_BASE + 0x244;
const DDR_CONTROL_SCHED: usize = DDR_CONTROL_BASE + 0x250;
const FLAG_LOW_PRIO: usize = bit(0);
const FLAG_PAGECLOSE: usize = bit(2);
shift!(SHIFT_LPR_NUM_ENTRIES, 8);

const DDR_CONTROL_SCHED1: usize = DDR_CONTROL_BASE + 0x254;
const DDR_CONTROL_DBG0: usize = DDR_CONTROL_BASE + 0x300;
const DDR_CONTROL_DBG1: usize = DDR_CONTROL_BASE + 0x304;
const DDR_CONTROL_DBGCMD: usize = DDR_CONTROL_BASE + 0x30c;
const DDR_CONTROL_PCCFG: usize = DDR_CONTROL_BASE + 0x400;

pub const DDR_CONTROL_SWCTL: usize = DDR_CONTROL_BASE + 0x320;

pub const DDR_CONTROL_PCTRL_0: usize = DDR_CONTROL_BASE + 0x490;
pub const DDR_CONTROL_PCTRL_1: usize = DDR_CONTROL_BASE + 0x540;
pub const DDR_CONTROL_PCTRL_2: usize = DDR_CONTROL_BASE + 0x5f0;
pub const DDR_CONTROL_PCTRL_3: usize = DDR_CONTROL_BASE + 0x6a0;

const DDR_CONTROL_PCFGR_0: usize = DDR_CONTROL_BASE + 0x404;
shift!(SHIFT_RD_PORT_PRIO, 0);
const FLAG_RD_PORT_AGING_ENABLE: usize = bit(12);

const DDR_CONTROL_PCFGR_1: usize = DDR_CONTROL_BASE + 0x4b4;
const DDR_CONTROL_PCFGR_2: usize = DDR_CONTROL_BASE + 0x564;
const DDR_CONTROL_PCFGR_3: usize = DDR_CONTROL_BASE + 0x614;

const DDR_CONTROL_PCFGW_0: usize = DDR_CONTROL_BASE + 0x408;
shift!(SHIFT_WR_PORT_PRIO, 0);
const FLAG_WR_PORT_AGING_ENABLE: usize = bit(12);

const DDR_CONTROL_PCFGW_1: usize = DDR_CONTROL_BASE + 0x4b8;
const DDR_CONTROL_PCFGW_2: usize = DDR_CONTROL_BASE + 0x568;
const DDR_CONTROL_PCFGW_3: usize = DDR_CONTROL_BASE + 0x618;

const DDR_CONTROL_PCFGQOS0_0: usize = DDR_CONTROL_BASE + 0x494;
shift!(SHIFT_RQOS_MAP_LEVEL1, 0);
shift!(SHIFT_RQOS_MAP_REGION1, 20);

const DDR_CONTROL_PCFGQOS0_1: usize = DDR_CONTROL_BASE + 0x544;
const DDR_CONTROL_PCFGQOS0_2: usize = DDR_CONTROL_BASE + 0x5f4;
const DDR_CONTROL_PCFGQOS0_3: usize = DDR_CONTROL_BASE + 0x6a4;

const DDR_CONTROL_PCFGQOS1_0: usize = DDR_CONTROL_BASE + 0x49c;
const DDR_CONTROL_PCFGQOS1_1: usize = DDR_CONTROL_BASE + 0x54c;
const DDR_CONTROL_PCFGQOS1_2: usize = DDR_CONTROL_BASE + 0x5fc;
const DDR_CONTROL_PCFGQOS1_3: usize = DDR_CONTROL_BASE + 0x6ac;

const DDR_CONTROL_PCFGWQOS0_0: usize = DDR_CONTROL_BASE + 0x498;
const DDR_CONTROL_PCFGWQOS0_1: usize = DDR_CONTROL_BASE + 0x548;
const DDR_CONTROL_PCFGWQOS0_2: usize = DDR_CONTROL_BASE + 0x5f8;
const DDR_CONTROL_PCFGWQOS0_3: usize = DDR_CONTROL_BASE + 0x6a8;

const DDR_CONTROL_PCFGWQOS1_0: usize = DDR_CONTROL_BASE + 0x4a0;
const DDR_CONTROL_PCFGWQOS1_1: usize = DDR_CONTROL_BASE + 0x550;
const DDR_CONTROL_PCFGWQOS1_2: usize = DDR_CONTROL_BASE + 0x600;
const DDR_CONTROL_PCFGWQOS1_3: usize = DDR_CONTROL_BASE + 0x6b0;

const DDR_CONTROL_PERFHPR1: usize = DDR_CONTROL_BASE + 0x25c;
shift!(SHIFT_HPR_MAX_STARVE, 0);
shift!(SHIFT_HPR_XACT_RUN_LENGTH, 24);

const DDR_CONTROL_PERFLPR1: usize = DDR_CONTROL_BASE + 0x264;
shift!(SHIFT_LPR_MAX_STARVE, 0);
shift!(SHIFT_LPR_XACT_RUN_LENGTH, 24);

const DDR_CONTROL_PERFWR1: usize = DDR_CONTROL_BASE + 0x26c;
shift!(SHIFT_W_MAX_STARVE, 0);
shift!(SHIFT_W_XACT_RUN_LENGTH, 24);

const DDR_CONTROL_PERFVPR1: usize = DDR_CONTROL_BASE + 0x274;
const DDR_CONTROL_PERFVPW1: usize = DDR_CONTROL_BASE + 0x278;

#[derive(ctor)]
pub struct DramControl {
    size: DramSize,
}

impl Driver for DramControl {
    unsafe fn init(&self) {
        unsafe {
            if self.size.is_dram_32_m() {
                writel(
                    DDR_CONTROL_MSTR,
                    FLAG_LPDDR2 | FLAG_16BIT | FLAG_BURST_RDWR | FLAG_ACTIVE_RANKS,
                );
            } else {
                writel(
                    DDR_CONTROL_MSTR,
                    FLAG_LPDDR2 | FLAG_BURST_RDWR | FLAG_ACTIVE_RANKS,
                );
            }

            writel(DDR_CONTROL_MRCTRL0, FLAG_MR_RANK);
            writel(DDR_CONTROL_MRCTRL1, 0);
            writel(DDR_CONTROL_DERATEEN, 0);
            writel(DDR_CONTROL_DERATEINT, FLAG_READ_INTERVAL);
            writel(DDR_CONTROL_PWRCTL, 0);
            writel(
                DDR_CONTROL_PWRTMG,
                FLAG_POWER_DOWN_TO_X32 | FLAG_SELF_REFRESH_TO_X32 | FLAG_DEEP_POWER_DOWN_TO_X1024,
            );
            writel(
                DDR_CONTROL_HWLPCTL,
                FLAG_HW_LOW_POWER | FLAG_HW_EXIT_IDLE | FLAG_HW_LOW_POWER_IDLE_X32,
            );
            writel(
                DDR_CONTROL_RFSHCTL0,
                FLAG_REFRESH_MARGIN | FLAG_REFRESH_TO_X32,
            );
            writel(DDR_CONTROL_RFSHCTL1, 0);
            writel(DDR_CONTROL_RFSHCTL2, 0);
            writel(DDR_CONTROL_RFSHCTL3, 0);

            if self.size.is_dram_32_m() {
                Self::init_timing_32m();
            } else {
                self.init_timing_common();
            }

            writel(DDR_CONTROL_ZQCTL2, 0);
            writel(
                DDR_CONTROL_DFITMG0,
                FLAG_DFI_TPHY_WRLAT | FLAG_DFI_T_RDDATA_EN | FLAG_DFI_T_CTRL_DELAY,
            );
            writel(
                DDR_CONTROL_DFITMG1,
                FLAG_DFI_T_DRAM_CLK_ENABLE | FLAG_DFI_T_DRAM_CLK_DISABLE | FLAG_DFI_T_WRDATA_DELAY,
            );
            writel(
                DDR_CONTROL_DFILPCFG0,
                FLAG_DFI_LP_EN_PD
                    | FLAG_DFI_LP_WAKEUP_PD
                    | FLAG_DFI_LP_EN_SR
                    | FLAG_DFI_LP_WAKEUP_SR
                    | FLAG_DFI_TLP_RESPONSE,
            );
            writel(
                DDR_CONTROL_DFIUPD0,
                FLAG_DFI_T_CTRLUP_MIN | FLAG_DFI_T_CTRLUP_MAX | FLAG_DIS_AUTO_CTRLUPD,
            );
            writel(DDR_CONTROL_DFIUPD1, 0);
            writel(
                DDR_CONTROL_DFIUPD2,
                FLAG_DFI_PHYUPD_TYPE0 | FLAG_DFI_PHYUPD_TYPE1 | FLAG_DFI_PHYUPD_EN,
            );
            writel(DDR_CONTROL_DFIMISC, 0);
            writel(DDR_CONTROL_ADDRMAP0, FLAG_ADDRMAP_CS_BIT0);

            self.configure_address_mapping();

            writel(DDR_CONTROL_OTDCFG, SHIFT_ODT_HOLD(4) | SHIFT_WR_ODT_HOLD(4));
            writel(DDR_CONTROL_ODTMAP, 0);
            writel(
                DDR_CONTROL_SCHED,
                FLAG_LOW_PRIO | FLAG_PAGECLOSE | SHIFT_LPR_NUM_ENTRIES(0x18),
            );
            writel(DDR_CONTROL_SCHED1, 0);
            writel(DDR_CONTROL_PCCFG, 0);
            writel(DDR_CONTROL_DBG0, 0);
            writel(DDR_CONTROL_DBG1, 0);
            writel(DDR_CONTROL_DBGCMD, 0);
            writel(DDR_CONTROL_PCCFG, 0);

            Self::init_priority();
        }
    }
}

impl DramControl {
    unsafe fn configure_address_mapping(&self) {
        match self.size {
            DramSize::Dram32M => unsafe {
                writel(
                    DDR_CONTROL_ADDRMAP1,
                    SHIFT_ADDRMAP_BANK_B0(6)
                        | SHIFT_ADDRMAP_BANK_B1(6)
                        | SHIFT_ADDRMAP_BANK_B2(0x1f),
                );
                writel(DDR_CONTROL_ADDRMAP2, 0);
                writel(
                    DDR_CONTROL_ADDRMAP3,
                    SHIFT_ADDRMAP_COL_B8(0xf) | SHIFT_ADDRMAP_COL_B9(0xf),
                );
                writel(
                    DDR_CONTROL_ADDRMAP4,
                    SHIFT_ADDRMAP_COL_B10(0xf) | SHIFT_ADDRMAP_COL_B11(0xf),
                );
                writel(
                    DDR_CONTROL_ADDRMAP5,
                    SHIFT_ADDRMAP_ROW_B0(4)
                        | SHIFT_ADDRMAP_ROW_B1(4)
                        | SHIFT_ADDRMAP_ROW_B2(4)
                        | SHIFT_ADDRMAP_ROW_B11(4),
                );
                writel(
                    DDR_CONTROL_ADDRMAP6,
                    SHIFT_ADDRMAP_ROW_B12(4)
                        | SHIFT_ADDRMAP_ROW_B13(0xf)
                        | SHIFT_ADDRMAP_ROW_B14(0xf)
                        | SHIFT_ADDRMAP_ROW_B15(0xf),
                );
            },
            DramSize::Dram64M => unsafe {
                writel(
                    DDR_CONTROL_ADDRMAP1,
                    SHIFT_ADDRMAP_BANK_B0(7)
                        | SHIFT_ADDRMAP_BANK_B1(7)
                        | SHIFT_ADDRMAP_BANK_B2(0x1f),
                );
                writel(DDR_CONTROL_ADDRMAP2, 0);
                writel(
                    DDR_CONTROL_ADDRMAP3,
                    SHIFT_ADDRMAP_COL_B8(0) | SHIFT_ADDRMAP_COL_B9(0xf),
                );
                writel(
                    DDR_CONTROL_ADDRMAP4,
                    SHIFT_ADDRMAP_COL_B10(0xf) | SHIFT_ADDRMAP_COL_B11(0xf),
                );
                writel(
                    DDR_CONTROL_ADDRMAP5,
                    SHIFT_ADDRMAP_ROW_B0(5)
                        | SHIFT_ADDRMAP_ROW_B1(5)
                        | SHIFT_ADDRMAP_ROW_B2(5)
                        | SHIFT_ADDRMAP_ROW_B11(5),
                );
                writel(
                    DDR_CONTROL_ADDRMAP6,
                    SHIFT_ADDRMAP_ROW_B12(5)
                        | SHIFT_ADDRMAP_ROW_B13(0xf)
                        | SHIFT_ADDRMAP_ROW_B14(0xf)
                        | SHIFT_ADDRMAP_ROW_B15(0xf),
                );
            },
            DramSize::Dram128M | DramSize::Dram256M => unsafe {
                writel(
                    DDR_CONTROL_ADDRMAP1,
                    SHIFT_ADDRMAP_BANK_B0(7) | SHIFT_ADDRMAP_BANK_B1(7) | SHIFT_ADDRMAP_BANK_B2(7),
                );
                writel(DDR_CONTROL_ADDRMAP2, 0);
                writel(
                    DDR_CONTROL_ADDRMAP3,
                    SHIFT_ADDRMAP_COL_B8(0) | SHIFT_ADDRMAP_COL_B9(0xf),
                );
                writel(
                    DDR_CONTROL_ADDRMAP4,
                    SHIFT_ADDRMAP_COL_B10(0xf) | SHIFT_ADDRMAP_COL_B11(0xf),
                );
                writel(
                    DDR_CONTROL_ADDRMAP5,
                    SHIFT_ADDRMAP_ROW_B0(6)
                        | SHIFT_ADDRMAP_ROW_B1(6)
                        | SHIFT_ADDRMAP_ROW_B2(6)
                        | SHIFT_ADDRMAP_ROW_B11(6),
                );

                if self.size.is_dram_128_m() {
                    writel(
                        DDR_CONTROL_ADDRMAP6,
                        SHIFT_ADDRMAP_ROW_B12(6)
                            | SHIFT_ADDRMAP_ROW_B13(0xf)
                            | SHIFT_ADDRMAP_ROW_B14(0xf)
                            | SHIFT_ADDRMAP_ROW_B15(0xf),
                    );
                } else {
                    writel(
                        DDR_CONTROL_ADDRMAP6,
                        SHIFT_ADDRMAP_ROW_B12(6)
                            | SHIFT_ADDRMAP_ROW_B13(6)
                            | SHIFT_ADDRMAP_ROW_B14(0xf)
                            | SHIFT_ADDRMAP_ROW_B15(0xf),
                    );
                }
            },
            DramSize::Dram512M => unsafe {
                writel(
                    DDR_CONTROL_ADDRMAP1,
                    SHIFT_ADDRMAP_BANK_B0(8) | SHIFT_ADDRMAP_BANK_B1(8) | SHIFT_ADDRMAP_BANK_B2(8),
                );
                writel(DDR_CONTROL_ADDRMAP2, 0);
                writel(
                    DDR_CONTROL_ADDRMAP3,
                    SHIFT_ADDRMAP_COL_B8(0) | SHIFT_ADDRMAP_COL_B9(0),
                );
                writel(
                    DDR_CONTROL_ADDRMAP4,
                    SHIFT_ADDRMAP_COL_B10(0xf) | SHIFT_ADDRMAP_COL_B11(0xf),
                );
                writel(
                    DDR_CONTROL_ADDRMAP5,
                    SHIFT_ADDRMAP_ROW_B0(7)
                        | SHIFT_ADDRMAP_ROW_B1(7)
                        | SHIFT_ADDRMAP_ROW_B2(7)
                        | SHIFT_ADDRMAP_ROW_B11(7),
                );
                writel(
                    DDR_CONTROL_ADDRMAP6,
                    SHIFT_ADDRMAP_ROW_B12(7)
                        | SHIFT_ADDRMAP_ROW_B13(7)
                        | SHIFT_ADDRMAP_ROW_B14(0xf)
                        | SHIFT_ADDRMAP_ROW_B15(0xf),
                );
            },
        }
    }

    unsafe fn init_priority() {
        unsafe {
            writel(
                DDR_CONTROL_PCFGR_0,
                SHIFT_RD_PORT_PRIO(0x3ff) | FLAG_RD_PORT_AGING_ENABLE,
            );
            writel(
                DDR_CONTROL_PCFGR_1,
                SHIFT_RD_PORT_PRIO(0x20) | FLAG_RD_PORT_AGING_ENABLE,
            );
            writel(
                DDR_CONTROL_PCFGR_2,
                SHIFT_RD_PORT_PRIO(0) | FLAG_RD_PORT_AGING_ENABLE,
            );
            writel(
                DDR_CONTROL_PCFGR_3,
                SHIFT_RD_PORT_PRIO(4) | FLAG_RD_PORT_AGING_ENABLE,
            );
            writel(
                DDR_CONTROL_PCFGW_0,
                SHIFT_WR_PORT_PRIO(0x3ff) | FLAG_WR_PORT_AGING_ENABLE,
            );
            writel(
                DDR_CONTROL_PCFGW_1,
                SHIFT_WR_PORT_PRIO(0xff) | FLAG_WR_PORT_AGING_ENABLE,
            );
            writel(
                DDR_CONTROL_PCFGW_2,
                SHIFT_WR_PORT_PRIO(0x3f) | FLAG_WR_PORT_AGING_ENABLE,
            );
            writel(
                DDR_CONTROL_PCFGW_3,
                SHIFT_WR_PORT_PRIO(0x5f) | FLAG_WR_PORT_AGING_ENABLE,
            );
            writel(
                DDR_CONTROL_PCFGQOS0_0,
                SHIFT_RQOS_MAP_LEVEL1(0xe) | SHIFT_RQOS_MAP_REGION1(2),
            );
            writel(
                DDR_CONTROL_PCFGQOS0_1,
                SHIFT_RQOS_MAP_LEVEL1(0xe) | SHIFT_RQOS_MAP_REGION1(2),
            );
            writel(
                DDR_CONTROL_PCFGQOS0_2,
                SHIFT_RQOS_MAP_LEVEL1(0xe) | SHIFT_RQOS_MAP_REGION1(2),
            );
            writel(
                DDR_CONTROL_PCFGQOS0_3,
                SHIFT_RQOS_MAP_LEVEL1(0xe) | SHIFT_RQOS_MAP_REGION1(2),
            );
            writel(DDR_CONTROL_PCFGQOS1_0, 0);
            writel(DDR_CONTROL_PCFGQOS1_1, 0);
            writel(DDR_CONTROL_PCFGQOS1_2, 0);
            writel(DDR_CONTROL_PCFGQOS1_3, 0);
            writel(DDR_CONTROL_PCFGWQOS0_0, 0);
            writel(DDR_CONTROL_PCFGWQOS0_1, 0);
            writel(DDR_CONTROL_PCFGWQOS0_2, 0);
            writel(DDR_CONTROL_PCFGWQOS0_3, 0);
            writel(DDR_CONTROL_PCFGWQOS1_0, 0);
            writel(DDR_CONTROL_PCFGWQOS1_1, 0);
            writel(DDR_CONTROL_PCFGWQOS1_2, 0);
            writel(DDR_CONTROL_PCFGWQOS1_3, 0);
            writel(
                DDR_CONTROL_PERFHPR1,
                SHIFT_HPR_MAX_STARVE(1) | SHIFT_HPR_XACT_RUN_LENGTH(0xf),
            );
            writel(
                DDR_CONTROL_PERFLPR1,
                SHIFT_LPR_MAX_STARVE(0x7f) | SHIFT_LPR_XACT_RUN_LENGTH(0xf),
            );
            writel(
                DDR_CONTROL_PERFWR1,
                SHIFT_W_MAX_STARVE(0x7f) | SHIFT_W_XACT_RUN_LENGTH(0xf),
            );
            writel(DDR_CONTROL_PERFVPR1, 0);
            writel(DDR_CONTROL_PERFVPW1, 0);
        }
    }

    unsafe fn init_timing_32m() {
        unsafe {
            writel(
                DDR_CONTROL_RFSHTMG,
                SHIFT_RFC_MIN(0x1a) | SHIFT_T_RFC_NOM_X32(0x30),
            );
            writel(
                DDR_CONTROL_VALUE_INIT0,
                FLAG_PRE_CKE_X1024 | SHIFT_POST_CKE_X1024(0x28),
            );
            writel(DDR_CONTROL_VALUE_INIT1, 0);
            writel(
                DDR_CONTROL_VALUE_INIT2,
                FLAG_MIN_STABLE_CLOCK_X1 | SHIFT_IDLE_AFTER_RESET_X32(6),
            );
            writel(DDR_CONTROL_VALUE_INIT3, FLAG_EMR | SHIFT_MR(0x83));
            writel(DDR_CONTROL_VALUE_INIT4, FLAG_MR3);
            writel(
                DDR_CONTROL_VALUE_INIT5,
                SHIFT_ZQINIT_X32(7) | FLAG_MAX_AUTO_INIT_X1024,
            );

            writel(
                DDR_CONTROL_RANKCTL,
                FLAG_MAX_RANK_RD | FLAG_DIFF_RANK_RD_GAP | FLAG_RANK_WR_GAP,
            );
            writel(
                DDR_CONTROL_DRAMTMG0,
                SHIFT_T_RAS_MIN(8) | SHIFT_T_RAS_MAX(0xd) | SHIFT_T_FAW(0xa) | SHIFT_WR2PRE(7),
            );
            writel(
                DDR_CONTROL_DRAMTMG1,
                SHIFT_T_RC(0x0d) | FLAG_RD2PRE | FLAG_T_XP,
            );
            writel(
                DDR_CONTROL_DRAMTMG2,
                SHIFT_WR2RD(7) | FLAG_RD2WR | FLAG_READ_LATENCY | FLAG_WRITE_LATENCY,
            );
            writel(DDR_CONTROL_DRAMTMG3, FLAG_T_MOD | FLAG_T_MRD | FLAG_T_MRW);
            writel(
                DDR_CONTROL_DRAMTMG4,
                SHIFT_T_RP(5) | FLAG_T_RRD | FLAG_T_CCD | SHIFT_T_RCD(4),
            );
            writel(
                DDR_CONTROL_DRAMTMG5,
                FLAG_T_CKE | FLAG_T_CKESR | FLAG_T_CKSRE | FLAG_T_CKSRX,
            );
            writel(
                DDR_CONTROL_DRAMTMG6,
                FLAG_T_CKCSX | FLAG_T_CKDPDX | FLAG_T_CKDPDE,
            );
            writel(DDR_CONTROL_DRAMTMG7, FLAG_T_CKPDX | FLAG_T_CKPDE);
            writel(DDR_CONTROL_DRAMTMG14, SHIFT_T_XSR(0x1c));
            writel(
                DDR_CONTROL_ZQCTL0,
                SHIFT_T_ZQ_SHORT_NOP(0x12) | SHIFT_T_ZQ_LONG_NOP(0x48) | FLAG_DIS_SRX_ZQCL,
            );
            writel(
                DDR_CONTROL_ZQCTL1,
                FLAG_T_ZQ_SHORT_INTERVAL_X1024 | SHIFT_T_ZQ_RESET_NOP(0xa),
            );
        }
    }

    unsafe fn init_timing_common(&self) {
        unsafe {
            if self.size.is_dram_256_m() || self.size.is_dram_512_m() {
                writel(
                    DDR_CONTROL_RFSHTMG,
                    SHIFT_RFC_MIN(0x14) | SHIFT_T_RFC_NOM_X32(0x13),
                );
            } else {
                writel(
                    DDR_CONTROL_RFSHTMG,
                    SHIFT_RFC_MIN(0x14) | SHIFT_T_RFC_NOM_X32(0x26),
                );
            }
            writel(
                DDR_CONTROL_VALUE_INIT0,
                FLAG_PRE_CKE_X1024 | SHIFT_POST_CKE_X1024(0x1f),
            );
            writel(DDR_CONTROL_VALUE_INIT1, 0);
            writel(
                DDR_CONTROL_VALUE_INIT2,
                FLAG_MIN_STABLE_CLOCK_X1 | SHIFT_IDLE_AFTER_RESET_X32(4),
            );
            writel(DDR_CONTROL_VALUE_INIT3, FLAG_EMR | SHIFT_MR(0x63));
            writel(DDR_CONTROL_VALUE_INIT4, FLAG_MR3);
            writel(
                DDR_CONTROL_VALUE_INIT5,
                SHIFT_ZQINIT_X32(5) | FLAG_MAX_AUTO_INIT_X1024,
            );

            writel(
                DDR_CONTROL_RANKCTL,
                FLAG_MAX_RANK_RD | FLAG_DIFF_RANK_RD_GAP | FLAG_RANK_WR_GAP,
            );
            writel(
                DDR_CONTROL_DRAMTMG0,
                SHIFT_T_RAS_MIN(7) | SHIFT_T_RAS_MAX(0xa) | SHIFT_T_FAW(8) | SHIFT_WR2PRE(6),
            );
            writel(
                DDR_CONTROL_DRAMTMG1,
                SHIFT_T_RC(0xa) | FLAG_RD2PRE | FLAG_T_XP,
            );
            writel(
                DDR_CONTROL_DRAMTMG2,
                SHIFT_WR2RD(6) | FLAG_RD2WR | FLAG_READ_LATENCY | FLAG_WRITE_LATENCY,
            );
            writel(DDR_CONTROL_DRAMTMG3, FLAG_T_MOD | FLAG_T_MRD | FLAG_T_MRW);
            writel(
                DDR_CONTROL_DRAMTMG4,
                SHIFT_T_RP(4) | FLAG_T_RRD | FLAG_T_CCD | SHIFT_T_RCD(3),
            );
            writel(
                DDR_CONTROL_DRAMTMG5,
                FLAG_T_CKE | FLAG_T_CKESR | FLAG_T_CKSRE | FLAG_T_CKSRX,
            );
            writel(
                DDR_CONTROL_DRAMTMG6,
                FLAG_T_CKCSX | FLAG_T_CKDPDX | FLAG_T_CKDPDE,
            );
            writel(DDR_CONTROL_DRAMTMG7, FLAG_T_CKPDX | FLAG_T_CKPDE);
            writel(DDR_CONTROL_DRAMTMG14, SHIFT_T_XSR(0x16));
            writel(
                DDR_CONTROL_ZQCTL0,
                SHIFT_T_ZQ_SHORT_NOP(0xf) | SHIFT_T_ZQ_LONG_NOP(0x39) | FLAG_DIS_SRX_ZQCL,
            );
            writel(
                DDR_CONTROL_ZQCTL1,
                FLAG_T_ZQ_SHORT_INTERVAL_X1024 | SHIFT_T_ZQ_RESET_NOP(0x8),
            );
        }
    }
}
