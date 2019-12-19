use modular_bitfield::prelude::*;

#[bitfield]
#[derive(PartialEq, Eq, Debug)]
pub struct PortBBits
{
    flippers_motor_backward: bool,
    spin_motor_forward: bool,
    spin_motor_backward: bool,
    mouth_open_switch: bool,
    mouth_closed_switch: bool,
    head_push_switch: bool,
    charger_inhibit_signal: bool,
    external_io: bool,
}

/*
union
{
    Byte: u8;
    _PORTB_BITS_    bits;
} _PORTB_BYTE_;

struct PortCBits
{
    _BIT_ photo_transistor_pull_up:1;
    _BIT_ flippers_position_switch:1;
    _BIT_ right_blue_led:1;
    _BIT_ left_blue_led:1;
    _BIT_ i2c_sda_line:1;
    _BIT_ i2c_scl_line:1;
    _BIT_ reset:1;
    _BIT_ ndef:1;
} _PORTC_BITS_;

union
{
    Byte: u8;
    _PORTC_BITS_    bits;
} _PORTC_BYTE_;

struct PortDBits
{
    _BIT_ head_motor_for_mouth:1;
    _BIT_ head_motor_for_eyes:1;
    _BIT_ ir_receiver_signal:1;
    _BIT_ spin_position_switch:1;
    _BIT_ flippers_motor_forward:1;
    _BIT_ ir_led:1;
    _BIT_ eyes_open_switch:1;
    _BIT_ eyes_closed_switch:1;
} _PORTD_BITS_;

union
{
    Byte: u8;
    _PORTD_BITS_    bits;
} _PORTD_BYTE_;

struct SensorStates
{
    _BIT_ left_wing_push_button:1;
    _BIT_ right_wing_push_button:1;
    _BIT_ power_plug_insertion_switch:1;
    _BIT_ head_push_button:1;
    _BIT_ charger_led_status:1;
    _BIT_ rf_connection_status:1;
    _BIT_ internal_power_switch:1;
    _BIT_ mute_status:1;
} _SENSORS_BITS_;

union
{
    Byte: u8;
    _SENSORS_BITS_    bits;
} _SENSORS_BYTE_;

struct InfraredState
{
    _BIT_ command:6;
    _BIT_ toggle:1;
    _BIT_ received_flag:1;
} _RC5_BITS_;

union
{
    Byte: u8;
    _RC5_BITS_   bits;
} _RC5_BYTE_;

struct Version
{
    _BIT_ cpu_number:3;
    _BIT_ major:5;
} _VERSION_FIRST_BITS_;

union
{
    Byte: u8;
    _VERSION_FIRST_BITS_    bits;
} _VERSION_FIRST_BYTE_;

struct
{
    _BIT_ local_modification:1;
    _BIT_ mixed_update:1;
    _BIT_ original_release:1;
    _BIT_ ndef:5;
} _REVISION_THIRD_BITS_;

union
{
    Byte: u8;
    _REVISION_THIRD_BITS_   bits;
} _REVISION_THIRD_BYTE_;

struct AudioState
{
    _BIT_ no_programming:1;
    _BIT_ flash_erased:1;
    _BIT_ toc:1;
    _BIT_ sounds_track:5;

} _AUDIO_BITS_;

union
{
    Byte: u8;
    _AUDIO_BITS_   bits;
} _AUDIO_BYTE_;

struct LedEffectState
{
    _BIT_ left_led_fading:1;
    _BIT_ left_led_pulsing:1;
    _BIT_ right_led_fading:1;
    _BIT_ right_led_pulsing:1;
    _BIT_ led_mask:1;
    _BIT_ ndef:3;
} _LED_EFFECT_STATUS_BITS_;

union
{
    Byte: u8;
    _LED_EFFECT_STATUS_BITS_   bits;
} _LED_EFFECT_STATUS_BYTE_;

struct MotorsState
{
    _BIT_ spin_right_on:1;
    _BIT_ spin_left_on:1;
    _BIT_ eyes_on:1;
    _BIT_ mouth_on:1;
    _BIT_ flippers_on:1;
    _BIT_ ndef:3;
};
*/

/*
union
{
    Byte: u8;
    _MOTORS_STATUS_BITS_       bits;
} _MOTORS_STATUS_BYTE_;

struct FrameBodyPorts
{
    _PORTB_BYTE_ portb;
    _PORTC_BYTE_ portc;
    _PORTD_BYTE_ portd;
};

struct FrameBodySensors1
{
    sensors: SensorStates;
    play_internal_sound: u8;
    play_general_sound: u8;
};
*/

struct FrameBodyLight
{
    high_level: u8,
    low_level: u8,
    mode: u8,
}

struct FrameBodyPosition1
{
    eyes_remaining_mvm: u8,
    mouth_remaining_mvm: u8,
    flippers_remaining_mvm: u8,
}

/*
struct FrameBodyPosition2
{
    spin_remaining_mvm: u8,
    flippers_down: u8,
    motors: MotorsState,
}

struct FrameBodyIR
{
    _RC5_BYTE_     rc5_code,
    /*unsigned char   ??;                       NDEF */
    /*unsigned char   ??;                       NDEF */
}

struct FrameBodyId
{
    msb_number: u8;
    lsb_number: u8;
    /*unsigned char   ??;                       NDEF */
};

struct FrameBodyBattery
{
    high_level: u8;
    low_level: u8;
    motors_state: u8;
};

struct FrameBodyVersion
{
    _VERSION_FIRST_BYTE_     cm;
    minor: u8;
    update: u8;
};

struct FrameBodyRevision
{
    lsb_number: u8;
    msb_number: u8;
    _REVISION_THIRD_BYTE_    release_type;
};

struct FrameBodyAuthor
{
    lsb_id: u8;
    msb_id: u8;
    variation_number: u8;
};

struct FrameBodyAudio
{
    sound_track_played: u8;
    _AUDIO_BYTE_    programming_steps;
    programmed_sound_track: u8;
};

struct FrameBodySoundVar
{
    number_of_sounds: u8;
    flash_usage: u8;
    /*unsigned char   ??;                       NDEF */
};

struct FrameBodyFlashProg
{
    current_state: u8;
    last_sound_size: u8;
    /*unsigned char   ??;                       NDEF */
};

struct FrameBodyLed
{
    left_led_intensity: u8;
    right_led_intensity: u8;
    _LED_EFFECT_STATUS_BYTE_    effect_status;
};

struct FrameBodyPong
{
    pongs_pending_number: u8;
    pongs_lost_by_i2c_number: u8;
    pongs_lost_by_rf_number: u8;
};

struct HardwareState
{
    ports: FrameBodyPorts;
    sensors1: FrameBodySensors1;
    light: FrameBodyLight;
    position1: FrameBodyPosition1;
    position2: FrameBodyPosition2;
    ir: frame_body_ir_t;
    id: frame_body_id_t;
    battery: frame_body_battery_t;
    version: frame_body_version_t;
    revision: frame_body_revision_t;
    author: frame_body_author_t;
    audio: frame_body_audio_t;
    sound_var: frame_body_sound_var_t;
    flash_prog: frame_body_flash_prog_t;
    led: frame_body_led_t;
    pong: frame_body_pong_t;
};
*/
