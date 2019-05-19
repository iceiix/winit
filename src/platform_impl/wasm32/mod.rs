#![cfg(target_arch = "wasm32")]
#![allow(dead_code, unused_variables)]

use std::collections::VecDeque;

use dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize};
use icon::Icon;
//use event::Event;
use event_loop::{EventLoopClosed, ControlFlow, EventLoopWindowTarget as RootELW};
use monitor::MonitorHandle as RootMonitorHandle;
use window::{WindowAttributes, CreationError, MouseCursor};

#[derive(Clone, Default)]
pub struct PlatformSpecificWindowBuilderAttributes {
}

pub enum Window {
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WindowId {
}

impl WindowId {
    pub unsafe fn dummy() -> Self {
        unimplemented!()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeviceId {
}

impl DeviceId {
    pub unsafe fn dummy() -> Self {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub enum MonitorHandle {
}

impl MonitorHandle {
    #[inline]
    pub fn get_name(&self) -> Option<String> {
        unimplemented!()
    }

    #[inline]
    pub fn get_native_identifier(&self) -> u32 {
        unimplemented!()
    }

    #[inline]
    pub fn get_dimensions(&self) -> PhysicalSize {
        unimplemented!()
    }

    #[inline]
    pub fn get_position(&self) -> PhysicalPosition {
        unimplemented!()
    }

    #[inline]
    pub fn get_hidpi_factor(&self) -> f64 {
        unimplemented!()
    }
}

impl Window {
    #[inline]
    pub fn new<T>(
        window_target: &EventLoopWindowTarget<T>,
        attribs: WindowAttributes,
        pl_attribs: PlatformSpecificWindowBuilderAttributes,
    ) -> Result<Self, CreationError> {
        unimplemented!()
    }

    #[inline]
    pub fn id(&self) -> WindowId {
        unimplemented!()
    }

    #[inline]
    pub fn set_title(&self, title: &str) {
        unimplemented!()
    }

    #[inline]
    pub fn show(&self) {
        unimplemented!()
    }

    #[inline]
    pub fn hide(&self) {
        unimplemented!()
    }

    #[inline]
    pub fn get_position(&self) -> Option<LogicalPosition> {
        unimplemented!()
    }

    #[inline]
    pub fn get_inner_position(&self) -> Option<LogicalPosition> {
        unimplemented!()
    }

    #[inline]
    pub fn set_position(&self, position: LogicalPosition) {
        unimplemented!()
    }

    #[inline]
    pub fn get_inner_size(&self) -> Option<LogicalSize> {
        unimplemented!()
    }

    #[inline]
    pub fn get_outer_size(&self) -> Option<LogicalSize> {
        unimplemented!()
    }

    #[inline]
    pub fn set_inner_size(&self, size: LogicalSize) {
        unimplemented!()
    }

    #[inline]
    pub fn set_min_dimensions(&self, dimensions: Option<LogicalSize>) {
        unimplemented!()
    }

    #[inline]
    pub fn set_max_dimensions(&self, dimensions: Option<LogicalSize>) {
        unimplemented!()
    }

    #[inline]
    pub fn set_resizable(&self, resizable: bool) {
        unimplemented!()
    }

    #[inline]
    pub fn set_cursor(&self, cursor: MouseCursor) {
        unimplemented!()
    }

    #[inline]
    pub fn grab_cursor(&self, grab: bool) -> Result<(), String> {
        unimplemented!()
    }

    #[inline]
    pub fn hide_cursor(&self, hide: bool) {
        unimplemented!()
    }

    #[inline]
    pub fn get_hidpi_factor(&self) -> f64 {
        unimplemented!()
    }

    #[inline]
    pub fn set_cursor_position(&self, position: LogicalPosition) -> Result<(), String> {
        unimplemented!()
    }

    #[inline]
    pub fn set_maximized(&self, maximized: bool) {
    }

    #[inline]
    pub fn get_fullscreen(&self) -> Option<RootMonitorHandle> {
        unimplemented!()
    }

    #[inline]
    pub fn set_fullscreen(&self, monitor: Option<RootMonitorHandle>) {
        unimplemented!()
    }

    #[inline]
    pub fn set_decorations(&self, decorations: bool) {
        unimplemented!()
    }

    #[inline]
    pub fn set_always_on_top(&self, always_on_top: bool) {
        unimplemented!()
    }

    #[inline]
    pub fn set_window_icon(&self, window_icon: Option<Icon>) {
        unimplemented!()
    }

    #[inline]
    pub fn set_ime_spot(&self, position: LogicalPosition) {
        unimplemented!()
    }

    #[inline]
    pub fn request_redraw(&self) {
        unimplemented!()
    }

    #[inline]
    pub fn get_current_monitor(&self) -> RootMonitorHandle {
        unimplemented!()
    }

    #[inline]
    pub fn get_available_monitors(&self) -> VecDeque<MonitorHandle> {
        unimplemented!()
    }

    #[inline]
    pub fn get_primary_monitor(&self) -> MonitorHandle {
        unimplemented!()
    }
}


pub enum EventLoop<T: 'static> {
    TODO(T)
}

#[derive(Clone)]
pub enum EventLoopProxy<T: 'static> {
    TODO(T)
}

impl<T:'static> EventLoop<T> {
    pub fn new() -> EventLoop<T> {
        unimplemented!()
    }

    #[inline]
    pub fn get_available_monitors(&self) -> VecDeque<MonitorHandle> {
        unimplemented!()
    }

    #[inline]
    pub fn get_primary_monitor(&self) -> MonitorHandle {
        unimplemented!()
    }

    pub fn create_proxy(&self) -> EventLoopProxy<T> {
        unimplemented!()
    }

    pub fn run_return<F>(&mut self, callback: F)
        where F: FnMut(::event::Event<T>, &RootELW<T>, &mut ControlFlow)
    {
        unimplemented!()
    }

    pub fn run<F>(self, callback: F) -> !
        where F: 'static + FnMut(::event::Event<T>, &RootELW<T>, &mut ControlFlow)
    {
        unimplemented!()
    }

    pub fn window_target(&self) -> &::event_loop::EventLoopWindowTarget<T> {
        unimplemented!()
    }
}

impl<T: 'static> EventLoopProxy<T> {
    pub fn send_event(&self, event: T) -> Result<(), EventLoopClosed> {
        unimplemented!()
    }
}

pub enum EventLoopWindowTarget<T> {
    TODO(T)
}
