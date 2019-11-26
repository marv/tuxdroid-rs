pub enum UsbTuxCommands {
    EyesOpenCmd = 0x33,
    EyesCloseCmd = 0x38,
    EYES_BLINK_CMD = 0x40,
    EYES_STOP_CMD = 0x32,

    MouthOpenCmd = 0x34,
    MouthCloseCmd = 0x35,
    MOUTH_MOVE_CMD = 0x41,
    MOUTH_STOP_CMD = 0x36,

    FLIPPERS_RAISE_CMD = 0x39,
    FLIPPERS_LOWER_CMD = 0x3A,
    FLIPPERS_WAVE_CMD = 0x80,
    FLIPPERS_STOP_CMD = 0x30,

    SPIN_LEFT_CMD = 0x83,
    SPIN_RIGHT_CMD = 0x82,
    SPIN_STOP_CMD = 0x37,

    LED_FADE_SPEED_CMD = 0xD0,
    LED_SET_CMD = 0xD1,
    LED_PULSE_RANGE_CMD = 0xD2,
    LED_PULSE_CMD = 0xD3,

    TUX_PONG_PING_CMD = 0x7F,

    MotorsSetCmd = 0xD4,
    MotorsConfigCmd = 0x81,

    TURN_IR_ON_CMD = 0x17,
    TURN_IR_OFF_CMD = 0x18,
    IR_SEND_RC5_CMD = 0x91,

    PLAY_SOUND_CMD = 0x90,
    STORE_SOUND_CMD = 0x52,
    CONFIRM_STORAGE_CMD = 0x53,
    ERASE_FLASH_CMD = 0x54,

    AUDIO_MUTE_CMD = 0x92,

    WIRELESS_FREQ_BOUNDARIES_CMD = 0x88,
}

enum MoveBodyPart {
    MOVE_EYES = 0,
    MOVE_MOUTH = 1,
    MOVE_FLIPPERS = 2,
    MOVE_SPIN_R = 3,
    MOVE_SPIN_L = 4,
}

enum MoveSpeed {
    SPEED_VERYLOW = 1,
    SPEED_LOW = 2,
    SPEED_MEDIUM = 3,
    SPEED_MIDHIGH = 4,
    SPEED_HIGH = 5,
}

enum MoveFinalState {
    FINAL_ST_UNDEFINED = 0,
    FINAL_ST_OPEN_UP = 1,
    FINAL_ST_CLOSE_DOWN = 2,
    FINAL_ST_STOP = 3,
}


