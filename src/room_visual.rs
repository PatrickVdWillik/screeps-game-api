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
        js_unwrap! { @{self.as_ref()}.roomName }
    }

    pub fn line(&self, pos1: &T, pos2: &T, style: LineStyle) -> &Self 
    where
        T: ?Sized + HasPosition,
    {

    }

    pub fn circle(&self, pos: &T, style: CircleStyle) -> &Self 
    where
        T: ?Sized + HasPosition,
    {

    }

    #[inline]
    pub fn rect(&self, pos: &T, width: u32, height: u32, style: PolyStyle) -> &Self     
    where
        T: ?Sized + HasPosition,
    {

    }

    pub fn poly(&self, points: Vec<&T>, style: PolyStyle) -> &Self 
    where
        T: ?Sized + HasPosition,
    {

    }

    pub fn text(&self, pos: &T, style: TextStyle) -> &Self 
    where
        T: ?Sized + HasPosition,
    {

    }

    pub fn clear(&self) -> &Self {

    }

    pub fn size(&self): usize {

    }
}