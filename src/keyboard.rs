use color::Color;
use hidapi::{HidApi, HidError};

mod functions {
    pub const SET: u8 = 0x40;
    pub const GRAD: u8 = 0x44;
}

mod modes {
    pub const NORMAL: u8 = 0x01;
    pub const GAMING: u8 = 0x02;
    pub const BREATHING: u8 = 0x03;
    pub const AUDIO: u8 = 0x04;
    pub const WAVE: u8 = 0x05;
    pub const DUAL_COLOR: u8 = 0x06;
}

mod regions {
    pub const LEFT: u8 = 0x01;
    pub const MIDDLE: u8 = 0x02;
    pub const RIGHT: u8 = 0x03;
}

struct Keyboard {
    api: HidApi,
}

impl Keyboard {
    pub fn new() -> Result<Keyboard, HidError> {
        // TODO: Change the HidApi in the struct to a HidDevice
        let api = HidApi::new()?;
        Ok(Keyboard { api })
    }

    fn write_color(&self, function: u8, zone: u8, color: &Color) -> Result<(), HidError> {
        let data = [
            0x01,
            0x02,
            function,
            zone,
            color.r(),
            color.g(),
            color.b(),
            0x00,
        ];

        self.api.open(0x1770, 0xFF00)?.send_feature_report(&data)?;
        Ok(())
    }

    fn write_gradient(&self, region: u8, first: Color, second: Color) -> Result<(), HidError> {
        let index = (region - 1) * 3 + 1;
        self.write_color(functions::GRAD, index, &first)?;
        self.write_color(functions::GRAD, index + 1, &second)?;
        self.write_color(functions::GRAD, index + 2, &Color::new(0x03, 0x03, 0x03))?;
        Ok(())
    }

    fn write_mode(&self, mode: u8) -> Result<(), HidError> {
        let data = [0x01, 0x02, 0x41, mode, 0x00, 0x00, 0x00, 0x00];

        self.api.open(0x1770, 0xFF00)?.send_feature_report(&data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use color::Color;
    use keyboard::Keyboard;
    #[test]
    fn test_write_color() {
        // Not a unit test. Should change keyboard to normal mode
        // with Red/Green/Blue pattern in this order
        let c1 = Color::new(255, 0, 0);
        let c2 = Color::new(0, 255, 0);
        let c3 = Color::new(0, 0, 255);
        let k = Keyboard::new().unwrap();
        k.write_mode(0x01).unwrap();
        k.write_color(0x40, 1, &c1).unwrap();
        k.write_color(0x40, 2, &c2).unwrap();
        k.write_color(0x40, 3, &c3).unwrap();
    }
}
