use bitflags::bitflags;

bitflags! 
{
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct WindowFlags: u32
    {
        const None = 0;
        const Default = Self::Shown.bits() | Self::Resizable.bits();
        const Fullscreen = 2 << 1;
        // const FullscreenDesktop;
        // const Opengl;
        // const Vulkan;
        const Shown = 2 << 5;
        const Hidden = 2 << 6;
        const Borderless = 2 << 7;
        const Resizable = 2 << 8;
        const Minimized = 2 << 9;
        const Maximized = 2 << 10;
        // const InputGrabbed;
        // const InputFocus;
        // const MouseFocus;
        // const Foreign;
        // const AllowHighdpi;
        // const MouseCapture;
        // const AlwaysOnTop;
        // const SkipTaskbar;
        // const Utility;
        // const Tooltip;
        // const PopupMenu;
    }
}