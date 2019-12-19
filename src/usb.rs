//    println!("Manufacturer string: {}", fux.get_manufacturer_string().unwrap().unwrap());
//    println!("Product string: {}", fux.get_product_string().unwrap().unwrap());
extern crate itertools;
use itertools::Itertools;

use crate::hardware;

const FUX_VENDOR_ID: u16 = 0x03eb;
const FUX_PRODUCT_ID: u16 = 0xff07;

const TUX_SEND_LENGTH: u8 = 5;
const TUX_RECEIVE_LENGTH: usize = 64;
// const TUX_READ_LOOP_INTERVAL: f32 = 0.1;
const TUX_USB_ERROR_LIMIT: u8 = 20;
const TUX_USB_FREEZED_FRAMES_LIMIT: u8 = 10;

const FRAME_STATUS_REQUEST: [u8; 5] = [1, 1, 0, 0, 0];
const FRAME_RESET_DONGLE: [u8; 5] = [1, 1, 0, 0, 0xFE];
const FRAME_RESET_RF: [u8; 5] = [1, 1, 0, 0, 0xFD];
// const FRAME_BLINK_EYES: [u8; 5] = [0, 0x40, 2, 0, 0];

struct ProcessState {
    empty_frames: u8,
    last_frame_id: u8,
    repeated_frames: u8,
}

fn process_usb_frame(data: &[u8], state: &mut ProcessState) -> Option<Vec<Vec<u8>>> {
    let id_frame = data[0];
    let rf_state = data[1];
    let packet_count = data[3];

    println!(
        "Frame ID: {}, RF state: {}, Packet count: {}",
        id_frame, rf_state, packet_count
    );

    if id_frame == state.last_frame_id {
        state.repeated_frames += 1;
        eprintln!(
            "The ID of the received USB frame is the same as the previous ({})",
            state.repeated_frames
        );

        if state.repeated_frames >= 15 {
            state.repeated_frames = 0;
            state.last_frame_id = 255;
            eprintln!("The USB frame retrieving seems to be freezed ({})", 15);

            //info!("The RF connection will be reinitialized");
            //tux_usb_rf_reset();
        }

        return None;
    }

    state.last_frame_id = id_frame;

    if packet_count == 0 && rf_state == 1 {
        state.empty_frames += 1;
        println!("Consecutive frames without status: {}", state.empty_frames);
    } else {
        state.empty_frames = 0;
    }

    /*
    if (last_knowed_rf_state != rf_state)
    {
        last_knowed_rf_state = rf_state;
        if (rf_state_callback_function)
        {
            rf_state_callback_function(last_knowed_rf_state);
        }
    }
    */

    if packet_count > 15 {
        eprintln!("DONGLE ERROR: Statuses packets count is wrong (>15)");
        return None;
    }

    let mut result = Vec::new();
    let mut chunks = data[4..].chunks(4);
    for i in 0..packet_count {
        let chunk = chunks.next();
        match chunk {
            None => {
                eprintln!("Could not get next chunk");
            }
            Some(chunk) => {
                println!("Packet #{}: {:02x}", i, chunk.iter().format(""));
                result.push(chunk.to_vec());
            }
        }
        /*
        if (frame_callback_function)
        {
            frame_callback_function((unsigned char*)packet_data);
        }
        */
    }

    Some(result)
}

pub fn read_frame(device: &hidapi::HidDevice) -> Option<Vec<u8>> {
    match device.write(&FRAME_STATUS_REQUEST) {
        Ok(_size) => (), /*{
            println!("Write successful, wrote {} bytes", size);
        }*/
        Err(e) => {
            eprintln!("Error while writing: {}", e);
        }
    }

    let ten_millis = std::time::Duration::from_millis(10);
    std::thread::sleep(ten_millis);

    let mut buf = [0u8; TUX_RECEIVE_LENGTH];
    match device.read(&mut buf) {
        Ok(_size) => {
            // println!(
            //     "Read successful, got {} bytes: {:02x}",
            //     size,
            //     buf.iter().format("")
            // );
            Some(buf.to_vec())
        }
        Err(e) => {
            eprintln!("Error while reading: {}", e);
            None
        }
    }
}

/*
enum FrameHeader {
PORTS              0xC0
SENSORS1           0xC1
LIGHT              0xC2
POSITION1          0xC3
POSITION2          0xC4
IR                 0xC5
ID                 0xC6
BATTERY            0xC7
VERSION            0xC8
REVISION           0xC9
AUTHOR             0xCA
SOUND_VAR          0xCB
AUDIO              0xCC
FLASH_PROG         0xCD
LED                0xCE
PONG               0xFF
}
*/

