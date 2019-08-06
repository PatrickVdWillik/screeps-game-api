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

    pub fn room_name(self) -> str { 
        js_unwrap! { @{self.as_ref()}.roomName } // This should probably be transformed into a LocalRoomName instance
    }

    pub fn line(&self, pos1: &T, pos2: &T, style: LineStyle) -> &Self 
    where
        T: ?Sized + HasPosition,
    {
        js_unwrap_ref!(@{self.as_ref()}.line(pos1, pos2, style))
    }

    pub fn circle(&self, pos: &T, style: CircleStyle) -> &Self 
    where
        T: ?Sized + HasPosition,
    {
        js_unwrap_ref!(@{self.as_ref()}.circle(pos, style))
    }

    pub fn rect(&self, pos: &T, width: u32, height: u32, style: PolyStyle) -> &Self     
    where
        T: ?Sized + HasPosition,
    {
        js_unwrap_ref!(@{self.as_ref()}.rect(pos, width, height, style))
    }

    pub fn poly(&self, points: Vec<&T>, style: PolyStyle) -> &Self 
    where
        T: ?Sized + HasPosition,
    {
        js_unwrap_ref!(@{self.as_ref()}.poly(points, style)) // Do we need to do something with points?
    }

    pub fn text(&self, pos: &T, style: TextStyle) -> &Self 
    where
        T: ?Sized + HasPosition,
    {
        js_unwrap_ref!(@{self.as_ref()}.text(pos, style))
    }

    pub fn clear(&self) -> &Self {
        js_unwrap_ref!(@{self.as_ref()}.clear())
    }

    pub fn size(&self): usize {
        js_unwrap!(@{self.as_ref()}.size())
    }
}