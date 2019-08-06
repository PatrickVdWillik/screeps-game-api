#[repr(transparent)]
pub struct RoomVisual {
}

// Memory alignment?
pub struct LineStyle {
    width: u32,
    color: str,
    opacity: f64,
    line_style: str // Option? Enum?
}

// Memory alignment?
pub struct CircleStyle {
    radius: f64,
    fill: str, // Default value?
    opacity: f64,
    stroke: str, // Option? Enum?
    stroke_width: f64,
    line_style: str // Option? Enum?
}

// Memory alignment?
pub struct PolyStyle {
    fill: str, // Default value?
    opacity: f64,
    stroke: str, // Option? Enum?
    stroke_width: f64,
    line_style: str // Option? Enum?
}

// Memory alignment?
pub struct TextStyle {
    color: str,
    font: str, // JS also support a number here, no idea how to solve that one.
    stroke: str,
    stroke_width: f64,
    background_color: str,
    background_padding: f64,
    align: str,
    opacity: f64
}

impl RoomVisual {
    // missing constructor?

    #[inline] // ???
    pub fn room_name(self) -> Option<str> {
        js_unwrap! { @{self.as_ref()}.roomName }
    }

    #[inline] // Replace RoomPosition with a where T?
    pub fn line(self, pos1: RoomPosition, pos2: RoomPosition, style: LineStyle) -> Self {

    }

    #[inline]
    pub fn circle(self, x: u32, y: u32, style: CircleStyle) -> Self {

    }

    #[inline]
    pub fn rect(self, x: u32, y: u32, width: u32, height: u32, style: PolyStyle) -> Self {

    }

    #[inline]
    pub fn poly(self, points: Vec<RoomPosition>, style: PolyStyle) -> Self {

    }

    #[inline]
    pub fn text(self, pos: RoomPosition, style: TextStyle) -> Self {

    }

    #[inline]
    pub fn clear(self) -> Self {

    }

    #[inline]
    pub fn size(self): usize {
        
    }
}