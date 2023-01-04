// Boost/Apache2 License

#![cfg(windows)]
#![deny(unsafe_code)]
#![forbid(future_incompatible, missing_docs, rust_2018_idioms)]
#![no_std]

//! Get the system colors for Win32.
//!
//! This crate provides a safe wrapper around the `GetSysColor` function. To get a color, call
//! [`SysColor::get`]. The available colors are listed in the [`SysColorIndex`] enum.
//!
//! # Examples
//!
//! ```
//! use win_syscolor::{SysColor, SysColorIndex};
//!
//! let color = SysColor::get(SysColorIndex::ActiveCaption).expect("Color not available");
//! println!("The active caption color is {}", color);
//! ```

use core::fmt;
use core::sync::atomic::{AtomicU8, Ordering};
use windows_sys::Win32::Graphics::Gdi;

/// The system color.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SysColor(u32);

impl fmt::Debug for SysColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SysColor")
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .finish()
    }
}

impl fmt::Display for SysColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "#{:02X}{:02X}{:02X}",
            self.red(),
            self.green(),
            self.blue()
        )
    }
}

impl SysColor {
    fn new(color: u32) -> Self {
        SysColor(color)
    }

    /// Get the raw color.
    pub fn raw(self) -> u32 {
        self.0
    }

    /// Get the red component.
    pub fn red(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Get the green component.
    pub fn green(self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Get the blue component.
    pub fn blue(self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }
}

impl From<SysColor> for u32 {
    fn from(color: SysColor) -> Self {
        color.0
    }
}

impl From<SysColor> for [u8; 3] {
    fn from(color: SysColor) -> Self {
        [color.red(), color.green(), color.blue()]
    }
}

impl From<SysColor> for (u8, u8, u8) {
    fn from(color: SysColor) -> Self {
        (color.red(), color.green(), color.blue())
    }
}

/// Generate the `SysColor` struct and associated functions.
macro_rules! generate_syscolor {
    ($($wname:ident => $name:ident),*) => {
        /// The available system colors.
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[non_exhaustive]
        pub enum SysColorIndex {
            $(
                #[doc = concat!("The `", stringify!($name), "` color.")]
                $name,
            )*
        }

        impl SysColor {
            /// Get the system color.
            pub fn get(index: SysColorIndex) -> Option<Self> {
                match index {
                    $(
                        SysColorIndex::$name => {
                            // Cache whether or not the value is present.
                            static PRESENT: OnceBool = OnceBool::new();

                            get_sys_color(Gdi::$wname, &PRESENT).map(SysColor::new)
                        }
                    )*
                }
            }
        }
    }
}

generate_syscolor! {
    COLOR_3DDKSHADOW => ThreeDDarkShadow,
    COLOR_ACTIVEBORDER => ActiveBorder,
    COLOR_ACTIVECAPTION => ActiveCaption,
    COLOR_APPWORKSPACE => AppWorkspace,
    COLOR_BACKGROUND => Background,
    COLOR_BTNFACE => ButtonFace,
    COLOR_BTNHIGHLIGHT => ButtonHighlight,
    COLOR_BTNSHADOW => ButtonShadow,
    COLOR_BTNTEXT => ButtonText,
    COLOR_CAPTIONTEXT => CaptionText,
    COLOR_GRADIENTACTIVECAPTION => GradientActiveCaption,
    COLOR_GRADIENTINACTIVECAPTION => GradientInactiveCaption,
    COLOR_GRAYTEXT => GrayText,
    COLOR_HIGHLIGHT => Highlight,
    COLOR_HIGHLIGHTTEXT => HighlightText,
    COLOR_HOTLIGHT => HotLight,
    COLOR_INACTIVEBORDER => InactiveBorder,
    COLOR_INACTIVECAPTION => InactiveCaption,
    COLOR_INACTIVECAPTIONTEXT => InactiveCaptionText,
    COLOR_INFOBK => InfoBackground,
    COLOR_INFOTEXT => InfoText,
    COLOR_MENU => Menu,
    COLOR_MENUTEXT => MenuText,
    COLOR_SCROLLBAR => ScrollBar,
    COLOR_WINDOW => Window,
    COLOR_WINDOWFRAME => WindowFrame,
    COLOR_WINDOWTEXT => WindowText
}

/// A lazily-initialized boolean value.
struct OnceBool(AtomicU8);

const UNINIT: u8 = 0xFF;
const FALSE: u8 = 0;
const TRUE: u8 = 1;

impl OnceBool {
    /// Creates a new `OnceBool` in an uninitialized state.
    const fn new() -> Self {
        OnceBool(AtomicU8::new(UNINIT))
    }

    /// Gets the value, initializing it with the closure if it is the first time this method has been
    /// called.
    fn get_or_init<F: FnOnce() -> bool>(&self, f: F) -> bool {
        let mut value = self.0.load(Ordering::Acquire);
        let mut closure = Some(f);

        loop {
            // Has the value been initialized?
            match value {
                TRUE => return true,
                FALSE => return false,
                _ => {}
            }

            // Initialize the value.
            let new_value = (closure.take().unwrap())() as u8;

            // Try to set the value.
            value = self
                .0
                .compare_exchange(value, new_value, Ordering::AcqRel, Ordering::Acquire)
                .unwrap_or_else(|x| x);
        }
    }
}

#[allow(unsafe_code)]
#[inline]
fn get_sys_color(index: i32, present: &'static OnceBool) -> Option<u32> {
    // See if the color is present.
    let present = present.get_or_init(move || {
        let brush = unsafe { Gdi::GetSysColorBrush(index) };
        brush != 0
    });

    if !present {
        return None;
    }

    // Get the color.
    let color = unsafe { Gdi::GetSysColor(index) };
    Some(color)
}
