//! Platform agnostic rust driver for the Antminer BM1397 SHA-256 miner asic.
//!
//! This is a low-level (ll) crate. The scope of this crate is:
//! 1) Register accessors.
//!
//! Higher level functionality should be built on-top of what is provided here.
//!
//! # Feature Flags
//!
//! All features are disabled by default.
//!
//! * `defmt`: Enable formatting most types with `defmt`.
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod addr;
mod registers;
mod specifiers;

pub use addr::Reg;
pub use registers::ChipAddress;
pub use specifiers::ClockSelect;

/// BM1397 register setters and getters.
///
/// * All register getters are simply the name of the register.
/// * All register setters are the name of the register prefixed with `set_`.
pub trait Registers {
    /// Register accessor error type.
    type Error;

    /// Read from the BM1397.
    ///
    /// # Arguments
    ///
    /// * `addr` - Starting address of the memory being read.
    /// * `data` - Buffer to read data into. The number of bytes read is equal
    ///   to the length of this buffer.
    fn read(&mut self, addr: u8, data: &mut [u32]) -> Result<(), Self::Error>;

    /// Write to the BM1397.
    ///
    /// # Arguments
    ///
    /// * `addr` - Starting address of the memory being written.
    /// * `data` - Buffer of data to write. The number of bytes written is equal
    ///   to the length of this buffer.
    fn write(&mut self, addr: u8, data: &[u32]) -> Result<(), Self::Error>;

    /// Get the Chip Address register.
    ///
    /// # Example
    ///
    /// ```
    /// # let spi = ehm1::spi::Mock::new(&[
    /// #   ehm1::spi::Transaction::transaction_start(),
    /// #   ehm1::spi::Transaction::write_vec(vec![0x00, 0x00, 0x00]),
    /// #   ehm1::spi::Transaction::read(0),
    /// #   ehm1::spi::Transaction::transaction_end(),
    /// # ]);
    /// use bm1397_ll::{eh1::vdm::BM1397, ChipAddress, Registers};
    ///
    /// let mut BM1397 = BM1397::new(spi);
    /// let chip_addr: ChipAddress = BM1397.chip_addr()?;
    /// assert_eq!(chip_addr, ChipAddress::default());
    /// # Ok::<(), eh1::spi::ErrorKind>(())
    /// ```
    fn chip_addr(&mut self) -> Result<ChipAddress, Self::Error> {
        let mut reg: [u32; 1] = [0];
        self.read(Reg::CHIPADDR.addr(), &mut reg)?;
        Ok(ChipAddress::from(reg[0]))
    }

    /// Set the Chip Address register.
    ///
    /// # Example
    ///
    /// ```
    /// # let spi = ehm1::spi::Mock::new(&[
    /// #   ehm1::spi::Transaction::transaction_start(),
    /// #   ehm1::spi::Transaction::write_vec(vec![0x00, 0x00, 0x04]),
    /// #   ehm1::spi::Transaction::write(bm1397_ll::ChipAddress::WOL_MASK),
    /// #   ehm1::spi::Transaction::transaction_end(),
    /// # ]);
    /// use bm1397_ll::{eh1::vdm::BM1397, ChipAddress, Registers};
    ///
    /// const CHIP_ADDR: ChipAddress = ChipAddress::DEFAULT.enable_wol();
    /// let mut BM1397 = BM1397::new(spi);
    /// BM1397.set_chip_addr(CHIP_ADDR)?;
    /// # Ok::<(), eh1::spi::ErrorKind>(())
    /// ```
    fn set_chip_addr(&mut self, chip_addr: ChipAddress) -> Result<(), Self::Error> {
        self.write(Reg::CHIPADDR.addr(), &[chip_addr.into()])
    }
}
