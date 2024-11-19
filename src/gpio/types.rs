use core::ops::Not;

/// Gpio Pin mode
#[derive(Clone, Copy)]
pub enum PinMode {
    Input = 0,
    Output = 1,
    Af = 2,
    Analog = 3,
}

/// Gpio Pin speed
#[derive(Clone, Copy)]
pub enum PinSpeed {
    VeryLow = 0,
    Low = 1,
    High = 2,
    VeryHigh = 3,
}

/// Gpio pin 功能复用
#[derive(Debug, Clone, Copy)]
pub enum PinAF {
    AF0 = 0,
    AF1 = 1,
    AF2 = 2,
    AF3 = 3,
    AF4 = 4,
    AF5 = 5,
    AF6 = 6,
    AF7 = 7,
    AF8 = 8,
    AF9 = 9,
    AF10 = 10,
    AF11 = 11,
    AF12 = 12,
    AF13 = 13,
    AF14 = 14,
    AF15 = 15,
}

impl From<u32> for PinAF {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::AF0,
            1 => Self::AF1,
            2 => Self::AF2,
            3 => Self::AF3,
            4 => Self::AF4,
            5 => Self::AF5,
            6 => Self::AF6,
            7 => Self::AF7,
            8 => Self::AF8,
            9 => Self::AF9,
            10 => Self::AF10,
            11 => Self::AF11,
            12 => Self::AF12,
            13 => Self::AF13,
            14 => Self::AF14,
            15 => Self::AF15,
            _ => unreachable!(),
        }
    }
}

/// Gpio pin 上下拉
#[derive(Clone, Copy)]
pub enum PinPullUpDown {
    No = 0,
    PullUp = 1,
    PullDown = 2,
}

/// Gpio pin 输出类型
#[derive(Clone, Copy)]
pub enum PinOutputType {
    PushPull = 0,
    OpenDrain = 1,
}

/// Gpio io 类型，综合了上拉和输出开漏模式
#[derive(Clone, Copy, PartialEq)]
pub enum PinIoType {
    Floating,
    PullUp,
    PullDown,
    OpenDrain,
}

impl PinIoType {
    pub(crate) fn split(self) -> (PinPullUpDown, PinOutputType) {
        let (push_pull, output_type) = match self {
            PinIoType::PullUp => (PinPullUpDown::PullUp, PinOutputType::PushPull),
            PinIoType::PullDown => (PinPullUpDown::PollDown, PinOutputType::PushPull),
            PinIoType::Floating => (PinPullUpDown::No, PinOutputType::PushPull),
            PinIoType::OpenDrain => (PinPullUpDown::No, PinOutputType::OpenDrain),
        };
        (push_pull, output_type)
    }
}

/// gpio 锁配置
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PinLock {
    Unlock = 0,
    Lock = 1,
}

/// Gpio io 电平
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PinLevel {
    Low = 0,
    Hight = 1,
}

impl Not for PinLevel {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self == Self::Low {
            Self::Hight
        } else {
            Self::Low
        }
    }
}

impl From<u32> for PinLevel {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Low,
            1 => Self::Hight,
            _ => unreachable!(),
        }
    }
}

impl From<PinLevel> for bool {
    fn from(value: PinLevel) -> Self {
        PinLevel::Hight == value
    }
}

#[derive(Debug)]
pub enum Error {}
