//! # Art
//! //!
//! A library for modeling artistic concepet.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds{

    ///The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Blue,
        Green,
        Yellow,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }

}

pub mod utils{
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        //snip

        let mut c3 = SecondaryColor::Green;

        c3     
    }
}
