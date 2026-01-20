#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioReverbPreset {
    #[default]
    Off = 0,
    Generic = 1,
    PaddedCell = 2,
    Room = 3,
    Bathroom = 4,
    Livingroom = 5,
    Stoneroom = 6,
    Auditorium = 7,
    Concerthall = 8,
    Cave = 9,
    Arena = 10,
    Hangar = 11,
    CarpetedHallway = 12,
    Hallway = 13,
    StoneCorridor = 14,
    Alley = 15,
    Forest = 16,
    City = 17,
    Mountains = 18,
    Quarry = 19,
    Plain = 20,
    ParkingLot = 21,
    SewerPipe = 22,
    Underwater = 23,
    Drugged = 24,
    Dizzy = 25,
    Psychotic = 26,
    User = 27,
}