fn process_packet(device: &hidapi::HidDevice, data: Vec<u8>) {
    match data[0] {
        0xC0 => {
            // println!(
            //     "B: 0x{:02x}, C: 0x{:02x}, D: 0x{:02x}",
            //     data[1], data[2], data[3]
            // );

            use std::convert::TryFrom as _;
            let copy = hardware::PortBBits::try_from(&data[1..2]);
            match copy {
                Err(e) => eprintln!("error: {}", e),
                Ok(c) => {
                    println!("Port B");
                    println!("  flippers_motor_backward: {}", c.get_flippers_motor_backward());
                    println!("  spin_motor_forward: {}", c.get_spin_motor_forward());
                    println!("  spin_motor_backward: {}", c.get_spin_motor_backward());
                    println!("  mouth_open_switch: {}", c.get_mouth_open_switch());
                    println!("  mouth_closed_switch: {}", c.get_mouth_closed_switch());
                    println!("  head_push_switch: {}", c.get_head_push_switch());
                    println!("  charger_inhibit_signal: {}", c.get_charger_inhibit_signal());
                    println!("  external_io: {}", c.get_external_io());

                    if c.get_head_push_switch() == false {
                        println!("Head push button PRESSED!");
                    }
                }
            }
        }
        0xC8 => {
            println!("Version packet");
            println!(" CPU: {}", (data[1] >> 5) & 0b0000_0111);
            println!(" Major: {}", data[1] & 0b0001_1111);
            println!(" Minor: {}", data[2]);
            println!(" Update: {}", data[3]);
        }
        _ => println!("something else"),
    }
}

pub fn read_usb_loop(device: &hidapi::HidDevice) {
    let timeout = std::time::Duration::from_millis(1);
    let initial_timeout = std::time::SystemTime::now();
    let mut current_timeout = initial_timeout;

    let mut state = ProcessState {
        empty_frames: 0,
        last_frame_id: 255,
        repeated_frames: 0,
    };

    loop {
        current_timeout += timeout;

        match read_frame(device) {
            Some(frame) => match process_usb_frame(&frame, &mut state) {
                Some(packets) => {
                    for packet in packets {
                        process_packet(device, packet);
                    }
                }
                None => eprintln!("Could not process frame"),
            },
            None => println!("Could not read frame"),
        }

        /*
        if (loop_cycle_complete_function)
        {
            loop_cycle_complete_function();
        }
        */

        // std::thread::sleep(std::time::Duration::from_millis(100));

        while std::time::SystemTime::now() < current_timeout {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}

pub fn tux_usb_rf_reset(device: &hidapi::HidDevice) -> hidapi::HidResult<usize> {
    device.write(&FRAME_RESET_RF)
}

pub fn tux_usb_reset(device: &hidapi::HidDevice) -> hidapi::HidResult<usize> {
    device.write(&FRAME_RESET_DONGLE)
}

/*
pub fn tux_usb_blink_eyes(device: &hidapi::HidDevice) -> hidapi::HidResult<usize> {
    let mut frame: [u8; 4] = [0; 4];
    frame[0] = UsbTuxCommands::EYES_BLINK_CMD as u8;

    tux_usb_send_to_tux(&device, &frame)
}

pub fn tux_usb_send_to_tux(device: &hidapi::HidDevice, data: &[u8; 4]) -> hidapi::HidResult<usize> {
    let frame: [u8; 5] = [0, data[0], data[1], data[2], data[3]];
    let result = device.write(&frame);

    std::thread::sleep(std::time::Duration::from_millis(1));

    result
}

pub fn tux_usb_mouth_open(device: &hidapi::HidDevice) -> hidapi::HidResult<usize> {
    let mut frame: [u8; 4] = [0; 4];

    frame[0] = UsbTuxCommands::MotorsConfigCmd as u8;
    frame[1] = MoveBodyPart::MOVE_EYES as u8;
    frame[2] = MoveSpeed::SPEED_HIGH as u8;
    let ret = tux_usb_send_to_tux(&device, &frame);

    frame[0] = UsbTuxCommands::MotorsSetCmd as u8;
    frame[1] = MoveBodyPart::MOVE_EYES as u8;
    // frame[2] = value;
    frame[3] = 0u8;
    let ret = tux_usb_send_to_tux(&device, &frame);

    device.write(&frame)
}
*/

