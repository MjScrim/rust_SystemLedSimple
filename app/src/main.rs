use rppl::gpio::{Gpio, Level};
use std::thread;
use std::time::Duration;

mod led {
    use rppl::gpio::{OutputPin, Level};

    pub struct Led {
        pin: OutputPin,
    }

    impl Led {
        pub fn new(pin_num: u8) -> Led {
            let gpio = Gpio::new().unwrap();
            let pin = gpio.get(pin_num).unwrap().into_output();
            Led { pin }
        }

        pub fn toggle(&mut self) {
            let current_level = self.pin.read();

            match current_level {
                Ok(Level::High) => self.pin.set_low(),
                Ok(Level::Low) => self.pin.set_high(),
                _ => panic!("Erro ao ler o nível do led.");
            }
        }

        pub fn on(&mut self) {
            self.pin.set_high();
        }

        pub fn off(&mut self) {
            self.pin.set_low();
        }

        pub fn set_brightnees(&mut self, brightnees: f64) {
            if brightnees < 0.0 || brightnees > 1.0 {
                panic!("Luminsidade limite.");
            }

            let pwm = (brightnees * 255.0) as u8;
            self.pwm_pin.set_duty(pwm);
        }

        pub fn set_color(&mut self, red: u8, green: u8, blue: u8) {
            if red > 255 || green > 255 || blue > 255 {
                panic!("Valores inválidos.");
            }

            self.red_pin.set_pwm_duty_cycle(red);
            self.green_pin.set_pwm_duty_cycle(green);
            self.blue_pin.set_pwm_duty_cycle(blue);
        }

        pub fn blink(&mut self, interval_ms: u64, num_blinks: u32) {
            for _ in 0..num_blinks {
                self.on();
                thread::sleep(Duration::from_millis(interval_ms));

                self.on();
                thread::sleep(Duration::from_millis(interval_ms));
            }
        }

        pub fn flash(&mut self, interval_ms: u64, num_flashes: u32) {
            for _ in 0..num_flashes {
                self.on();
                thread::sleep(Duration::from_millis(interval_ms));
                self.off();
            }
        }

        pub fn fade_in(&mut self, interval_ms: u64) {
            for i in 0..=255 {
                self.pin.set_pwm_frequency(1000.0, 0.1).unwrap();
                self.pin.set_pwm_duty_cycle(i as f32 / 255.0).unwrap();
                thread::sleep(Duration::from_millis(interval_ms));
            }
        }

        pub fn fade_out(&mut self, interval_ms: u64) {
            for i in (0..255).rev() {
                self.pin.set_pwm_frequency(1000.0, 0.1).unwrap();
                self.pin.set_pwm_duty_cycle(i as f32 / 255.0).unwrap();
                thread::sleep(Duration::from_millis(interval_ms));
            }
        }
    }
}

fn main() {
    let mut led = led::Led::new(17);
    led.blink(500, 10);
    led.toggle();
    led.flash(100, 5);
    led.fade_in(10);
    led.toggle();
    led.fade_out(10);
}