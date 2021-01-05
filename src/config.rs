//! # Device Configuration
//!
//! The device configuration is read from EEPROM.
//!
//! ## Memory Map
//!
//!                 0           8          16          24          32
//!                 +-----------+-----------+-----------+-----------+
//!     0x0808_0000 | Version   | Reserved                          |
//!                 +-----------+-----------+-----------+-----------+
//!     0x0808_0004 | DevAddr                                       |
//!                 +-----------+-----------+-----------+-----------+
//!     0x0808_0008 |                                               |
//!     0x0808_000C | NwkSKey                                       |
//!     0x0808_0010 |                                               |
//!     0x0808_0014 |                                               |
//!                 +-----------+-----------+-----------+-----------+
//!     0x0808_0018 |                                               |
//!     0x0808_001C | AppSKey                                       |
//!     0x0808_0020 |                                               |
//!     0x0808_0024 |                                               |
//!                 +-----------+-----------+-----------+-----------+
//!     0x0808_0028 | WakeupInterval        | ITempHumi | IVoltage  |
//!                 +-----------+-----------+-----------+-----------+
//!
//! ## Fields
//!
//! ### Header (0x0808_0000 - 0x0808_0004, 4 bytes)
//!
//! - `Version`: The constant `0x01`, can be used to change the config layout
//!   in the future (1 byte)
//! - The other three bytes are reserved, for version 1 they should contain the
//!   sequence `0x23 0x42 0x99` (in order to have some more checks against
//!   configuration data corruption).
//!
//! ### LoRaWAN Configuration (0x0808_0004 - 0x0808_0028, 36 bytes)
//!
//! - `DevAddr`: LoRaWAN device address (4 bytes)
//! - `NwkSKey`: LoRaWAN ABP network session key (16 bytes)
//! - `AppSKey`: LoRaWAN ABP app session key (16 bytes)
//!
//! ### Interval Configuration (0x0808_0028 - 0x0808_002C, 4 bytes)
//!
//! - `WakeupInterval`: How often (in seconds) the device should wake up to
//!   start measurement(s) (2 bytes, u16, LE)
//! - `ITempHumi`: Every n-th measurement will measure and send temperature and
//!   humidity (1 byte, u8)
//! - `IVoltage`: Every n-th measurement will measure and send battery
//!   voltage (1 byte, u8)
//!
//! Example: With the following value at `0x0808_0028`:
//!
//!     +-------------------------------------------+
//!     | 00000384   00000000 | 00000001 | 00000004 |
//!     +-------------------------------------------+
//!
//! ...the temperature and humidity will be sent every 15 minutes, while the
//! voltage will be sent every hour.

use core::{convert::TryInto, fmt};

use stm32l0xx_hal::pac;

pub const BASE_ADDR: usize = 0x0808_0000;
pub const CONFIG_DATA_SIZE: usize = 44;

pub enum ConfigVersion {
    V1,
}

impl fmt::Display for ConfigVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V1 => write!(f, "1"),
        }
    }
}

pub enum ConfigError {
    /// The version byte is not supported.
    UnsupportedVersion(u8),
    /// Wrong magic bytes, the configuration data might be corrupted.
    WrongMagicBytes,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedVersion(v) => write!(f, "Unsupported config format version ({})", v),
            Self::WrongMagicBytes => write!(f, "Wrong magic bytes"),
        }
    }
}

pub struct Config {
    /// Configuration format version
    pub version: ConfigVersion,
    /// LoRaWAN device address (4 bytes)
    pub devaddr: [u8; 4],
    /// LoRaWAN ABP network session key (16 bytes)
    pub nwkskey: [u8; 16],
    /// LoRaWAN ABP app session key (16 bytes)
    pub appskey: [u8; 16],
    /// How often (in seconds) the device should wake up to start measurement(s)
    pub wakeup_interval_seconds: u16,
    /// Every n-th measurement will measure and send temperature and humidity
    pub nth_temp_humi: u8,
    /// Every n-th measurement will measure and send battery voltage
    pub nth_voltage: u8,
}

impl Config {
    /// Read current device configuration from EEPROM.
    ///
    /// Returns an error if the version field does not contain a supported
    /// value.
    pub fn read_from_eeprom(_flash: &mut pac::FLASH) -> Result<Self, ConfigError> {
        // Note(unsafe): Read with no side effects. Because we have a mutable
        // reference to the `FLASH` peripheral, no other part of the code can
        // write to EEPROM at the same time, so the slice should remain valid
        // for the duration of this function.
        let config_data: &[u8] =
            unsafe { core::slice::from_raw_parts(BASE_ADDR as *const u8, CONFIG_DATA_SIZE) };

        // Determine version
        let version: ConfigVersion = match config_data[0] {
            1 => ConfigVersion::V1,
            other => return Err(ConfigError::UnsupportedVersion(other)),
        };

        // Validate magic bytes
        if &config_data[0x01..0x04] != &[0x23, 0x42, 0x99] {
            return Err(ConfigError::WrongMagicBytes);
        }

        // Read keys
        let devaddr: [u8; 4] = config_data[0x04..=0x07]
            .try_into()
            .expect("Reading devaddr failed");
        let nwkskey: [u8; 16] = config_data[0x08..=0x17]
            .try_into()
            .expect("Reading nwkskey failed");
        let appskey: [u8; 16] = config_data[0x18..=0x27]
            .try_into()
            .expect("Reading appskey failed");

        // Read interval data
        let wakeup_interval_seconds = u16::from_le_bytes(
            config_data[0x28..=0x29]
                .try_into()
                .expect("Reading wakeup interval failed"),
        );
        let nth_temp_humi = config_data[0x2A];
        let nth_voltage = config_data[0x2B];

        Ok(Self {
            version,
            devaddr,
            nwkskey,
            appskey,
            wakeup_interval_seconds,
            nth_temp_humi,
            nth_voltage,
        })
    }
}
