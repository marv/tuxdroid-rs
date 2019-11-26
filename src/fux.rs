use crate::tuxdroid::TuxDroid;

use crate::usb::read_usb_loop;
use crate::usb::tux_usb_reset;
use crate::usb::tux_usb_rf_reset;

mod commands;

const FUX_VENDOR_ID: u16 = 0x03eb;
const FUX_PRODUCT_ID: u16 = 0xff07;

const WAKEUP_FRAME: [u8; 4] = [0xB6, 0xFF, 0x01, 0x00];

enum CommandHeader {
    USB_HEADER_TUX = 0,
    USB_HEADER_DONGLE = 1,
    USB_HEADER_BOOTLOADER = 2,
}

enum DongleCommands {
    Connection = 0,
    Status = 1,
    Audio = 2,
    Version = 6,
}

enum DongleConnectionAction {
    Disconnect = 1,
    Connect = 2,
    IdRequest = 3,
    IdLookup = 4,
    ChangeId = 5,
    Wakeup = 6,
    WirelessChannel = 7,
}

pub enum UsbTuxCommands {
    EyesOpen = 0x33,
    EyesClose = 0x38,
    EyesBlink = 0x40,
    EyesStop = 0x32,

    MouthOpen = 0x34,
    MouthClose = 0x35,
    MouthMove = 0x41,
    MouthStop = 0x36,

    FlippersRaise = 0x39,
    FlippersLower = 0x3A,
    FlippersWave = 0x80,
    FlippersStop = 0x30,

    /*
    SPIN_LEFT_CMD = 0x83,
    SPIN_RIGHT_CMD = 0x82,
    SPIN_STOP_CMD = 0x37,
    */
    LedFadeSpeed = 0xD0,
    LedSet = 0xD1,
    LedPulseRange = 0xD2,
    LedPulse = 0xD3,

    /*
    TUX_PONG_PING_CMD = 0x7F,
    */
    MotorsSetCmd = 0xD4,
    MotorsConfigCmd = 0x81,
    /*
    TURN_IR_ON_CMD = 0x17,
    TURN_IR_OFF_CMD = 0x18,
    IR_SEND_RC5_CMD = 0x91,

    PLAY_SOUND_CMD = 0x90,
    STORE_SOUND_CMD = 0x52,
    CONFIRM_STORAGE_CMD = 0x53,
    ERASE_FLASH_CMD = 0x54,

    AUDIO_MUTE_CMD = 0x92,

    WIRELESS_FREQ_BOUNDARIES_CMD = 0x88,
    */
}

enum MoveBodyPart {
    Eyes = 0,
    Mouth = 1,
    Flippers = 2,
    SpinRight = 3,
    SpinLeft = 4,
}

enum MoveSpeed {
    Verylow = 1,
    Low = 2,
    Medium = 3,
    Midhigh = 4,
    High = 5,
}

enum MoveFinalState {
    Undefined = 0,
    OpenUp = 1,
    CloseDown = 2,
    Stop = 3,
}

/// ----

#[derive(Copy, Clone)]
pub enum Leds {
    NoLed = 0,
    Left = 0x01,
    Right = 0x02,
    Both = 0x03,
}

///! Types of effects applied when changing the intensity of the LEDs.
pub enum LedEffectType
{
    UNAFFECTED,     /**< Don't update the effect parameters. This can either be
                      the last effect set by software, or by firmware in the
                      autonomous mode. This is probably not what you want. */
    LAST,           /**< Last effect requested by software. */
    NONE,           /**< Don't use effects, equivalent to on/off mode. */
    DEFAULT,        /**< Default effect which is a short fading effect. */
    FADE_DURATION,  /**< Fading effect, 'effect.speed' sets the duration (in
                      seconds) the effect will last. */

    ///! Fading effect, 'effect.speed' sets the rate of the
    ///! effect. Its value represents the number of seconds it
    ///! takes to apply the effect from off to on. So the actual
    ///! effect duration will take less time than specified if the
    ///! intensity starts or ends at intermediate values.
    ///! Therefore this parameter guarantees a constant rate of
    ///! the effect, not the duration.
    FADE_RATE,

