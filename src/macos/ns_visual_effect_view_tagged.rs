#![cfg(target_os = "macos")]
#![allow(non_snake_case)]

use objc2::mutability::MainThreadOnly;
use objc2::rc::{Allocated, Retained};
use objc2::ClassType;
use objc2::DeclaredClass;
use objc2::{declare_class, msg_send, msg_send_id};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSVisualEffectBlendingMode, NSVisualEffectMaterial,
    NSVisualEffectState, NSVisualEffectView,
};
use objc2_foundation::{CGFloat, NSInteger, NSRect};

/// NSVisualEffectViewTagged state.
/// Forced to be public by declare_class! macro.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct NSVisualEffectViewTaggedIvars {
    /// NSView tag to identify the view
    pub tag: NSInteger,
}

declare_class!(
    /// A custom NSVisualEffectView subclass
    /// that overrides the tag method to provide a custom tag, to later identify the view
    pub struct NSVisualEffectViewTagged;

    unsafe impl ClassType for NSVisualEffectViewTagged {
        type Super = NSVisualEffectView;
        type Mutability = MainThreadOnly;

        const NAME: &'static str = "NSVisualEffectViewTagged";
    }

    impl DeclaredClass for NSVisualEffectViewTagged {
      type Ivars = NSVisualEffectViewTaggedIvars;
    }

    unsafe impl NSVisualEffectViewTagged {
        #[method(tag)]
        fn tag(&self) -> NSInteger {
            self.ivars().tag
        }
    }
);

impl NSVisualEffectViewTagged {
    pub unsafe fn initWithFrame(
        this: Allocated<Self>,
        frame_rect: NSRect,
        tag: NSInteger,
    ) -> Retained<Self> {
        let state = NSVisualEffectViewTaggedIvars { tag };
        let this = this.set_ivars(state);

        msg_send_id![super(this), initWithFrame: frame_rect]
    }

    pub unsafe fn setMaterial(&self, material: NSVisualEffectMaterial) {
        let () = msg_send![self, setMaterial: material];
    }

    pub unsafe fn setBlendingMode(&self, blending_mode: NSVisualEffectBlendingMode) {
        let () = msg_send![self, setBlendingMode: blending_mode];
    }

    pub unsafe fn setState(&self, state: NSVisualEffectState) {
        let () = msg_send![self, setState: state];
    }

    pub unsafe fn setAutoresizingMask(&self, mask: NSAutoresizingMaskOptions) {
        let () = msg_send![self, setAutoresizingMask: mask];
    }

    pub unsafe fn setCornerRadius(&self, radius: CGFloat) {
        let () = msg_send![self, setCornerRadius: radius];
    }
}
