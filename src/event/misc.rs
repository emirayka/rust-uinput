use libc::c_int;
use ffi::*;
use {Event};
use super::{Kind, Code};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Msc {
    Serial,
    PulseLED,
    Gesture,
    Raw,
    Scan,
    Timestamp,
    Max,
    Cnt,
}

impl Into<Event> for Msc {
    fn into(self) -> Event {
        Event::Misc(self)
    }
}

impl Kind for Msc {
    fn kind(&self) -> c_int {
        EV_MSC
    }
}

impl Code for Msc {
    fn code(&self) -> c_int {
        match self {
            Msc::Serial => MSC_SERIAL,
            Msc::PulseLED => MSC_PULSELED,
            Msc::Gesture => MSC_GESTURE,
            Msc::Raw => MSC_RAW,
            Msc::Scan => MSC_SCAN,
            Msc::Timestamp => MSC_TIMESTAMP,
            Msc::Max => MSC_MAX,
            Msc::Cnt => MSC_CNT,
        }
    }
}
