#[repr(transparent)]
pub struct RoomVisual {
}


impl RoomVisual {
    #[inline]
    pub fn room_name(self) -> Option<str> {
        js_unwrap! { @{self.as_ref()}.roomName }
    }
}