/// BM1397 common register addresses.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum Reg {
    /// Address of the Chip Address register.
    CHIPADDR = 0x00,
    /// Address of the Hash Rate register.
    HASHRATE = 0x04,
    /// Address of the PLL0 Parameter register.
    PLL0PARAM = 0x08,
    /// Address of the Chip Nonce Offset register.
    CHIPNONCEOFF = 0x0C,
    /// Address of the Hash Counting Number register.
    HASHCNTNBR = 0x10,
    /// Address of the Ticket Mask register.
    TICKETMASK = 0x14,
    /// Address of the Misc Control register.
    MISCCTRL = 0x18,
    /// Address of the Ordered Clock Enable register.
    ORDCLKEN = 0x20,
    /// Address of the Fast UART Configuration register.
    FASTUARTCFG = 0x28,
    /// Address of the UART Relay register.
    UARTRELAY = 0x2C,
    /// Address of the Ticket Mask2 register.
    TICKETMASK2 = 0x38,
    /// Address of the External Temperature Sensor Read register.
    EXTTEMPSENSRD = 0x44,
    /// Address of the Error Flag register.
    ERRFLAG = 0x48,
    /// Address of the Nonce Error Counter register.
    NONCEERRCNT = 0x4C,
    /// Address of the Nonce Overflow Counter register.
    NONCEOVFLCNT = 0x50,
    /// Address of the Analog Mux Control register.
    ANAMUXCTRL = 0x54,
    /// Address of the IO Driver Strenght Configuration register.
    IODRVSTRCFG = 0x58,
    /// Address of the Time Out register.
    TIMEOUT = 0x5C,
    /// Address of the PLL1 Parameter register.
    PLL1PARAM = 0x60,
    /// Address of the PLL2 Parameter register.
    PLL2PARAM = 0x64,
    /// Address of the PLL3 Parameter register.
    PLL3PARAM = 0x68,
    /// Address of the Ordered Clock Monitor register.
    ORDCLKMON = 0x6C,
    /// Address of the PLL0 Divider register.
    PLL0DIV = 0x70,
    /// Address of the PLL1 Divider register.
    PLL1DIV = 0x74,
    /// Address of the PLL2 Divider register.
    PLL2DIV = 0x78,
    /// Address of the PLL3 Divider register.
    PLL3DIV = 0x7C,
    /// Address of the Clock Order Control register, index 0.
    CLKORDCTRL0 = 0x80,
    /// Address of the Clock Order Control register, index 1.
    CLKORDCTRL1 = 0x84,
    /// Address of the Clock Order Status register.
    CLKORDSTAT = 0x8C,
    /// Address of the Frequency Sweep Control1 register.
    FREQSWECTRL1 = 0x90,
    /// Address of the Golden Nonce For Sweep Return register.
    GLDNONCESWERET = 0x94,
    /// Address of the Returned Group Pattern Status register.
    RETGRPPATTSTAT = 0x98,
    /// Address of the Nonce Returned Timeout register.
    NONCERETTIMO = 0x9C,
    /// Address of the Returned Single Pattern Status register.
    RETSINGPATTSTAT = 0xA0,
}

impl From<Reg> for u8 {
    fn from(reg: Reg) -> Self {
        reg as u8
    }
}

impl TryFrom<u8> for Reg {
    type Error = u8;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            x if x == Self::CHIPADDR as u8 => Ok(Self::CHIPADDR),
            x if x == Self::HASHRATE as u8 => Ok(Self::HASHRATE),
            x if x == Self::PLL0PARAM as u8 => Ok(Self::PLL0PARAM),
            x if x == Self::CHIPNONCEOFF as u8 => Ok(Self::CHIPNONCEOFF),
            x if x == Self::HASHCNTNBR as u8 => Ok(Self::HASHCNTNBR),
            x if x == Self::TICKETMASK as u8 => Ok(Self::TICKETMASK),
            x if x == Self::MISCCTRL as u8 => Ok(Self::MISCCTRL),
            x if x == Self::ORDCLKEN as u8 => Ok(Self::ORDCLKEN),
            x if x == Self::FASTUARTCFG as u8 => Ok(Self::FASTUARTCFG),
            x if x == Self::UARTRELAY as u8 => Ok(Self::UARTRELAY),
            x if x == Self::TICKETMASK2 as u8 => Ok(Self::TICKETMASK2),
            x if x == Self::EXTTEMPSENSRD as u8 => Ok(Self::EXTTEMPSENSRD),
            x if x == Self::ERRFLAG as u8 => Ok(Self::ERRFLAG),
            x if x == Self::NONCEERRCNT as u8 => Ok(Self::NONCEERRCNT),
            x if x == Self::NONCEOVFLCNT as u8 => Ok(Self::NONCEOVFLCNT),
            x if x == Self::ANAMUXCTRL as u8 => Ok(Self::ANAMUXCTRL),
            x if x == Self::IODRVSTRCFG as u8 => Ok(Self::IODRVSTRCFG),
            x if x == Self::TIMEOUT as u8 => Ok(Self::TIMEOUT),
            x if x == Self::PLL1PARAM as u8 => Ok(Self::PLL1PARAM),
            x if x == Self::PLL2PARAM as u8 => Ok(Self::PLL2PARAM),
            x if x == Self::PLL3PARAM as u8 => Ok(Self::PLL3PARAM),
            x if x == Self::ORDCLKMON as u8 => Ok(Self::ORDCLKMON),
            x if x == Self::PLL0DIV as u8 => Ok(Self::PLL0DIV),
            x if x == Self::PLL1DIV as u8 => Ok(Self::PLL1DIV),
            x if x == Self::PLL2DIV as u8 => Ok(Self::PLL2DIV),
            x if x == Self::PLL3DIV as u8 => Ok(Self::PLL3DIV),
            x if x == Self::CLKORDCTRL0 as u8 => Ok(Self::CLKORDCTRL0),
            x if x == Self::CLKORDCTRL1 as u8 => Ok(Self::CLKORDCTRL1),
            x if x == Self::CLKORDSTAT as u8 => Ok(Self::CLKORDSTAT),
            x if x == Self::FREQSWECTRL1 as u8 => Ok(Self::FREQSWECTRL1),
            x if x == Self::GLDNONCESWERET as u8 => Ok(Self::GLDNONCESWERET),
            x if x == Self::RETGRPPATTSTAT as u8 => Ok(Self::RETGRPPATTSTAT),
            x if x == Self::NONCERETTIMO as u8 => Ok(Self::NONCERETTIMO),
            x if x == Self::RETSINGPATTSTAT as u8 => Ok(Self::RETSINGPATTSTAT),
            _ => Err(val),
        }
    }
}

impl Reg {
    /// Get the address of the register.
    ///
    /// # Example
    ///
    /// ```
    /// use bm1397_ll::Reg;
    ///
    /// assert_eq!(Reg::CHIPADDR.addr(), 0x00);
    /// ```
    pub const fn addr(self) -> u8 {
        self as u8
    }

    /// Returns `true` if the register is read-only.
    ///
    /// # Example
    ///
    /// ```
    /// use bm1397_ll::Reg;
    ///
    /// assert!(Reg::CLKORDSTAT.is_ro());
    /// assert!(!Reg::CHIPADDR.is_ro());
    /// ```
    pub const fn is_ro(self) -> bool {
        matches!(
            self,
            Reg::CLKORDSTAT | Reg::RETGRPPATTSTAT | Reg::RETSINGPATTSTAT
        )
    }
}
