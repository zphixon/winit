#![cfg(target_os = "wasi")]

use std::{mem, ptr, str};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::os::raw::{c_char, c_void, c_double, c_ulong, c_int};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, Arc};

use dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize};
use raw_window_handle::RawWindowHandle;
use window::MonitorId as RootMonitorId;

#[derive(Clone, Default)]
pub struct PlatformSpecificWindowBuilderAttributes;

unsafe impl Send for PlatformSpecificWindowBuilderAttributes {}
unsafe impl Sync for PlatformSpecificWindowBuilderAttributes {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId;

impl DeviceId {
    pub unsafe fn dummy() -> Self {
        DeviceId
    }
}

#[derive(Clone, Default)]
pub struct PlatformSpecificHeadlessBuilderAttributes;

#[derive(Debug, Clone)]
pub struct MonitorId;

impl MonitorId {
    #[inline]
    pub fn get_name(&self) -> Option<String> {
        Some("window".to_owned())
    }

    #[inline]
    pub fn get_position(&self) -> PhysicalPosition {
        unimplemented!()
    }

    #[inline]
    pub fn get_dimensions(&self) -> PhysicalSize {
        (0, 0).into()
    }

    #[inline]
    pub fn get_hidpi_factor(&self) -> f64 {
        1.0
    }
}

#[derive(Clone)]
pub struct EventsLoopProxy;

impl EventsLoopProxy {
    pub fn wakeup(&self) -> Result<(), ::EventsLoopClosed> {
        unimplemented!()
    }
}

pub struct EventsLoop {
}

impl EventsLoop {
    pub fn new() -> EventsLoop {
        EventsLoop {
        }
    }

    #[inline]
    pub fn interrupt(&self) {
    }

    #[inline]
    pub fn create_proxy(&self) -> EventsLoopProxy {
        unimplemented!()
    }

    #[inline]
    pub fn get_available_monitors(&self) -> VecDeque<MonitorId> {
        let mut list = VecDeque::with_capacity(1);
        list.push_back(MonitorId);
        list
    }

    #[inline]
    pub fn get_primary_monitor(&self) -> MonitorId {
        MonitorId
    }

    pub fn poll_events<F>(&self, mut callback: F)
        where F: FnMut(::Event)
    {
    }

    pub fn run_forever<F>(&self, mut callback: F)
        where F: FnMut(::Event) -> ::ControlFlow
    {
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(usize);

impl WindowId {
    pub unsafe fn dummy() -> Self {
        WindowId(0)
    }
}

pub struct Window {
}

impl Window {
    pub fn new(events_loop: &EventsLoop, attribs: ::WindowAttributes,
               _pl_attribs: PlatformSpecificWindowBuilderAttributes)
        -> Result<Window, ::CreationError>
    {
        Ok(Window {})
    }

    #[inline]
    pub fn id(&self) -> WindowId {
        WindowId(0)
    }

    #[inline]
    pub fn set_title(&self, _title: &str) {
    }

    #[inline]
    pub fn get_position(&self) -> Option<LogicalPosition> {
        Some((0, 0).into())
    }

    #[inline]
    pub fn get_inner_position(&self) -> Option<LogicalPosition> {
        Some((0, 0).into())
    }

    #[inline]
    pub fn set_position(&self, _: LogicalPosition) {
    }

    #[inline]
    pub fn get_inner_size(&self) -> Option<LogicalSize> {
        None
    }

    #[inline]
    pub fn get_outer_size(&self) -> Option<LogicalSize> {
        self.get_inner_size()
    }

    #[inline]
    pub fn set_inner_size(&self, size: LogicalSize) {
    }

    #[inline]
    pub fn set_min_dimensions(&self, _dimensions: Option<LogicalSize>) {
        // N/A
    }

    #[inline]
    pub fn set_max_dimensions(&self, _dimensions: Option<LogicalSize>) {
        // N/A
    }

    #[inline]
    pub fn set_resizable(&self, _resizable: bool) {
        // N/A
    }

    #[inline]
    pub fn show(&self) {
        // N/A
    }

    #[inline]
    pub fn hide(&self) {
        // N/A
    }

    #[inline]
    pub fn set_cursor(&self, _cursor: ::MouseCursor) {
        // N/A
    }

    #[inline]
    pub fn grab_cursor(&self, grab: bool) -> Result<(), String> {
        Err("".into())
    }

    #[inline]
    pub fn hide_cursor(&self, hide: bool) {
    }

    #[inline]
    pub fn get_hidpi_factor(&self) -> f64 {
        1.0
    }

    #[inline]
    pub fn set_cursor_position(&self, _position: LogicalPosition) -> Result<(), String> {
        Err("Setting cursor position is not possible on Emscripten.".to_owned())
    }

    #[inline]
    pub fn set_maximized(&self, _maximized: bool) {
        // iOS has single screen maximized apps so nothing to do
    }

    #[inline]
    pub fn get_fullscreen(&self) -> Option<::MonitorId> {
        None
    }

    #[inline]
    pub fn set_fullscreen(&self, _monitor: Option<::MonitorId>) {
        // iOS has single screen maximized apps so nothing to do
    }

    #[inline]
    pub fn set_decorations(&self, _decorations: bool) {
        // N/A
    }

    #[inline]
    pub fn set_always_on_top(&self, _always_on_top: bool) {
        // N/A
    }

    #[inline]
    pub fn set_window_icon(&self, _icon: Option<::Icon>) {
        // N/A
    }

    #[inline]
    pub fn set_ime_spot(&self, _logical_spot: LogicalPosition) {
        // N/A
    }

    #[inline]
    pub fn get_current_monitor(&self) -> RootMonitorId {
        RootMonitorId { inner: MonitorId }
    }

    #[inline]
    pub fn get_available_monitors(&self) -> VecDeque<MonitorId> {
        let mut list = VecDeque::with_capacity(1);
        list.push_back(MonitorId);
        list
    }

    #[inline]
    pub fn get_primary_monitor(&self) -> MonitorId {
        MonitorId
    }

    pub fn raw_window_handle(&self) -> RawWindowHandle {
        unimplemented!()
    }
}
