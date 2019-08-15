use crate::macros::*;
use crate::local::RoomName;
use crate::objects::RoomVisual;
use crate::objects::HasPosition;

pub struct LineStyle {
    width: u32,
    color: Option<String>,
    opacity: f64,
    line_style: Option<String> // Option? Enum?
}

// Memory alignment?
pub struct CircleStyle {
    radius: f64,
    fill: Option<String>, // Default value?
    opacity: f64,
    stroke: Option<String>, // Option? Enum?
    stroke_width: f64,
    line_style: Option<String> // Option? Enum?
}

// Memory alignment?
pub struct PolyStyle {
    fill: Option<String>, // Default value?
    opacity: f64,
    stroke: Option<String>, // Option? Enum?
    stroke_width: f64,
    line_style: Option<String> // Option? Enum?
}

// Memory alignment?
pub struct TextStyle {
    color: Option<String>,
    font: Option<String>, // JS also support a number here, no idea how to solve that one.
    stroke: Option<String>,
    stroke_width: f64,
    background_color: Option<String>,
    background_padding: f64,
    align: Option<String>,
    opacity: f64
}

js_serializable!(TextStyle);

impl RoomVisual {
    pub fn constructor(room_name: RoomName) -> Self {
        js_unwrap!(RoomVisual(@{room_name}))
        //js_unwrap!(new RoomVisual(@{room_name}))
    }

    pub fn room_name(self) -> String { 
        js_unwrap! { @{self}.roomName.unwrap() } // This should probably be transformed into a LocalRoomName instance
    }

    // pub fn line<T>(&self, pos1: &T, pos2: &T, style: LineStyle) -> &Self 
    // where
    //     T: ?Sized + HasPosition,
    // {
    //     js_unwrap_ref!(@{self}.line(pos1, pos2, @{style}))
    // }

    // pub fn circle<T>(&self, pos: &T, style: CircleStyle) -> &Self 
    // where
    //     T: ?Sized + HasPosition,
    // {
    //     js_unwrap_ref!(@{self}.circle(pos, @{style}))
    // }

    // pub fn rect<T>(&self, pos: &T, width: u32, height: u32, style: PolyStyle) -> &Self     
    // where
    //     T: ?Sized + HasPosition,
    // {
    //     js_unwrap_ref!(@{self}.rect(pos, width, height, style))
    // }

    // pub fn poly<T>(&self, points: Vec<&T>, style: PolyStyle) -> &Self 
    // where
    //     T: ?Sized + HasPosition,
    // {
    //     js_unwrap_ref!(@{self}.poly(points, @{style})) // Do we need to do something with points?
    // }

    pub fn text<T>(&self, pos: &T, style: TextStyle) -> Self 
    where
        T: ?Sized + HasPosition,
    {
        js_unwrap!(@{self}.text(pos, @{style}))
    }

    pub fn clear(&self) -> Self {
        js_unwrap!(@{self}.clear())
    }

    pub fn size(&self) -> usize {
        js_unwrap!(@{self}.size())
    }
}