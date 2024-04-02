
#[cfg(feature = "raspberry_pi_5")]
pub mod raspberry_pi_5;

#[cfg(feature = "raspberry_pi_5")]
pub use raspberry_pi_5::init as init;
