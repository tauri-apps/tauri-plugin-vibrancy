// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// The use of NSVisualEffectView comes from https://github.com/joboet/winit/tree/macos_blurred_background
// with a bit of rewrite by @youngsing to make it more like cocoa::appkit style.

/// <https://developer.apple.com/documentation/appkit/nsvisualeffectview/material>

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSVisualEffectMaterial {
    #[deprecated(
        since = "macOS 10.14",
        note = "A default material appropriate for the view's effectiveAppearance.  You should instead choose an appropriate semantic material."
    )]
    AppearanceBased = 0,
    #[deprecated(since = "macOS 10.14", note = "Use a semantic material instead.")]
    Light = 1,
    #[deprecated(since = "macOS 10.14", note = "Use a semantic material instead.")]
    Dark = 2,
    #[deprecated(since = "macOS 10.14", note = "Use a semantic material instead.")]
    MediumLight = 8,
    #[deprecated(since = "macOS 10.14", note = "Use a semantic material instead.")]
    UltraDark = 9,

    /// macOS 10.10+
    Titlebar = 3,
    /// macOS 10.10+
    Selection = 4,

    /// macOS 10.11+
    Menu = 5,
    /// macOS 10.11+
    Popover = 6,
    /// macOS 10.11+
    Sidebar = 7,

    /// macOS 10.14+
    HeaderView = 10,
    /// macOS 10.14+
    Sheet = 11,
    /// macOS 10.14+
    WindowBackground = 12,
    /// macOS 10.14+
    HudWindow = 13,
    /// macOS 10.14+
    FullScreenUI = 15,
    /// macOS 10.14+
    Tooltip = 17,
    /// macOS 10.14+
    ContentBackground = 18,
    /// macOS 10.14+
    UnderWindowBackground = 21,
    /// macOS 10.14+
    UnderPageBackground = 22,
}

/// <https://developer.apple.com/documentation/appkit/nsvisualeffectview/state>
#[allow(dead_code)]
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSVisualEffectState {
    /// Make window vibrancy state follow the window's active state
    FollowsWindowActiveState = 0,
    /// Make window vibrancy state always active
    Active = 1,
    /// Make window vibrancy state always inactive
    Inactive = 2,
}

#[cfg(target_os = "macos")]
pub use internal::apply_vibrancy;

#[cfg(target_os = "macos")]
mod internal {
    use std::{ffi::c_void, ptr::NonNull};

    use objc2_app_kit::{
        NSAppKitVersionNumber, NSAppKitVersionNumber10_10, NSAppKitVersionNumber10_11,
        NSAppKitVersionNumber10_14, NSAutoresizingMaskOptions, NSView, NSVisualEffectBlendingMode,
        NSVisualEffectMaterial, NSVisualEffectState, NSVisualEffectView, NSWindowOrderingMode,
    };
    use objc2_foundation::{CGFloat, MainThreadMarker};

    use crate::Error;

    #[allow(deprecated)]
    pub unsafe fn apply_vibrancy(
        ns_view: NonNull<c_void>,
        appearance: super::NSVisualEffectMaterial,
        state: Option<super::NSVisualEffectState>,
        radius: Option<f64>,
    ) -> Result<(), Error> {
        let mtm = MainThreadMarker::new().ok_or(Error::NotMainThread(
            "\"apply_vibrancy()\" can only be used on the main thread.",
        ))?;

        unsafe {
            let view: &NSView = ns_view.cast().as_ref();

            if NSAppKitVersionNumber < NSAppKitVersionNumber10_10 {
                return Err(Error::UnsupportedPlatformVersion(
                    "\"apply_vibrancy()\" is only available on macOS 10.0 or newer.",
                ));
            }

            let mut m = NSVisualEffectMaterial(appearance as isize);
            if (appearance as u32 > 9 && NSAppKitVersionNumber < NSAppKitVersionNumber10_14)
                || (appearance as u32 > 4 && NSAppKitVersionNumber < NSAppKitVersionNumber10_11)
            {
                m = NSVisualEffectMaterial::AppearanceBased;
            }

            let bounds = view.bounds();
            let blurred_view = NSVisualEffectView::initWithFrame(mtm.alloc(), bounds);

            blurred_view.setMaterial(m);
            set_corner_radius(&blurred_view, radius.unwrap_or(0.0));
            blurred_view.setBlendingMode(NSVisualEffectBlendingMode::BehindWindow);
            blurred_view.setState(
                state
                    .map(|state| NSVisualEffectState(state as isize))
                    .unwrap_or(NSVisualEffectState::FollowsWindowActiveState),
            );
            blurred_view.setAutoresizingMask(
                NSAutoresizingMaskOptions::NSViewWidthSizable
                    | NSAutoresizingMaskOptions::NSViewHeightSizable,
            );

            view.addSubview_positioned_relativeTo(
                &blurred_view,
                NSWindowOrderingMode::NSWindowBelow,
                None,
            );
        }
        Ok(())
    }

    // TODO: Does not seem to be public?
    fn set_corner_radius(view: &NSVisualEffectView, radius: CGFloat) {
        unsafe { objc2::msg_send![view, setCornerRadius: radius] }
    }
}