    ///! Gradient effect, the intensity changes gradually by a
    ///! number of steps given by 'effect.step'. 'effect.speed'
    ///! represents the number of seconds it should take to apply
    ///! the effect.
    GRADIENT_NBR,

    ///! Gradient effect, the intensity changes by a delta
    ///! value of 'effect.step'. 'effect.speed' represents the
    ///! number of seconds it should take to apply the effect.
    GRADIENT_DELTA
}

/// Fading or gradient effect. This structure holds the type of effect and the
/// corresponding parameters.
/// \sa The effect types and parameters are described in the documentation of
/// effect_type_t. */
pub struct LedEffect
{
    /// Type of effect.
    pub effect_type: LedEffectType,

    /// Speed of the effect, used in both gradients and fading effects.
    pub speed: f32,

    /// Intensity step of the gradient effect. Not used for the fading effect.
    pub step: i32,
}

pub struct Fux {
    //usb_thread: std::thread::Thread,
    running: bool,

    /// Our USB device
    device: hidapi::HidDevice,
}

pub struct Callbacks {
    //frame_callback: Option<Box<>>
}

fn find_fux() -> Option<hidapi::HidDevice> {
    println!("Trying to find fux");

    match hidapi::HidApi::new() {
        Ok(api) => {
            // let device = api.devices().iter().find(|device| device.vendor_id == FUX_VENDOR_ID && device.product_id == FUX_PRODUCT_ID);
            // match device {
            //     None => None,
            //     Some(d) => None
            // }
            for device in api.devices() {
                if device.vendor_id == FUX_VENDOR_ID && device.product_id == FUX_PRODUCT_ID {
                    match device.open_device(&api) {
                        Ok(dev) => {
                            println!("{:#?}", device);
                            return Some(dev);
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            return None;
                        }
                    }
                }
            }
            None
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    }
}

impl Fux {
    pub fn new() -> Option<Fux> {
        let fux = find_fux();
        if fux.is_none() {
            panic!("Error: Could not find fux :-(");
        }

        Some(Fux {
            device: fux.unwrap(),
            running: false,
            //usb_thread: std::thread::Thread::new(Some("tuxdroid".to_string()))
        })
    }

    pub fn reset(&self) -> hidapi::HidResult<usize> {
        tux_usb_reset(&self.device)
    }

    pub fn reset_rf(&self) -> hidapi::HidResult<usize> {
        tux_usb_rf_reset(&self.device)
    }

    fn send_to_dongle(&self, data: &[u8; 4]) -> hidapi::HidResult<usize> {
        let mut frame: [u8; 64] = [0; 64];
        frame[0] = 1;
        frame[1] = data[0];
        frame[2] = data[1];
        frame[3] = data[2];
        frame[4] = data[3];

        let result = self.device.write(&frame);

        std::thread::sleep(std::time::Duration::from_millis(1));

        result
    }

    fn send_to_tux(&self, data: &[u8; 4]) -> hidapi::HidResult<usize> {
        //let frame: [u8; 5] = [0, data[0], data[1], data[2], data[3]];
        let mut frame: [u8; 65] = [0; 65];
        frame[0] = 0;
        frame[1] = 0;
        frame[2] = data[0];
        frame[3] = data[1];
        frame[4] = data[2];
        frame[5] = data[3];

        let result = self.device.write(&frame);

        std::thread::sleep(std::time::Duration::from_millis(1));

        result
    }

    /// Convenience function to send simple (1-byte) commands to Tux
    pub fn send_tux_simple_command(&self, cmd: UsbTuxCommands) -> bool {
        let frame: [u8; 4] = [cmd as u8, 0, 0, 0];

        match self.send_to_tux(&frame) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn get_status(&self) -> hidapi::HidResult<usize> {
        let frame: [u8; 4] = [DongleCommands::Status as u8, 0, 0, 0];

        self.send_to_dongle(&frame)
    }

    pub fn get_id(&self) -> hidapi::HidResult<usize> {
        let frame: [u8; 4] = [
            DongleCommands::Connection as u8,
            DongleConnectionAction::IdRequest as u8,
            0,
            0,
        ];

        self.send_to_dongle(&frame)
    }

    pub fn wake_up_tux(&self) {
        match self.send_to_tux(&WAKEUP_FRAME) {
            Ok(bytes) => println!("Wake up frame written ({} bytes)", bytes),
            Err(e) => eprintln!("Could not write wakeup frame: {}", e),
        }
    }

    pub fn start(&self) -> bool {
        if self.running {
            return false;
        }

        read_usb_loop(&self.device);

        true
    }

    pub fn eyes_close(&self) -> hidapi::HidResult<usize> {
        let mut frame: [u8; 4] = [0; 4];
        frame[0] = UsbTuxCommands::EyesClose as u8;

        self.send_to_tux(&frame)
    }

    pub fn eyes_open(&self) -> hidapi::HidResult<usize> {
        let mut frame: [u8; 4] = [0; 4];
        frame[0] = UsbTuxCommands::EyesOpen as u8;

        self.send_to_tux(&frame)
    }

    pub fn eyes_on(&self) -> hidapi::HidResult<usize> {
        let mut frame: [u8; 4] = [0; 4];
        frame[0] = UsbTuxCommands::LedSet as u8;
        frame[1] = Leds::Both as u8;
        frame[2] = 255;

        self.send_to_tux(&frame)
    }

    pub fn led_pulse(
        &self,
        leds: Leds,
        min_intensity: u8,
        max_intensity: u8,
        toggle_count: u8,
        pulse_period: f32,
        effect: &LedEffect,
    ) -> bool {
        let mut frame: [u8; 4] = [0, 0, 0, 0];
        /*
        bool ret;

        /* Pulse width or duration of the pulse, in hardware loops. The pulse
         * period is twice that number. */
        int pulse_width;
        int delta;

        min_intensity = bound_to_range(min_intensity, 0, 255);
        max_intensity = bound_to_range(max_intensity, 0, 255);

        if (min_intensity > max_intensity)
        {
            min_intensity = max_intensity;
        }

        /* TODO right now the limitation is the firmware limitation of 255, if we
         * want to overcome this limitation, this libary should split the user
         * command into multiple commands sent over time in order to achieve the
         * required effect. i.e. 500 toggles could be split into 2 commands of
         * 200 toggles and one of 100, each command sent when the previous one is
         * completed. */
        toggle_count = bound_to_range(toggle_count, 1, 255);

        let pulse_width = roundf(pulse_period/FW_MAIN_LOOP_DELAY/2);

        /* TODO right now the limitation is the firmware limitation of 255, if we
         * want to overcome this limitation, this libary should split the user
         * command into multiple commands sent over time in order to achieve the
         * required effect. i.e. 500 could be split into 2 commands of
         * 200 and one of 100, each command sent when the previous one is
         * completed. */
        pulse_width = bound_to_range(pulse_width, 1, 255);

        delta = max_intensity - min_intensity;

        ret = led_configure_effects(leds, delta, delta, effect);
        */

        frame[0] = UsbTuxCommands::LedPulseRange as u8;
        frame[1] = leds as u8;
        frame[2] = max_intensity;
        frame[3] = min_intensity;

        let ret = self.send_to_tux(&frame);

        frame[0] = UsbTuxCommands::LedPulse as u8;
        frame[1] = leds as u8;
        frame[2] = toggle_count;
        frame[3] = 150; //pulse_width;

        let ret = self.send_to_tux(&frame);

        true
    }
}

impl TuxDroid for Fux {
    fn eyes_blink(&self) -> bool {
        self.send_tux_simple_command(UsbTuxCommands::EyesBlink)
    }

    fn open_mouth(&self) -> bool {
        self.send_tux_simple_command(UsbTuxCommands::MouthOpen)
    }

    fn close_mouth(&self) -> bool {
        self.send_tux_simple_command(UsbTuxCommands::MouthClose)
    }

    fn flippers_raise(&self) -> bool {
        self.send_tux_simple_command(UsbTuxCommands::FlippersRaise)
    }

    fn flippers_lower(&self) -> bool {
        self.send_tux_simple_command(UsbTuxCommands::FlippersLower)
    }

    fn flippers_wave(&self) -> bool {
        self.send_tux_simple_command(UsbTuxCommands::FlippersWave)
    }

    fn flippers_stop(&self) -> bool {
        self.send_tux_simple_command(UsbTuxCommands::FlippersStop)
    }
}
