// automatically generated
use wheel::{WheelInit, Wheel, WheelElem};

#[derive(Debug)]
pub struct Wheel210;
impl Wheel for Wheel210 {
    #[inline(always)]
    fn modulo(&self) -> usize { MODULO }

    #[inline(always)]
    fn size(&self) -> usize { SIZE }

    #[inline(always)]
    fn wheel(&self) -> &'static [WheelElem] { WHEEL }

    #[inline(always)]
    fn init(&self) -> &'static [WheelInit] { INIT }

    #[inline(always)]
    unsafe fn hardcoded_sieve(&self,
                              bytes: &mut [u8], si_: &mut usize, wi_: &mut usize, prime: usize) {
        hardcoded_sieve(bytes, si_, wi_, prime)
    }
}

pub const SIZE: usize = 48;

pub const MODULO: usize = 210;

#[allow(dead_code)]
pub const SMALL_BITS: usize = 2304;
#[allow(dead_code)]
pub const SMALL: &'static [u64; SMALL_BITS / 64] = &[
    0b1111100100111101110110111011011001111110111011111101111111111110,
    0b1001111011101101101001101110101011110011000111100100111111010101,
    0b1010010101011001110111010011101111010011110100110000110011100110,
    0b0101011010100110000111100111100010111101100100100110011101101010,
    0b0101010101001100001010101101111000101101101011011110001101010110,
    0b1111100010100001010101000000001110011111111100001010001111011001,
    0b0011101000010011111101100110011011101001010001001111110100101110,
    0b0101010010111111000100010100010100111010001010110100110010111000,
    0b0100111110001100101111001100100010110011011110101100000110001100,
    0b1110111100010111110000011001101101110001011100010101100000100001,
    0b0100011010001100100000111110010100001000000110101001011001010100,
    0b1000011101011000100011111001001001100101101011101111101101110010,
    0b1010000011100011001001100110010110000001110110001001001011010010,
    0b1001100111101011100000010011110000100110110001110011100000010001,
    0b0100110100110011111100110010010000111110100010000101000110001101,
    0b0100110001011000101101000010101010100111000111001000101101011010,
    0b1100001110000011110111001000001000011001111101100010011001001110,
    0b0000001011001101110011011011010100000010001110001111000100101100,
    0b0011000001111010010011000101011100001100100101000100101010110010,
    0b1111100000100100011011000100010011001011111100010000101101000011,
    0b1000110111101010011100110101110010101000100101010000000100011001,
    0b1100010000011110001000101010011001010000001010111001011000100100,
    0b1001110001110100001011110011101011010100000001100100100011010001,
    0b0010111000010101011010001011111110001000000001010110101000000111,
    0b0001010000001000100110000101000110110111111000110101010101100000,
    0b0010011101110000010010010100110101000101101010100101101010000110,
    0b0110000110000011000000101010101111001010110101011001001111010010,
    0b1010110110101001110000100010001010000111110011100010010000000101,
    0b1011000000010110100010011101000101111000010011011000110000011000,
    0b0101001000100100001101001100000010100010011000101100011101010111,
    0b0100001100001000001000011000110100110010010000000101101010101110,
    0b0110000011100001000110011101100110110110110100101011011000110100,
    0b1001010001111010010001001101000001100000001110010001101001100111,
    0b0001000001010101011101001010100010000011100010000000100110011010,
    0b0011001011001000001000110001111001101000010111011010000100100111,
    0b0011100010110001010010000111001101000100000000110001100111100000,
];
const INIT: &'static [WheelInit; 210] = &[
    WheelInit { next_mult_factor: 1, wheel_index: 0 }, // 0
    WheelInit { next_mult_factor: 0, wheel_index: 0 }, // 1
    WheelInit { next_mult_factor: 9, wheel_index: 1 }, // 2
    WheelInit { next_mult_factor: 8, wheel_index: 1 }, // 3
    WheelInit { next_mult_factor: 7, wheel_index: 1 }, // 4
    WheelInit { next_mult_factor: 6, wheel_index: 1 }, // 5
    WheelInit { next_mult_factor: 5, wheel_index: 1 }, // 6
    WheelInit { next_mult_factor: 4, wheel_index: 1 }, // 7
    WheelInit { next_mult_factor: 3, wheel_index: 1 }, // 8
    WheelInit { next_mult_factor: 2, wheel_index: 1 }, // 9
    WheelInit { next_mult_factor: 1, wheel_index: 1 }, // 10
    WheelInit { next_mult_factor: 0, wheel_index: 1 }, // 11
    WheelInit { next_mult_factor: 1, wheel_index: 2 }, // 12
    WheelInit { next_mult_factor: 0, wheel_index: 2 }, // 13
    WheelInit { next_mult_factor: 3, wheel_index: 3 }, // 14
    WheelInit { next_mult_factor: 2, wheel_index: 3 }, // 15
    WheelInit { next_mult_factor: 1, wheel_index: 3 }, // 16
    WheelInit { next_mult_factor: 0, wheel_index: 3 }, // 17
    WheelInit { next_mult_factor: 1, wheel_index: 4 }, // 18
    WheelInit { next_mult_factor: 0, wheel_index: 4 }, // 19
    WheelInit { next_mult_factor: 3, wheel_index: 5 }, // 20
    WheelInit { next_mult_factor: 2, wheel_index: 5 }, // 21
    WheelInit { next_mult_factor: 1, wheel_index: 5 }, // 22
    WheelInit { next_mult_factor: 0, wheel_index: 5 }, // 23
    WheelInit { next_mult_factor: 5, wheel_index: 6 }, // 24
    WheelInit { next_mult_factor: 4, wheel_index: 6 }, // 25
    WheelInit { next_mult_factor: 3, wheel_index: 6 }, // 26
    WheelInit { next_mult_factor: 2, wheel_index: 6 }, // 27
    WheelInit { next_mult_factor: 1, wheel_index: 6 }, // 28
    WheelInit { next_mult_factor: 0, wheel_index: 6 }, // 29
    WheelInit { next_mult_factor: 1, wheel_index: 7 }, // 30
    WheelInit { next_mult_factor: 0, wheel_index: 7 }, // 31
    WheelInit { next_mult_factor: 5, wheel_index: 8 }, // 32
    WheelInit { next_mult_factor: 4, wheel_index: 8 }, // 33
    WheelInit { next_mult_factor: 3, wheel_index: 8 }, // 34
    WheelInit { next_mult_factor: 2, wheel_index: 8 }, // 35
    WheelInit { next_mult_factor: 1, wheel_index: 8 }, // 36
    WheelInit { next_mult_factor: 0, wheel_index: 8 }, // 37
    WheelInit { next_mult_factor: 3, wheel_index: 9 }, // 38
    WheelInit { next_mult_factor: 2, wheel_index: 9 }, // 39
    WheelInit { next_mult_factor: 1, wheel_index: 9 }, // 40
    WheelInit { next_mult_factor: 0, wheel_index: 9 }, // 41
    WheelInit { next_mult_factor: 1, wheel_index: 10 }, // 42
    WheelInit { next_mult_factor: 0, wheel_index: 10 }, // 43
    WheelInit { next_mult_factor: 3, wheel_index: 11 }, // 44
    WheelInit { next_mult_factor: 2, wheel_index: 11 }, // 45
    WheelInit { next_mult_factor: 1, wheel_index: 11 }, // 46
    WheelInit { next_mult_factor: 0, wheel_index: 11 }, // 47
    WheelInit { next_mult_factor: 5, wheel_index: 12 }, // 48
    WheelInit { next_mult_factor: 4, wheel_index: 12 }, // 49
    WheelInit { next_mult_factor: 3, wheel_index: 12 }, // 50
    WheelInit { next_mult_factor: 2, wheel_index: 12 }, // 51
    WheelInit { next_mult_factor: 1, wheel_index: 12 }, // 52
    WheelInit { next_mult_factor: 0, wheel_index: 12 }, // 53
    WheelInit { next_mult_factor: 5, wheel_index: 13 }, // 54
    WheelInit { next_mult_factor: 4, wheel_index: 13 }, // 55
    WheelInit { next_mult_factor: 3, wheel_index: 13 }, // 56
    WheelInit { next_mult_factor: 2, wheel_index: 13 }, // 57
    WheelInit { next_mult_factor: 1, wheel_index: 13 }, // 58
    WheelInit { next_mult_factor: 0, wheel_index: 13 }, // 59
    WheelInit { next_mult_factor: 1, wheel_index: 14 }, // 60
    WheelInit { next_mult_factor: 0, wheel_index: 14 }, // 61
    WheelInit { next_mult_factor: 5, wheel_index: 15 }, // 62
    WheelInit { next_mult_factor: 4, wheel_index: 15 }, // 63
    WheelInit { next_mult_factor: 3, wheel_index: 15 }, // 64
    WheelInit { next_mult_factor: 2, wheel_index: 15 }, // 65
    WheelInit { next_mult_factor: 1, wheel_index: 15 }, // 66
    WheelInit { next_mult_factor: 0, wheel_index: 15 }, // 67
    WheelInit { next_mult_factor: 3, wheel_index: 16 }, // 68
    WheelInit { next_mult_factor: 2, wheel_index: 16 }, // 69
    WheelInit { next_mult_factor: 1, wheel_index: 16 }, // 70
    WheelInit { next_mult_factor: 0, wheel_index: 16 }, // 71
    WheelInit { next_mult_factor: 1, wheel_index: 17 }, // 72
    WheelInit { next_mult_factor: 0, wheel_index: 17 }, // 73
    WheelInit { next_mult_factor: 5, wheel_index: 18 }, // 74
    WheelInit { next_mult_factor: 4, wheel_index: 18 }, // 75
    WheelInit { next_mult_factor: 3, wheel_index: 18 }, // 76
    WheelInit { next_mult_factor: 2, wheel_index: 18 }, // 77
    WheelInit { next_mult_factor: 1, wheel_index: 18 }, // 78
    WheelInit { next_mult_factor: 0, wheel_index: 18 }, // 79
    WheelInit { next_mult_factor: 3, wheel_index: 19 }, // 80
    WheelInit { next_mult_factor: 2, wheel_index: 19 }, // 81
    WheelInit { next_mult_factor: 1, wheel_index: 19 }, // 82
    WheelInit { next_mult_factor: 0, wheel_index: 19 }, // 83
    WheelInit { next_mult_factor: 5, wheel_index: 20 }, // 84
    WheelInit { next_mult_factor: 4, wheel_index: 20 }, // 85
    WheelInit { next_mult_factor: 3, wheel_index: 20 }, // 86
    WheelInit { next_mult_factor: 2, wheel_index: 20 }, // 87
    WheelInit { next_mult_factor: 1, wheel_index: 20 }, // 88
    WheelInit { next_mult_factor: 0, wheel_index: 20 }, // 89
    WheelInit { next_mult_factor: 7, wheel_index: 21 }, // 90
    WheelInit { next_mult_factor: 6, wheel_index: 21 }, // 91
    WheelInit { next_mult_factor: 5, wheel_index: 21 }, // 92
    WheelInit { next_mult_factor: 4, wheel_index: 21 }, // 93
    WheelInit { next_mult_factor: 3, wheel_index: 21 }, // 94
    WheelInit { next_mult_factor: 2, wheel_index: 21 }, // 95
    WheelInit { next_mult_factor: 1, wheel_index: 21 }, // 96
    WheelInit { next_mult_factor: 0, wheel_index: 21 }, // 97
    WheelInit { next_mult_factor: 3, wheel_index: 22 }, // 98
    WheelInit { next_mult_factor: 2, wheel_index: 22 }, // 99
    WheelInit { next_mult_factor: 1, wheel_index: 22 }, // 100
    WheelInit { next_mult_factor: 0, wheel_index: 22 }, // 101
    WheelInit { next_mult_factor: 1, wheel_index: 23 }, // 102
    WheelInit { next_mult_factor: 0, wheel_index: 23 }, // 103
    WheelInit { next_mult_factor: 3, wheel_index: 24 }, // 104
    WheelInit { next_mult_factor: 2, wheel_index: 24 }, // 105
    WheelInit { next_mult_factor: 1, wheel_index: 24 }, // 106
    WheelInit { next_mult_factor: 0, wheel_index: 24 }, // 107
    WheelInit { next_mult_factor: 1, wheel_index: 25 }, // 108
    WheelInit { next_mult_factor: 0, wheel_index: 25 }, // 109
    WheelInit { next_mult_factor: 3, wheel_index: 26 }, // 110
    WheelInit { next_mult_factor: 2, wheel_index: 26 }, // 111
    WheelInit { next_mult_factor: 1, wheel_index: 26 }, // 112
    WheelInit { next_mult_factor: 0, wheel_index: 26 }, // 113
    WheelInit { next_mult_factor: 7, wheel_index: 27 }, // 114
    WheelInit { next_mult_factor: 6, wheel_index: 27 }, // 115
    WheelInit { next_mult_factor: 5, wheel_index: 27 }, // 116
    WheelInit { next_mult_factor: 4, wheel_index: 27 }, // 117
    WheelInit { next_mult_factor: 3, wheel_index: 27 }, // 118
    WheelInit { next_mult_factor: 2, wheel_index: 27 }, // 119
    WheelInit { next_mult_factor: 1, wheel_index: 27 }, // 120
    WheelInit { next_mult_factor: 0, wheel_index: 27 }, // 121
    WheelInit { next_mult_factor: 5, wheel_index: 28 }, // 122
    WheelInit { next_mult_factor: 4, wheel_index: 28 }, // 123
    WheelInit { next_mult_factor: 3, wheel_index: 28 }, // 124
    WheelInit { next_mult_factor: 2, wheel_index: 28 }, // 125
    WheelInit { next_mult_factor: 1, wheel_index: 28 }, // 126
    WheelInit { next_mult_factor: 0, wheel_index: 28 }, // 127
    WheelInit { next_mult_factor: 3, wheel_index: 29 }, // 128
    WheelInit { next_mult_factor: 2, wheel_index: 29 }, // 129
    WheelInit { next_mult_factor: 1, wheel_index: 29 }, // 130
    WheelInit { next_mult_factor: 0, wheel_index: 29 }, // 131
    WheelInit { next_mult_factor: 5, wheel_index: 30 }, // 132
    WheelInit { next_mult_factor: 4, wheel_index: 30 }, // 133
    WheelInit { next_mult_factor: 3, wheel_index: 30 }, // 134
    WheelInit { next_mult_factor: 2, wheel_index: 30 }, // 135
    WheelInit { next_mult_factor: 1, wheel_index: 30 }, // 136
    WheelInit { next_mult_factor: 0, wheel_index: 30 }, // 137
    WheelInit { next_mult_factor: 1, wheel_index: 31 }, // 138
    WheelInit { next_mult_factor: 0, wheel_index: 31 }, // 139
    WheelInit { next_mult_factor: 3, wheel_index: 32 }, // 140
    WheelInit { next_mult_factor: 2, wheel_index: 32 }, // 141
    WheelInit { next_mult_factor: 1, wheel_index: 32 }, // 142
    WheelInit { next_mult_factor: 0, wheel_index: 32 }, // 143
    WheelInit { next_mult_factor: 5, wheel_index: 33 }, // 144
    WheelInit { next_mult_factor: 4, wheel_index: 33 }, // 145
    WheelInit { next_mult_factor: 3, wheel_index: 33 }, // 146
    WheelInit { next_mult_factor: 2, wheel_index: 33 }, // 147
    WheelInit { next_mult_factor: 1, wheel_index: 33 }, // 148
    WheelInit { next_mult_factor: 0, wheel_index: 33 }, // 149
    WheelInit { next_mult_factor: 1, wheel_index: 34 }, // 150
    WheelInit { next_mult_factor: 0, wheel_index: 34 }, // 151
    WheelInit { next_mult_factor: 5, wheel_index: 35 }, // 152
    WheelInit { next_mult_factor: 4, wheel_index: 35 }, // 153
    WheelInit { next_mult_factor: 3, wheel_index: 35 }, // 154
    WheelInit { next_mult_factor: 2, wheel_index: 35 }, // 155
    WheelInit { next_mult_factor: 1, wheel_index: 35 }, // 156
    WheelInit { next_mult_factor: 0, wheel_index: 35 }, // 157
    WheelInit { next_mult_factor: 5, wheel_index: 36 }, // 158
    WheelInit { next_mult_factor: 4, wheel_index: 36 }, // 159
    WheelInit { next_mult_factor: 3, wheel_index: 36 }, // 160
    WheelInit { next_mult_factor: 2, wheel_index: 36 }, // 161
    WheelInit { next_mult_factor: 1, wheel_index: 36 }, // 162
    WheelInit { next_mult_factor: 0, wheel_index: 36 }, // 163
    WheelInit { next_mult_factor: 3, wheel_index: 37 }, // 164
    WheelInit { next_mult_factor: 2, wheel_index: 37 }, // 165
    WheelInit { next_mult_factor: 1, wheel_index: 37 }, // 166
    WheelInit { next_mult_factor: 0, wheel_index: 37 }, // 167
    WheelInit { next_mult_factor: 1, wheel_index: 38 }, // 168
    WheelInit { next_mult_factor: 0, wheel_index: 38 }, // 169
    WheelInit { next_mult_factor: 3, wheel_index: 39 }, // 170
    WheelInit { next_mult_factor: 2, wheel_index: 39 }, // 171
    WheelInit { next_mult_factor: 1, wheel_index: 39 }, // 172
    WheelInit { next_mult_factor: 0, wheel_index: 39 }, // 173
    WheelInit { next_mult_factor: 5, wheel_index: 40 }, // 174
    WheelInit { next_mult_factor: 4, wheel_index: 40 }, // 175
    WheelInit { next_mult_factor: 3, wheel_index: 40 }, // 176
    WheelInit { next_mult_factor: 2, wheel_index: 40 }, // 177
    WheelInit { next_mult_factor: 1, wheel_index: 40 }, // 178
    WheelInit { next_mult_factor: 0, wheel_index: 40 }, // 179
    WheelInit { next_mult_factor: 1, wheel_index: 41 }, // 180
    WheelInit { next_mult_factor: 0, wheel_index: 41 }, // 181
    WheelInit { next_mult_factor: 5, wheel_index: 42 }, // 182
    WheelInit { next_mult_factor: 4, wheel_index: 42 }, // 183
    WheelInit { next_mult_factor: 3, wheel_index: 42 }, // 184
    WheelInit { next_mult_factor: 2, wheel_index: 42 }, // 185
    WheelInit { next_mult_factor: 1, wheel_index: 42 }, // 186
    WheelInit { next_mult_factor: 0, wheel_index: 42 }, // 187
    WheelInit { next_mult_factor: 3, wheel_index: 43 }, // 188
    WheelInit { next_mult_factor: 2, wheel_index: 43 }, // 189
    WheelInit { next_mult_factor: 1, wheel_index: 43 }, // 190
    WheelInit { next_mult_factor: 0, wheel_index: 43 }, // 191
    WheelInit { next_mult_factor: 1, wheel_index: 44 }, // 192
    WheelInit { next_mult_factor: 0, wheel_index: 44 }, // 193
    WheelInit { next_mult_factor: 3, wheel_index: 45 }, // 194
    WheelInit { next_mult_factor: 2, wheel_index: 45 }, // 195
    WheelInit { next_mult_factor: 1, wheel_index: 45 }, // 196
    WheelInit { next_mult_factor: 0, wheel_index: 45 }, // 197
    WheelInit { next_mult_factor: 1, wheel_index: 46 }, // 198
    WheelInit { next_mult_factor: 0, wheel_index: 46 }, // 199
    WheelInit { next_mult_factor: 9, wheel_index: 47 }, // 200
    WheelInit { next_mult_factor: 8, wheel_index: 47 }, // 201
    WheelInit { next_mult_factor: 7, wheel_index: 47 }, // 202
    WheelInit { next_mult_factor: 6, wheel_index: 47 }, // 203
    WheelInit { next_mult_factor: 5, wheel_index: 47 }, // 204
    WheelInit { next_mult_factor: 4, wheel_index: 47 }, // 205
    WheelInit { next_mult_factor: 3, wheel_index: 47 }, // 206
    WheelInit { next_mult_factor: 2, wheel_index: 47 }, // 207
    WheelInit { next_mult_factor: 1, wheel_index: 47 }, // 208
    WheelInit { next_mult_factor: 0, wheel_index: 47 }, // 209
];
const WHEEL: &'static [WheelElem; 384] = &[
    // remainder 1
    WheelElem { unset_bit: 254, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 1, next: -47 },
    // remainder 7
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 1, next: -47 },
    // remainder 11
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 2, next: -47 },
    // remainder 13
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: -47 },
    // remainder 17
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 2, next: -47 },
    // remainder 19
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 2, next: -47 },
    // remainder 23
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 3, next: -47 },
    // remainder 29
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 6, next: -47 },
];
pub unsafe fn hardcoded_sieve(bytes: &mut [u8], si_: &mut usize, wi_: &mut usize, prime: usize) {
    let bytes = bytes;
    let start = bytes.as_mut_ptr();
    let len = bytes.len() as isize;
    let largest_step = (178 * prime + 168) as isize;
    let loop_len = len - largest_step;
    let loop_end = start.offset(loop_len);
    let end = start.offset(len);
    let si = *si_ as isize;
    let mut p = start.offset(si);
    let mut wi = *wi_;
    let prime_ = prime as isize;

    'outer: loop {
    match wi {
        0...47 => { // 30 * x + 1
            loop {
             'label47: loop {
              'label46: loop {
               'label45: loop {
                'label44: loop {
                 'label43: loop {
                  'label42: loop {
                   'label41: loop {
                    'label40: loop {
                     'label39: loop {
                      'label38: loop {
                       'label37: loop {
                        'label36: loop {
                         'label35: loop {
                          'label34: loop {
                           'label33: loop {
                            'label32: loop {
                             'label31: loop {
                              'label30: loop {
                               'label29: loop {
                                'label28: loop {
                                 'label27: loop {
                                  'label26: loop {
                                   'label25: loop {
                                    'label24: loop {
                                     'label23: loop {
                                      'label22: loop {
                                       'label21: loop {
                                        'label20: loop {
                                         'label19: loop {
                                          'label18: loop {
                                           'label17: loop {
                                            'label16: loop {
                                             'label15: loop {
                                              'label14: loop {
                                               'label13: loop {
                                                'label12: loop {
                                                 'label11: loop {
                                                  'label10: loop {
                                                   'label9: loop {
                                                    'label8: loop {
                                                     'label7: loop {
                                                      'label6: loop {
                                                       'label5: loop {
                                                        'label4: loop {
                                                         'label3: loop {
                                                          'label2: loop {
                                                           'label1: loop {
                                                            'label0: loop {
                                                             match wi {
                                                              0 => break 'label0,
                                                              1 => break 'label1,
                                                              2 => break 'label2,
                                                              3 => break 'label3,
                                                              4 => break 'label4,
                                                              5 => break 'label5,
                                                              6 => break 'label6,
                                                              7 => break 'label7,
                                                              8 => break 'label8,
                                                              9 => break 'label9,
                                                              10 => break 'label10,
                                                              11 => break 'label11,
                                                              12 => break 'label12,
                                                              13 => break 'label13,
                                                              14 => break 'label14,
                                                              15 => break 'label15,
                                                              16 => break 'label16,
                                                              17 => break 'label17,
                                                              18 => break 'label18,
                                                              19 => break 'label19,
                                                              20 => break 'label20,
                                                              21 => break 'label21,
                                                              22 => break 'label22,
                                                              23 => break 'label23,
                                                              24 => break 'label24,
                                                              25 => break 'label25,
                                                              26 => break 'label26,
                                                              27 => break 'label27,
                                                              28 => break 'label28,
                                                              29 => break 'label29,
                                                              30 => break 'label30,
                                                              31 => break 'label31,
                                                              32 => break 'label32,
                                                              33 => break 'label33,
                                                              34 => break 'label34,
                                                              35 => break 'label35,
                                                              36 => break 'label36,
                                                              37 => break 'label37,
                                                              38 => break 'label38,
                                                              39 => break 'label39,
                                                              40 => break 'label40,
                                                              41 => break 'label41,
                                                              42 => break 'label42,
                                                              43 => break 'label43,
                                                              44 => break 'label44,
                                                              45 => break 'label45,
                                                              46 => break 'label46,
                                                              _ => break 'label47,
                                                             }
                                                            }
                                                            while p < loop_end {
                                                                p = ::b(p);
                                                                *p.offset(prime_ * 0 + 0) &= 254;
                                                                *p.offset(prime_ * 6 + 0) &= 253;
                                                                *p.offset(prime_ * 10 + 0) &= 251;
                                                                *p.offset(prime_ * 12 + 0) &= 247;
                                                                *p.offset(prime_ * 16 + 0) &= 239;
                                                                *p.offset(prime_ * 18 + 0) &= 223;
                                                                *p.offset(prime_ * 22 + 0) &= 191;
                                                                *p.offset(prime_ * 28 + 0) &= 127;
                                                                *p.offset(prime_ * 30 + 1) &= 254;
                                                                *p.offset(prime_ * 36 + 1) &= 253;
                                                                *p.offset(prime_ * 40 + 1) &= 251;
                                                                *p.offset(prime_ * 42 + 1) &= 247;
                                                                *p.offset(prime_ * 46 + 1) &= 239;
                                                                *p.offset(prime_ * 48 + 1) &= 223;
                                                                *p.offset(prime_ * 52 + 1) &= 191;
                                                                *p.offset(prime_ * 58 + 1) &= 127;
                                                                *p.offset(prime_ * 60 + 2) &= 254;
                                                                *p.offset(prime_ * 66 + 2) &= 253;
                                                                *p.offset(prime_ * 70 + 2) &= 251;
                                                                *p.offset(prime_ * 72 + 2) &= 247;
                                                                *p.offset(prime_ * 76 + 2) &= 239;
                                                                *p.offset(prime_ * 78 + 2) &= 223;
                                                                *p.offset(prime_ * 82 + 2) &= 191;
                                                                *p.offset(prime_ * 88 + 2) &= 127;
                                                                *p.offset(prime_ * 90 + 3) &= 254;
                                                                *p.offset(prime_ * 96 + 3) &= 253;
                                                                *p.offset(prime_ * 100 + 3) &= 251;
                                                                *p.offset(prime_ * 102 + 3) &= 247;
                                                                *p.offset(prime_ * 106 + 3) &= 239;
                                                                *p.offset(prime_ * 108 + 3) &= 223;
                                                                *p.offset(prime_ * 112 + 3) &= 191;
                                                                *p.offset(prime_ * 118 + 3) &= 127;
                                                                *p.offset(prime_ * 120 + 4) &= 254;
                                                                *p.offset(prime_ * 126 + 4) &= 253;
                                                                *p.offset(prime_ * 130 + 4) &= 251;
                                                                *p.offset(prime_ * 132 + 4) &= 247;
                                                                *p.offset(prime_ * 136 + 4) &= 239;
                                                                *p.offset(prime_ * 138 + 4) &= 223;
                                                                *p.offset(prime_ * 142 + 4) &= 191;
                                                                *p.offset(prime_ * 148 + 4) &= 127;
                                                                *p.offset(prime_ * 150 + 5) &= 254;
                                                                *p.offset(prime_ * 156 + 5) &= 253;
                                                                *p.offset(prime_ * 160 + 5) &= 251;
                                                                *p.offset(prime_ * 162 + 5) &= 247;
                                                                *p.offset(prime_ * 166 + 5) &= 239;
                                                                *p.offset(prime_ * 168 + 5) &= 223;
                                                                *p.offset(prime_ * 172 + 5) &= 191;
                                                                *p.offset(prime_ * 178 + 5) &= 127;

                                                                p = p.offset(prime_ * 180 + 6)
                                                            }
                                                            if p >= end { wi = 0; break 'outer; }
                                                            *p &= 254; p = p.offset(prime_ * 6 + 0);
                                                            break 'label1
                                                           }
                                                           if p >= end { wi = 1; break 'outer; }
                                                           *p &= 253; p = p.offset(prime_ * 4 + 0);
                                                           break 'label2
                                                          }
                                                          if p >= end { wi = 2; break 'outer; }
                                                          *p &= 251; p = p.offset(prime_ * 2 + 0);
                                                          break 'label3
                                                         }
                                                         if p >= end { wi = 3; break 'outer; }
                                                         *p &= 247; p = p.offset(prime_ * 4 + 0);
                                                         break 'label4
                                                        }
                                                        if p >= end { wi = 4; break 'outer; }
                                                        *p &= 239; p = p.offset(prime_ * 2 + 0);
                                                        break 'label5
                                                       }
                                                       if p >= end { wi = 5; break 'outer; }
                                                       *p &= 223; p = p.offset(prime_ * 4 + 0);
                                                       break 'label6
                                                      }
                                                      if p >= end { wi = 6; break 'outer; }
                                                      *p &= 191; p = p.offset(prime_ * 6 + 0);
                                                      break 'label7
                                                     }
                                                     if p >= end { wi = 7; break 'outer; }
                                                     *p &= 127; p = p.offset(prime_ * 2 + 1);
                                                     break 'label8
                                                    }
                                                    if p >= end { wi = 8; break 'outer; }
                                                    *p &= 254; p = p.offset(prime_ * 6 + 0);
                                                    break 'label9
                                                   }
                                                   if p >= end { wi = 9; break 'outer; }
                                                   *p &= 253; p = p.offset(prime_ * 4 + 0);
                                                   break 'label10
                                                  }
                                                  if p >= end { wi = 10; break 'outer; }
                                                  *p &= 251; p = p.offset(prime_ * 2 + 0);
                                                  break 'label11
                                                 }
                                                 if p >= end { wi = 11; break 'outer; }
                                                 *p &= 247; p = p.offset(prime_ * 4 + 0);
                                                 break 'label12
                                                }
                                                if p >= end { wi = 12; break 'outer; }
                                                *p &= 239; p = p.offset(prime_ * 2 + 0);
                                                break 'label13
                                               }
                                               if p >= end { wi = 13; break 'outer; }
                                               *p &= 223; p = p.offset(prime_ * 4 + 0);
                                               break 'label14
                                              }
                                              if p >= end { wi = 14; break 'outer; }
                                              *p &= 191; p = p.offset(prime_ * 6 + 0);
                                              break 'label15
                                             }
                                             if p >= end { wi = 15; break 'outer; }
                                             *p &= 127; p = p.offset(prime_ * 2 + 1);
                                             break 'label16
                                            }
                                            if p >= end { wi = 16; break 'outer; }
                                            *p &= 254; p = p.offset(prime_ * 6 + 0);
                                            break 'label17
                                           }
                                           if p >= end { wi = 17; break 'outer; }
                                           *p &= 253; p = p.offset(prime_ * 4 + 0);
                                           break 'label18
                                          }
                                          if p >= end { wi = 18; break 'outer; }
                                          *p &= 251; p = p.offset(prime_ * 2 + 0);
                                          break 'label19
                                         }
                                         if p >= end { wi = 19; break 'outer; }
                                         *p &= 247; p = p.offset(prime_ * 4 + 0);
                                         break 'label20
                                        }
                                        if p >= end { wi = 20; break 'outer; }
                                        *p &= 239; p = p.offset(prime_ * 2 + 0);
                                        break 'label21
                                       }
                                       if p >= end { wi = 21; break 'outer; }
                                       *p &= 223; p = p.offset(prime_ * 4 + 0);
                                       break 'label22
                                      }
                                      if p >= end { wi = 22; break 'outer; }
                                      *p &= 191; p = p.offset(prime_ * 6 + 0);
                                      break 'label23
                                     }
                                     if p >= end { wi = 23; break 'outer; }
                                     *p &= 127; p = p.offset(prime_ * 2 + 1);
                                     break 'label24
                                    }
                                    if p >= end { wi = 24; break 'outer; }
                                    *p &= 254; p = p.offset(prime_ * 6 + 0);
                                    break 'label25
                                   }
                                   if p >= end { wi = 25; break 'outer; }
                                   *p &= 253; p = p.offset(prime_ * 4 + 0);
                                   break 'label26
                                  }
                                  if p >= end { wi = 26; break 'outer; }
                                  *p &= 251; p = p.offset(prime_ * 2 + 0);
                                  break 'label27
                                 }
                                 if p >= end { wi = 27; break 'outer; }
                                 *p &= 247; p = p.offset(prime_ * 4 + 0);
                                 break 'label28
                                }
                                if p >= end { wi = 28; break 'outer; }
                                *p &= 239; p = p.offset(prime_ * 2 + 0);
                                break 'label29
                               }
                               if p >= end { wi = 29; break 'outer; }
                               *p &= 223; p = p.offset(prime_ * 4 + 0);
                               break 'label30
                              }
                              if p >= end { wi = 30; break 'outer; }
                              *p &= 191; p = p.offset(prime_ * 6 + 0);
                              break 'label31
                             }
                             if p >= end { wi = 31; break 'outer; }
                             *p &= 127; p = p.offset(prime_ * 2 + 1);
                             break 'label32
                            }
                            if p >= end { wi = 32; break 'outer; }
                            *p &= 254; p = p.offset(prime_ * 6 + 0);
                            break 'label33
                           }
                           if p >= end { wi = 33; break 'outer; }
                           *p &= 253; p = p.offset(prime_ * 4 + 0);
                           break 'label34
                          }
                          if p >= end { wi = 34; break 'outer; }
                          *p &= 251; p = p.offset(prime_ * 2 + 0);
                          break 'label35
                         }
                         if p >= end { wi = 35; break 'outer; }
                         *p &= 247; p = p.offset(prime_ * 4 + 0);
                         break 'label36
                        }
                        if p >= end { wi = 36; break 'outer; }
                        *p &= 239; p = p.offset(prime_ * 2 + 0);
                        break 'label37
                       }
                       if p >= end { wi = 37; break 'outer; }
                       *p &= 223; p = p.offset(prime_ * 4 + 0);
                       break 'label38
                      }
                      if p >= end { wi = 38; break 'outer; }
                      *p &= 191; p = p.offset(prime_ * 6 + 0);
                      break 'label39
                     }
                     if p >= end { wi = 39; break 'outer; }
                     *p &= 127; p = p.offset(prime_ * 2 + 1);
                     break 'label40
                    }
                    if p >= end { wi = 40; break 'outer; }
                    *p &= 254; p = p.offset(prime_ * 6 + 0);
                    break 'label41
                   }
                   if p >= end { wi = 41; break 'outer; }
                   *p &= 253; p = p.offset(prime_ * 4 + 0);
                   break 'label42
                  }
                  if p >= end { wi = 42; break 'outer; }
                  *p &= 251; p = p.offset(prime_ * 2 + 0);
                  break 'label43
                 }
                 if p >= end { wi = 43; break 'outer; }
                 *p &= 247; p = p.offset(prime_ * 4 + 0);
                 break 'label44
                }
                if p >= end { wi = 44; break 'outer; }
                *p &= 239; p = p.offset(prime_ * 2 + 0);
                break 'label45
               }
               if p >= end { wi = 45; break 'outer; }
               *p &= 223; p = p.offset(prime_ * 4 + 0);
               break 'label46
              }
              if p >= end { wi = 46; break 'outer; }
              *p &= 191; p = p.offset(prime_ * 6 + 0);
              break 'label47
             }
             if p >= end { wi = 47; break 'outer; }
             *p &= 127; p = p.offset(prime_ * 2 + 1);
             wi = 0
            }
        }
        48...95 => { // 30 * x + 7
            loop {
             'label95: loop {
              'label94: loop {
               'label93: loop {
                'label92: loop {
                 'label91: loop {
                  'label90: loop {
                   'label89: loop {
                    'label88: loop {
                     'label87: loop {
                      'label86: loop {
                       'label85: loop {
                        'label84: loop {
                         'label83: loop {
                          'label82: loop {
                           'label81: loop {
                            'label80: loop {
                             'label79: loop {
                              'label78: loop {
                               'label77: loop {
                                'label76: loop {
                                 'label75: loop {
                                  'label74: loop {
                                   'label73: loop {
                                    'label72: loop {
                                     'label71: loop {
                                      'label70: loop {
                                       'label69: loop {
                                        'label68: loop {
                                         'label67: loop {
                                          'label66: loop {
                                           'label65: loop {
                                            'label64: loop {
                                             'label63: loop {
                                              'label62: loop {
                                               'label61: loop {
                                                'label60: loop {
                                                 'label59: loop {
                                                  'label58: loop {
                                                   'label57: loop {
                                                    'label56: loop {
                                                     'label55: loop {
                                                      'label54: loop {
                                                       'label53: loop {
                                                        'label52: loop {
                                                         'label51: loop {
                                                          'label50: loop {
                                                           'label49: loop {
                                                            'label48: loop {
                                                             match wi {
                                                              48 => break 'label48,
                                                              49 => break 'label49,
                                                              50 => break 'label50,
                                                              51 => break 'label51,
                                                              52 => break 'label52,
                                                              53 => break 'label53,
                                                              54 => break 'label54,
                                                              55 => break 'label55,
                                                              56 => break 'label56,
                                                              57 => break 'label57,
                                                              58 => break 'label58,
                                                              59 => break 'label59,
                                                              60 => break 'label60,
                                                              61 => break 'label61,
                                                              62 => break 'label62,
                                                              63 => break 'label63,
                                                              64 => break 'label64,
                                                              65 => break 'label65,
                                                              66 => break 'label66,
                                                              67 => break 'label67,
                                                              68 => break 'label68,
                                                              69 => break 'label69,
                                                              70 => break 'label70,
                                                              71 => break 'label71,
                                                              72 => break 'label72,
                                                              73 => break 'label73,
                                                              74 => break 'label74,
                                                              75 => break 'label75,
                                                              76 => break 'label76,
                                                              77 => break 'label77,
                                                              78 => break 'label78,
                                                              79 => break 'label79,
                                                              80 => break 'label80,
                                                              81 => break 'label81,
                                                              82 => break 'label82,
                                                              83 => break 'label83,
                                                              84 => break 'label84,
                                                              85 => break 'label85,
                                                              86 => break 'label86,
                                                              87 => break 'label87,
                                                              88 => break 'label88,
                                                              89 => break 'label89,
                                                              90 => break 'label90,
                                                              91 => break 'label91,
                                                              92 => break 'label92,
                                                              93 => break 'label93,
                                                              94 => break 'label94,
                                                              _ => break 'label95,
                                                             }
                                                            }
                                                            while p < loop_end {
                                                                p = ::b(p);
                                                                *p.offset(prime_ * 0 + 0) &= 223;
                                                                *p.offset(prime_ * 4 + 1) &= 239;
                                                                *p.offset(prime_ * 6 + 2) &= 254;
                                                                *p.offset(prime_ * 10 + 2) &= 127;
                                                                *p.offset(prime_ * 12 + 3) &= 247;
                                                                *p.offset(prime_ * 16 + 4) &= 251;
                                                                *p.offset(prime_ * 22 + 5) &= 191;
                                                                *p.offset(prime_ * 24 + 6) &= 253;
                                                                *p.offset(prime_ * 30 + 7) &= 223;
                                                                *p.offset(prime_ * 34 + 8) &= 239;
                                                                *p.offset(prime_ * 36 + 9) &= 254;
                                                                *p.offset(prime_ * 40 + 9) &= 127;
                                                                *p.offset(prime_ * 42 + 10) &= 247;
                                                                *p.offset(prime_ * 46 + 11) &= 251;
                                                                *p.offset(prime_ * 52 + 12) &= 191;
                                                                *p.offset(prime_ * 54 + 13) &= 253;
                                                                *p.offset(prime_ * 60 + 14) &= 223;
                                                                *p.offset(prime_ * 64 + 15) &= 239;
                                                                *p.offset(prime_ * 66 + 16) &= 254;
                                                                *p.offset(prime_ * 70 + 16) &= 127;
                                                                *p.offset(prime_ * 72 + 17) &= 247;
                                                                *p.offset(prime_ * 76 + 18) &= 251;
                                                                *p.offset(prime_ * 82 + 19) &= 191;
                                                                *p.offset(prime_ * 84 + 20) &= 253;
                                                                *p.offset(prime_ * 90 + 21) &= 223;
                                                                *p.offset(prime_ * 94 + 22) &= 239;
                                                                *p.offset(prime_ * 96 + 23) &= 254;
                                                                *p.offset(prime_ * 100 + 23) &= 127;
                                                                *p.offset(prime_ * 102 + 24) &= 247;
                                                                *p.offset(prime_ * 106 + 25) &= 251;
                                                                *p.offset(prime_ * 112 + 26) &= 191;
                                                                *p.offset(prime_ * 114 + 27) &= 253;
                                                                *p.offset(prime_ * 120 + 28) &= 223;
                                                                *p.offset(prime_ * 124 + 29) &= 239;
                                                                *p.offset(prime_ * 126 + 30) &= 254;
                                                                *p.offset(prime_ * 130 + 30) &= 127;
                                                                *p.offset(prime_ * 132 + 31) &= 247;
                                                                *p.offset(prime_ * 136 + 32) &= 251;
                                                                *p.offset(prime_ * 142 + 33) &= 191;
                                                                *p.offset(prime_ * 144 + 34) &= 253;
                                                                *p.offset(prime_ * 150 + 35) &= 223;
                                                                *p.offset(prime_ * 154 + 36) &= 239;
                                                                *p.offset(prime_ * 156 + 37) &= 254;
                                                                *p.offset(prime_ * 160 + 37) &= 127;
                                                                *p.offset(prime_ * 162 + 38) &= 247;
                                                                *p.offset(prime_ * 166 + 39) &= 251;
                                                                *p.offset(prime_ * 172 + 40) &= 191;
                                                                *p.offset(prime_ * 174 + 41) &= 253;

                                                                p = p.offset(prime_ * 180 + 42)
                                                            }
                                                            if p >= end { wi = 48; break 'outer; }
                                                            *p &= 223; p = p.offset(prime_ * 4 + 1);
                                                            break 'label49
                                                           }
                                                           if p >= end { wi = 49; break 'outer; }
                                                           *p &= 239; p = p.offset(prime_ * 2 + 1);
                                                           break 'label50
                                                          }
                                                          if p >= end { wi = 50; break 'outer; }
                                                          *p &= 254; p = p.offset(prime_ * 4 + 0);
                                                          break 'label51
                                                         }
                                                         if p >= end { wi = 51; break 'outer; }
                                                         *p &= 127; p = p.offset(prime_ * 2 + 1);
                                                         break 'label52
                                                        }
                                                        if p >= end { wi = 52; break 'outer; }
                                                        *p &= 247; p = p.offset(prime_ * 4 + 1);
                                                        break 'label53
                                                       }
                                                       if p >= end { wi = 53; break 'outer; }
                                                       *p &= 251; p = p.offset(prime_ * 6 + 1);
                                                       break 'label54
                                                      }
                                                      if p >= end { wi = 54; break 'outer; }
                                                      *p &= 191; p = p.offset(prime_ * 2 + 1);
                                                      break 'label55
                                                     }
                                                     if p >= end { wi = 55; break 'outer; }
                                                     *p &= 253; p = p.offset(prime_ * 6 + 1);
                                                     break 'label56
                                                    }
                                                    if p >= end { wi = 56; break 'outer; }
                                                    *p &= 223; p = p.offset(prime_ * 4 + 1);
                                                    break 'label57
                                                   }
                                                   if p >= end { wi = 57; break 'outer; }
                                                   *p &= 239; p = p.offset(prime_ * 2 + 1);
                                                   break 'label58
                                                  }
                                                  if p >= end { wi = 58; break 'outer; }
                                                  *p &= 254; p = p.offset(prime_ * 4 + 0);
                                                  break 'label59
                                                 }
                                                 if p >= end { wi = 59; break 'outer; }
                                                 *p &= 127; p = p.offset(prime_ * 2 + 1);
                                                 break 'label60
                                                }
                                                if p >= end { wi = 60; break 'outer; }
                                                *p &= 247; p = p.offset(prime_ * 4 + 1);
                                                break 'label61
                                               }
                                               if p >= end { wi = 61; break 'outer; }
                                               *p &= 251; p = p.offset(prime_ * 6 + 1);
                                               break 'label62
                                              }
                                              if p >= end { wi = 62; break 'outer; }
                                              *p &= 191; p = p.offset(prime_ * 2 + 1);
                                              break 'label63
                                             }
                                             if p >= end { wi = 63; break 'outer; }
                                             *p &= 253; p = p.offset(prime_ * 6 + 1);
                                             break 'label64
                                            }
                                            if p >= end { wi = 64; break 'outer; }
                                            *p &= 223; p = p.offset(prime_ * 4 + 1);
                                            break 'label65
                                           }
                                           if p >= end { wi = 65; break 'outer; }
                                           *p &= 239; p = p.offset(prime_ * 2 + 1);
                                           break 'label66
                                          }
                                          if p >= end { wi = 66; break 'outer; }
                                          *p &= 254; p = p.offset(prime_ * 4 + 0);
                                          break 'label67
                                         }
                                         if p >= end { wi = 67; break 'outer; }
                                         *p &= 127; p = p.offset(prime_ * 2 + 1);
                                         break 'label68
                                        }
                                        if p >= end { wi = 68; break 'outer; }
                                        *p &= 247; p = p.offset(prime_ * 4 + 1);
                                        break 'label69
                                       }
                                       if p >= end { wi = 69; break 'outer; }
                                       *p &= 251; p = p.offset(prime_ * 6 + 1);
                                       break 'label70
                                      }
                                      if p >= end { wi = 70; break 'outer; }
                                      *p &= 191; p = p.offset(prime_ * 2 + 1);
                                      break 'label71
                                     }
                                     if p >= end { wi = 71; break 'outer; }
                                     *p &= 253; p = p.offset(prime_ * 6 + 1);
                                     break 'label72
                                    }
                                    if p >= end { wi = 72; break 'outer; }
                                    *p &= 223; p = p.offset(prime_ * 4 + 1);
                                    break 'label73
                                   }
                                   if p >= end { wi = 73; break 'outer; }
                                   *p &= 239; p = p.offset(prime_ * 2 + 1);
                                   break 'label74
                                  }
                                  if p >= end { wi = 74; break 'outer; }
                                  *p &= 254; p = p.offset(prime_ * 4 + 0);
                                  break 'label75
                                 }
                                 if p >= end { wi = 75; break 'outer; }
                                 *p &= 127; p = p.offset(prime_ * 2 + 1);
                                 break 'label76
                                }
                                if p >= end { wi = 76; break 'outer; }
                                *p &= 247; p = p.offset(prime_ * 4 + 1);
                                break 'label77
                               }
                               if p >= end { wi = 77; break 'outer; }
                               *p &= 251; p = p.offset(prime_ * 6 + 1);
                               break 'label78
                              }
                              if p >= end { wi = 78; break 'outer; }
                              *p &= 191; p = p.offset(prime_ * 2 + 1);
                              break 'label79
                             }
                             if p >= end { wi = 79; break 'outer; }
                             *p &= 253; p = p.offset(prime_ * 6 + 1);
                             break 'label80
                            }
                            if p >= end { wi = 80; break 'outer; }
                            *p &= 223; p = p.offset(prime_ * 4 + 1);
                            break 'label81
                           }
                           if p >= end { wi = 81; break 'outer; }
                           *p &= 239; p = p.offset(prime_ * 2 + 1);
                           break 'label82
                          }
                          if p >= end { wi = 82; break 'outer; }
                          *p &= 254; p = p.offset(prime_ * 4 + 0);
                          break 'label83
                         }
                         if p >= end { wi = 83; break 'outer; }
                         *p &= 127; p = p.offset(prime_ * 2 + 1);
                         break 'label84
                        }
                        if p >= end { wi = 84; break 'outer; }
                        *p &= 247; p = p.offset(prime_ * 4 + 1);
                        break 'label85
                       }
                       if p >= end { wi = 85; break 'outer; }
                       *p &= 251; p = p.offset(prime_ * 6 + 1);
                       break 'label86
                      }
                      if p >= end { wi = 86; break 'outer; }
                      *p &= 191; p = p.offset(prime_ * 2 + 1);
                      break 'label87
                     }
                     if p >= end { wi = 87; break 'outer; }
                     *p &= 253; p = p.offset(prime_ * 6 + 1);
                     break 'label88
                    }
                    if p >= end { wi = 88; break 'outer; }
                    *p &= 223; p = p.offset(prime_ * 4 + 1);
                    break 'label89
                   }
                   if p >= end { wi = 89; break 'outer; }
                   *p &= 239; p = p.offset(prime_ * 2 + 1);
                   break 'label90
                  }
                  if p >= end { wi = 90; break 'outer; }
                  *p &= 254; p = p.offset(prime_ * 4 + 0);
                  break 'label91
                 }
                 if p >= end { wi = 91; break 'outer; }
                 *p &= 127; p = p.offset(prime_ * 2 + 1);
                 break 'label92
                }
                if p >= end { wi = 92; break 'outer; }
                *p &= 247; p = p.offset(prime_ * 4 + 1);
                break 'label93
               }
               if p >= end { wi = 93; break 'outer; }
               *p &= 251; p = p.offset(prime_ * 6 + 1);
               break 'label94
              }
              if p >= end { wi = 94; break 'outer; }
              *p &= 191; p = p.offset(prime_ * 2 + 1);
              break 'label95
             }
             if p >= end { wi = 95; break 'outer; }
             *p &= 253; p = p.offset(prime_ * 6 + 1);
             wi = 48
            }
        }
        96...143 => { // 30 * x + 11
            loop {
             'label143: loop {
              'label142: loop {
               'label141: loop {
                'label140: loop {
                 'label139: loop {
                  'label138: loop {
                   'label137: loop {
                    'label136: loop {
                     'label135: loop {
                      'label134: loop {
                       'label133: loop {
                        'label132: loop {
                         'label131: loop {
                          'label130: loop {
                           'label129: loop {
                            'label128: loop {
                             'label127: loop {
                              'label126: loop {
                               'label125: loop {
                                'label124: loop {
                                 'label123: loop {
                                  'label122: loop {
                                   'label121: loop {
                                    'label120: loop {
                                     'label119: loop {
                                      'label118: loop {
                                       'label117: loop {
                                        'label116: loop {
                                         'label115: loop {
                                          'label114: loop {
                                           'label113: loop {
                                            'label112: loop {
                                             'label111: loop {
                                              'label110: loop {
                                               'label109: loop {
                                                'label108: loop {
                                                 'label107: loop {
                                                  'label106: loop {
                                                   'label105: loop {
                                                    'label104: loop {
                                                     'label103: loop {
                                                      'label102: loop {
                                                       'label101: loop {
                                                        'label100: loop {
                                                         'label99: loop {
                                                          'label98: loop {
                                                           'label97: loop {
                                                            'label96: loop {
                                                             match wi {
                                                              96 => break 'label96,
                                                              97 => break 'label97,
                                                              98 => break 'label98,
                                                              99 => break 'label99,
                                                              100 => break 'label100,
                                                              101 => break 'label101,
                                                              102 => break 'label102,
                                                              103 => break 'label103,
                                                              104 => break 'label104,
                                                              105 => break 'label105,
                                                              106 => break 'label106,
                                                              107 => break 'label107,
                                                              108 => break 'label108,
                                                              109 => break 'label109,
                                                              110 => break 'label110,
                                                              111 => break 'label111,
                                                              112 => break 'label112,
                                                              113 => break 'label113,
                                                              114 => break 'label114,
                                                              115 => break 'label115,
                                                              116 => break 'label116,
                                                              117 => break 'label117,
                                                              118 => break 'label118,
                                                              119 => break 'label119,
                                                              120 => break 'label120,
                                                              121 => break 'label121,
                                                              122 => break 'label122,
                                                              123 => break 'label123,
                                                              124 => break 'label124,
                                                              125 => break 'label125,
                                                              126 => break 'label126,
                                                              127 => break 'label127,
                                                              128 => break 'label128,
                                                              129 => break 'label129,
                                                              130 => break 'label130,
                                                              131 => break 'label131,
                                                              132 => break 'label132,
                                                              133 => break 'label133,
                                                              134 => break 'label134,
                                                              135 => break 'label135,
                                                              136 => break 'label136,
                                                              137 => break 'label137,
                                                              138 => break 'label138,
                                                              139 => break 'label139,
                                                              140 => break 'label140,
                                                              141 => break 'label141,
                                                              142 => break 'label142,
                                                              _ => break 'label143,
                                                             }
                                                            }
                                                            while p < loop_end {
                                                                p = ::b(p);
                                                                *p.offset(prime_ * 0 + 0) &= 254;
                                                                *p.offset(prime_ * 2 + 0) &= 191;
                                                                *p.offset(prime_ * 6 + 2) &= 253;
                                                                *p.offset(prime_ * 8 + 2) &= 127;
                                                                *p.offset(prime_ * 12 + 4) &= 247;
                                                                *p.offset(prime_ * 18 + 6) &= 223;
                                                                *p.offset(prime_ * 20 + 7) &= 251;
                                                                *p.offset(prime_ * 26 + 9) &= 239;
                                                                *p.offset(prime_ * 30 + 11) &= 254;
                                                                *p.offset(prime_ * 32 + 11) &= 191;
                                                                *p.offset(prime_ * 36 + 13) &= 253;
                                                                *p.offset(prime_ * 38 + 13) &= 127;
                                                                *p.offset(prime_ * 42 + 15) &= 247;
                                                                *p.offset(prime_ * 48 + 17) &= 223;
                                                                *p.offset(prime_ * 50 + 18) &= 251;
                                                                *p.offset(prime_ * 56 + 20) &= 239;
                                                                *p.offset(prime_ * 60 + 22) &= 254;
                                                                *p.offset(prime_ * 62 + 22) &= 191;
                                                                *p.offset(prime_ * 66 + 24) &= 253;
                                                                *p.offset(prime_ * 68 + 24) &= 127;
                                                                *p.offset(prime_ * 72 + 26) &= 247;
                                                                *p.offset(prime_ * 78 + 28) &= 223;
                                                                *p.offset(prime_ * 80 + 29) &= 251;
                                                                *p.offset(prime_ * 86 + 31) &= 239;
                                                                *p.offset(prime_ * 90 + 33) &= 254;
                                                                *p.offset(prime_ * 92 + 33) &= 191;
                                                                *p.offset(prime_ * 96 + 35) &= 253;
                                                                *p.offset(prime_ * 98 + 35) &= 127;
                                                                *p.offset(prime_ * 102 + 37) &= 247;
                                                                *p.offset(prime_ * 108 + 39) &= 223;
                                                                *p.offset(prime_ * 110 + 40) &= 251;
                                                                *p.offset(prime_ * 116 + 42) &= 239;
                                                                *p.offset(prime_ * 120 + 44) &= 254;
                                                                *p.offset(prime_ * 122 + 44) &= 191;
                                                                *p.offset(prime_ * 126 + 46) &= 253;
                                                                *p.offset(prime_ * 128 + 46) &= 127;
                                                                *p.offset(prime_ * 132 + 48) &= 247;
                                                                *p.offset(prime_ * 138 + 50) &= 223;
                                                                *p.offset(prime_ * 140 + 51) &= 251;
                                                                *p.offset(prime_ * 146 + 53) &= 239;
                                                                *p.offset(prime_ * 150 + 55) &= 254;
                                                                *p.offset(prime_ * 152 + 55) &= 191;
                                                                *p.offset(prime_ * 156 + 57) &= 253;
                                                                *p.offset(prime_ * 158 + 57) &= 127;
                                                                *p.offset(prime_ * 162 + 59) &= 247;
                                                                *p.offset(prime_ * 168 + 61) &= 223;
                                                                *p.offset(prime_ * 170 + 62) &= 251;
                                                                *p.offset(prime_ * 176 + 64) &= 239;

                                                                p = p.offset(prime_ * 180 + 66)
                                                            }
                                                            if p >= end { wi = 96; break 'outer; }
                                                            *p &= 254; p = p.offset(prime_ * 2 + 0);
                                                            break 'label97
                                                           }
                                                           if p >= end { wi = 97; break 'outer; }
                                                           *p &= 191; p = p.offset(prime_ * 4 + 2);
                                                           break 'label98
                                                          }
                                                          if p >= end { wi = 98; break 'outer; }
                                                          *p &= 253; p = p.offset(prime_ * 2 + 0);
                                                          break 'label99
                                                         }
                                                         if p >= end { wi = 99; break 'outer; }
                                                         *p &= 127; p = p.offset(prime_ * 4 + 2);
                                                         break 'label100
                                                        }
                                                        if p >= end { wi = 100; break 'outer; }
                                                        *p &= 247; p = p.offset(prime_ * 6 + 2);
                                                        break 'label101
                                                       }
                                                       if p >= end { wi = 101; break 'outer; }
                                                       *p &= 223; p = p.offset(prime_ * 2 + 1);
                                                       break 'label102
                                                      }
                                                      if p >= end { wi = 102; break 'outer; }
                                                      *p &= 251; p = p.offset(prime_ * 6 + 2);
                                                      break 'label103
                                                     }
                                                     if p >= end { wi = 103; break 'outer; }
                                                     *p &= 239; p = p.offset(prime_ * 4 + 2);
                                                     break 'label104
                                                    }
                                                    if p >= end { wi = 104; break 'outer; }
                                                    *p &= 254; p = p.offset(prime_ * 2 + 0);
                                                    break 'label105
                                                   }
                                                   if p >= end { wi = 105; break 'outer; }
                                                   *p &= 191; p = p.offset(prime_ * 4 + 2);
                                                   break 'label106
                                                  }
                                                  if p >= end { wi = 106; break 'outer; }
                                                  *p &= 253; p = p.offset(prime_ * 2 + 0);
                                                  break 'label107
                                                 }
                                                 if p >= end { wi = 107; break 'outer; }
                                                 *p &= 127; p = p.offset(prime_ * 4 + 2);
                                                 break 'label108
                                                }
                                                if p >= end { wi = 108; break 'outer; }
                                                *p &= 247; p = p.offset(prime_ * 6 + 2);
                                                break 'label109
                                               }
                                               if p >= end { wi = 109; break 'outer; }
                                               *p &= 223; p = p.offset(prime_ * 2 + 1);
                                               break 'label110
                                              }
                                              if p >= end { wi = 110; break 'outer; }
                                              *p &= 251; p = p.offset(prime_ * 6 + 2);
                                              break 'label111
                                             }
                                             if p >= end { wi = 111; break 'outer; }
                                             *p &= 239; p = p.offset(prime_ * 4 + 2);
                                             break 'label112
                                            }
                                            if p >= end { wi = 112; break 'outer; }
                                            *p &= 254; p = p.offset(prime_ * 2 + 0);
                                            break 'label113
                                           }
                                           if p >= end { wi = 113; break 'outer; }
                                           *p &= 191; p = p.offset(prime_ * 4 + 2);
                                           break 'label114
                                          }
                                          if p >= end { wi = 114; break 'outer; }
                                          *p &= 253; p = p.offset(prime_ * 2 + 0);
                                          break 'label115
                                         }
                                         if p >= end { wi = 115; break 'outer; }
                                         *p &= 127; p = p.offset(prime_ * 4 + 2);
                                         break 'label116
                                        }
                                        if p >= end { wi = 116; break 'outer; }
                                        *p &= 247; p = p.offset(prime_ * 6 + 2);
                                        break 'label117
                                       }
                                       if p >= end { wi = 117; break 'outer; }
                                       *p &= 223; p = p.offset(prime_ * 2 + 1);
                                       break 'label118
                                      }
                                      if p >= end { wi = 118; break 'outer; }
                                      *p &= 251; p = p.offset(prime_ * 6 + 2);
                                      break 'label119
                                     }
                                     if p >= end { wi = 119; break 'outer; }
                                     *p &= 239; p = p.offset(prime_ * 4 + 2);
                                     break 'label120
                                    }
                                    if p >= end { wi = 120; break 'outer; }
                                    *p &= 254; p = p.offset(prime_ * 2 + 0);
                                    break 'label121
                                   }
                                   if p >= end { wi = 121; break 'outer; }
                                   *p &= 191; p = p.offset(prime_ * 4 + 2);
                                   break 'label122
                                  }
                                  if p >= end { wi = 122; break 'outer; }
                                  *p &= 253; p = p.offset(prime_ * 2 + 0);
                                  break 'label123
                                 }
                                 if p >= end { wi = 123; break 'outer; }
                                 *p &= 127; p = p.offset(prime_ * 4 + 2);
                                 break 'label124
                                }
                                if p >= end { wi = 124; break 'outer; }
                                *p &= 247; p = p.offset(prime_ * 6 + 2);
                                break 'label125
                               }
                               if p >= end { wi = 125; break 'outer; }
                               *p &= 223; p = p.offset(prime_ * 2 + 1);
                               break 'label126
                              }
                              if p >= end { wi = 126; break 'outer; }
                              *p &= 251; p = p.offset(prime_ * 6 + 2);
                              break 'label127
                             }
                             if p >= end { wi = 127; break 'outer; }
                             *p &= 239; p = p.offset(prime_ * 4 + 2);
                             break 'label128
                            }
                            if p >= end { wi = 128; break 'outer; }
                            *p &= 254; p = p.offset(prime_ * 2 + 0);
                            break 'label129
                           }
                           if p >= end { wi = 129; break 'outer; }
                           *p &= 191; p = p.offset(prime_ * 4 + 2);
                           break 'label130
                          }
                          if p >= end { wi = 130; break 'outer; }
                          *p &= 253; p = p.offset(prime_ * 2 + 0);
                          break 'label131
                         }
                         if p >= end { wi = 131; break 'outer; }
                         *p &= 127; p = p.offset(prime_ * 4 + 2);
                         break 'label132
                        }
                        if p >= end { wi = 132; break 'outer; }
                        *p &= 247; p = p.offset(prime_ * 6 + 2);
                        break 'label133
                       }
                       if p >= end { wi = 133; break 'outer; }
                       *p &= 223; p = p.offset(prime_ * 2 + 1);
                       break 'label134
                      }
                      if p >= end { wi = 134; break 'outer; }
                      *p &= 251; p = p.offset(prime_ * 6 + 2);
                      break 'label135
                     }
                     if p >= end { wi = 135; break 'outer; }
                     *p &= 239; p = p.offset(prime_ * 4 + 2);
                     break 'label136
                    }
                    if p >= end { wi = 136; break 'outer; }
                    *p &= 254; p = p.offset(prime_ * 2 + 0);
                    break 'label137
                   }
                   if p >= end { wi = 137; break 'outer; }
                   *p &= 191; p = p.offset(prime_ * 4 + 2);
                   break 'label138
                  }
                  if p >= end { wi = 138; break 'outer; }
                  *p &= 253; p = p.offset(prime_ * 2 + 0);
                  break 'label139
                 }
                 if p >= end { wi = 139; break 'outer; }
                 *p &= 127; p = p.offset(prime_ * 4 + 2);
                 break 'label140
                }
                if p >= end { wi = 140; break 'outer; }
                *p &= 247; p = p.offset(prime_ * 6 + 2);
                break 'label141
               }
               if p >= end { wi = 141; break 'outer; }
               *p &= 223; p = p.offset(prime_ * 2 + 1);
               break 'label142
              }
              if p >= end { wi = 142; break 'outer; }
              *p &= 251; p = p.offset(prime_ * 6 + 2);
              break 'label143
             }
             if p >= end { wi = 143; break 'outer; }
             *p &= 239; p = p.offset(prime_ * 4 + 2);
             wi = 96
            }
        }
        144...191 => { // 30 * x + 13
            loop {
             'label191: loop {
              'label190: loop {
               'label189: loop {
                'label188: loop {
                 'label187: loop {
                  'label186: loop {
                   'label185: loop {
                    'label184: loop {
                     'label183: loop {
                      'label182: loop {
                       'label181: loop {
                        'label180: loop {
                         'label179: loop {
                          'label178: loop {
                           'label177: loop {
                            'label176: loop {
                             'label175: loop {
                              'label174: loop {
                               'label173: loop {
                                'label172: loop {
                                 'label171: loop {
                                  'label170: loop {
                                   'label169: loop {
                                    'label168: loop {
                                     'label167: loop {
                                      'label166: loop {
                                       'label165: loop {
                                        'label164: loop {
                                         'label163: loop {
                                          'label162: loop {
                                           'label161: loop {
                                            'label160: loop {
                                             'label159: loop {
                                              'label158: loop {
                                               'label157: loop {
                                                'label156: loop {
                                                 'label155: loop {
                                                  'label154: loop {
                                                   'label153: loop {
                                                    'label152: loop {
                                                     'label151: loop {
                                                      'label150: loop {
                                                       'label149: loop {
                                                        'label148: loop {
                                                         'label147: loop {
                                                          'label146: loop {
                                                           'label145: loop {
                                                            'label144: loop {
                                                             match wi {
                                                              144 => break 'label144,
                                                              145 => break 'label145,
                                                              146 => break 'label146,
                                                              147 => break 'label147,
                                                              148 => break 'label148,
                                                              149 => break 'label149,
                                                              150 => break 'label150,
                                                              151 => break 'label151,
                                                              152 => break 'label152,
                                                              153 => break 'label153,
                                                              154 => break 'label154,
                                                              155 => break 'label155,
                                                              156 => break 'label156,
                                                              157 => break 'label157,
                                                              158 => break 'label158,
                                                              159 => break 'label159,
                                                              160 => break 'label160,
                                                              161 => break 'label161,
                                                              162 => break 'label162,
                                                              163 => break 'label163,
                                                              164 => break 'label164,
                                                              165 => break 'label165,
                                                              166 => break 'label166,
                                                              167 => break 'label167,
                                                              168 => break 'label168,
                                                              169 => break 'label169,
                                                              170 => break 'label170,
                                                              171 => break 'label171,
                                                              172 => break 'label172,
                                                              173 => break 'label173,
                                                              174 => break 'label174,
                                                              175 => break 'label175,
                                                              176 => break 'label176,
                                                              177 => break 'label177,
                                                              178 => break 'label178,
                                                              179 => break 'label179,
                                                              180 => break 'label180,
                                                              181 => break 'label181,
                                                              182 => break 'label182,
                                                              183 => break 'label183,
                                                              184 => break 'label184,
                                                              185 => break 'label185,
                                                              186 => break 'label186,
                                                              187 => break 'label187,
                                                              188 => break 'label188,
                                                              189 => break 'label189,
                                                              190 => break 'label190,
                                                              _ => break 'label191,
                                                             }
                                                            }
                                                            while p < loop_end {
                                                                p = ::b(p);
                                                                *p.offset(prime_ * 0 + 0) &= 223;
                                                                *p.offset(prime_ * 4 + 2) &= 251;
                                                                *p.offset(prime_ * 6 + 3) &= 253;
                                                                *p.offset(prime_ * 10 + 4) &= 127;
                                                                *p.offset(prime_ * 16 + 7) &= 239;
                                                                *p.offset(prime_ * 18 + 8) &= 247;
                                                                *p.offset(prime_ * 24 + 11) &= 254;
                                                                *p.offset(prime_ * 28 + 12) &= 191;
                                                                *p.offset(prime_ * 30 + 13) &= 223;
                                                                *p.offset(prime_ * 34 + 15) &= 251;
                                                                *p.offset(prime_ * 36 + 16) &= 253;
                                                                *p.offset(prime_ * 40 + 17) &= 127;
                                                                *p.offset(prime_ * 46 + 20) &= 239;
                                                                *p.offset(prime_ * 48 + 21) &= 247;
                                                                *p.offset(prime_ * 54 + 24) &= 254;
                                                                *p.offset(prime_ * 58 + 25) &= 191;
                                                                *p.offset(prime_ * 60 + 26) &= 223;
                                                                *p.offset(prime_ * 64 + 28) &= 251;
                                                                *p.offset(prime_ * 66 + 29) &= 253;
                                                                *p.offset(prime_ * 70 + 30) &= 127;
                                                                *p.offset(prime_ * 76 + 33) &= 239;
                                                                *p.offset(prime_ * 78 + 34) &= 247;
                                                                *p.offset(prime_ * 84 + 37) &= 254;
                                                                *p.offset(prime_ * 88 + 38) &= 191;
                                                                *p.offset(prime_ * 90 + 39) &= 223;
                                                                *p.offset(prime_ * 94 + 41) &= 251;
                                                                *p.offset(prime_ * 96 + 42) &= 253;
                                                                *p.offset(prime_ * 100 + 43) &= 127;
                                                                *p.offset(prime_ * 106 + 46) &= 239;
                                                                *p.offset(prime_ * 108 + 47) &= 247;
                                                                *p.offset(prime_ * 114 + 50) &= 254;
                                                                *p.offset(prime_ * 118 + 51) &= 191;
                                                                *p.offset(prime_ * 120 + 52) &= 223;
                                                                *p.offset(prime_ * 124 + 54) &= 251;
                                                                *p.offset(prime_ * 126 + 55) &= 253;
                                                                *p.offset(prime_ * 130 + 56) &= 127;
                                                                *p.offset(prime_ * 136 + 59) &= 239;
                                                                *p.offset(prime_ * 138 + 60) &= 247;
                                                                *p.offset(prime_ * 144 + 63) &= 254;
                                                                *p.offset(prime_ * 148 + 64) &= 191;
                                                                *p.offset(prime_ * 150 + 65) &= 223;
                                                                *p.offset(prime_ * 154 + 67) &= 251;
                                                                *p.offset(prime_ * 156 + 68) &= 253;
                                                                *p.offset(prime_ * 160 + 69) &= 127;
                                                                *p.offset(prime_ * 166 + 72) &= 239;
                                                                *p.offset(prime_ * 168 + 73) &= 247;
                                                                *p.offset(prime_ * 174 + 76) &= 254;
                                                                *p.offset(prime_ * 178 + 77) &= 191;

                                                                p = p.offset(prime_ * 180 + 78)
                                                            }
                                                            if p >= end { wi = 144; break 'outer; }
                                                            *p &= 223; p = p.offset(prime_ * 4 + 2);
                                                            break 'label145
                                                           }
                                                           if p >= end { wi = 145; break 'outer; }
                                                           *p &= 251; p = p.offset(prime_ * 2 + 1);
                                                           break 'label146
                                                          }
                                                          if p >= end { wi = 146; break 'outer; }
                                                          *p &= 253; p = p.offset(prime_ * 4 + 1);
                                                          break 'label147
                                                         }
                                                         if p >= end { wi = 147; break 'outer; }
                                                         *p &= 127; p = p.offset(prime_ * 6 + 3);
                                                         break 'label148
                                                        }
                                                        if p >= end { wi = 148; break 'outer; }
                                                        *p &= 239; p = p.offset(prime_ * 2 + 1);
                                                        break 'label149
                                                       }
                                                       if p >= end { wi = 149; break 'outer; }
                                                       *p &= 247; p = p.offset(prime_ * 6 + 3);
                                                       break 'label150
                                                      }
                                                      if p >= end { wi = 150; break 'outer; }
                                                      *p &= 254; p = p.offset(prime_ * 4 + 1);
                                                      break 'label151
                                                     }
                                                     if p >= end { wi = 151; break 'outer; }
                                                     *p &= 191; p = p.offset(prime_ * 2 + 1);
                                                     break 'label152
                                                    }
                                                    if p >= end { wi = 152; break 'outer; }
                                                    *p &= 223; p = p.offset(prime_ * 4 + 2);
                                                    break 'label153
                                                   }
                                                   if p >= end { wi = 153; break 'outer; }
                                                   *p &= 251; p = p.offset(prime_ * 2 + 1);
                                                   break 'label154
                                                  }
                                                  if p >= end { wi = 154; break 'outer; }
                                                  *p &= 253; p = p.offset(prime_ * 4 + 1);
                                                  break 'label155
                                                 }
                                                 if p >= end { wi = 155; break 'outer; }
                                                 *p &= 127; p = p.offset(prime_ * 6 + 3);
                                                 break 'label156
                                                }
                                                if p >= end { wi = 156; break 'outer; }
                                                *p &= 239; p = p.offset(prime_ * 2 + 1);
                                                break 'label157
                                               }
                                               if p >= end { wi = 157; break 'outer; }
                                               *p &= 247; p = p.offset(prime_ * 6 + 3);
                                               break 'label158
                                              }
                                              if p >= end { wi = 158; break 'outer; }
                                              *p &= 254; p = p.offset(prime_ * 4 + 1);
                                              break 'label159
                                             }
                                             if p >= end { wi = 159; break 'outer; }
                                             *p &= 191; p = p.offset(prime_ * 2 + 1);
                                             break 'label160
                                            }
                                            if p >= end { wi = 160; break 'outer; }
                                            *p &= 223; p = p.offset(prime_ * 4 + 2);
                                            break 'label161
                                           }
                                           if p >= end { wi = 161; break 'outer; }
                                           *p &= 251; p = p.offset(prime_ * 2 + 1);
                                           break 'label162
                                          }
                                          if p >= end { wi = 162; break 'outer; }
                                          *p &= 253; p = p.offset(prime_ * 4 + 1);
                                          break 'label163
                                         }
                                         if p >= end { wi = 163; break 'outer; }
                                         *p &= 127; p = p.offset(prime_ * 6 + 3);
                                         break 'label164
                                        }
                                        if p >= end { wi = 164; break 'outer; }
                                        *p &= 239; p = p.offset(prime_ * 2 + 1);
                                        break 'label165
                                       }
                                       if p >= end { wi = 165; break 'outer; }
                                       *p &= 247; p = p.offset(prime_ * 6 + 3);
                                       break 'label166
                                      }
                                      if p >= end { wi = 166; break 'outer; }
                                      *p &= 254; p = p.offset(prime_ * 4 + 1);
                                      break 'label167
                                     }
                                     if p >= end { wi = 167; break 'outer; }
                                     *p &= 191; p = p.offset(prime_ * 2 + 1);
                                     break 'label168
                                    }
                                    if p >= end { wi = 168; break 'outer; }
                                    *p &= 223; p = p.offset(prime_ * 4 + 2);
                                    break 'label169
                                   }
                                   if p >= end { wi = 169; break 'outer; }
                                   *p &= 251; p = p.offset(prime_ * 2 + 1);
                                   break 'label170
                                  }
                                  if p >= end { wi = 170; break 'outer; }
                                  *p &= 253; p = p.offset(prime_ * 4 + 1);
                                  break 'label171
                                 }
                                 if p >= end { wi = 171; break 'outer; }
                                 *p &= 127; p = p.offset(prime_ * 6 + 3);
                                 break 'label172
                                }
                                if p >= end { wi = 172; break 'outer; }
                                *p &= 239; p = p.offset(prime_ * 2 + 1);
                                break 'label173
                               }
                               if p >= end { wi = 173; break 'outer; }
                               *p &= 247; p = p.offset(prime_ * 6 + 3);
                               break 'label174
                              }
                              if p >= end { wi = 174; break 'outer; }
                              *p &= 254; p = p.offset(prime_ * 4 + 1);
                              break 'label175
                             }
                             if p >= end { wi = 175; break 'outer; }
                             *p &= 191; p = p.offset(prime_ * 2 + 1);
                             break 'label176
                            }
                            if p >= end { wi = 176; break 'outer; }
                            *p &= 223; p = p.offset(prime_ * 4 + 2);
                            break 'label177
                           }
                           if p >= end { wi = 177; break 'outer; }
                           *p &= 251; p = p.offset(prime_ * 2 + 1);
                           break 'label178
                          }
                          if p >= end { wi = 178; break 'outer; }
                          *p &= 253; p = p.offset(prime_ * 4 + 1);
                          break 'label179
                         }
                         if p >= end { wi = 179; break 'outer; }
                         *p &= 127; p = p.offset(prime_ * 6 + 3);
                         break 'label180
                        }
                        if p >= end { wi = 180; break 'outer; }
                        *p &= 239; p = p.offset(prime_ * 2 + 1);
                        break 'label181
                       }
                       if p >= end { wi = 181; break 'outer; }
                       *p &= 247; p = p.offset(prime_ * 6 + 3);
                       break 'label182
                      }
                      if p >= end { wi = 182; break 'outer; }
                      *p &= 254; p = p.offset(prime_ * 4 + 1);
                      break 'label183
                     }
                     if p >= end { wi = 183; break 'outer; }
                     *p &= 191; p = p.offset(prime_ * 2 + 1);
                     break 'label184
                    }
                    if p >= end { wi = 184; break 'outer; }
                    *p &= 223; p = p.offset(prime_ * 4 + 2);
                    break 'label185
                   }
                   if p >= end { wi = 185; break 'outer; }
                   *p &= 251; p = p.offset(prime_ * 2 + 1);
                   break 'label186
                  }
                  if p >= end { wi = 186; break 'outer; }
                  *p &= 253; p = p.offset(prime_ * 4 + 1);
                  break 'label187
                 }
                 if p >= end { wi = 187; break 'outer; }
                 *p &= 127; p = p.offset(prime_ * 6 + 3);
                 break 'label188
                }
                if p >= end { wi = 188; break 'outer; }
                *p &= 239; p = p.offset(prime_ * 2 + 1);
                break 'label189
               }
               if p >= end { wi = 189; break 'outer; }
               *p &= 247; p = p.offset(prime_ * 6 + 3);
               break 'label190
              }
              if p >= end { wi = 190; break 'outer; }
              *p &= 254; p = p.offset(prime_ * 4 + 1);
              break 'label191
             }
             if p >= end { wi = 191; break 'outer; }
             *p &= 191; p = p.offset(prime_ * 2 + 1);
             wi = 144
            }
        }
        192...239 => { // 30 * x + 17
            loop {
             'label239: loop {
              'label238: loop {
               'label237: loop {
                'label236: loop {
                 'label235: loop {
                  'label234: loop {
                   'label233: loop {
                    'label232: loop {
                     'label231: loop {
                      'label230: loop {
                       'label229: loop {
                        'label228: loop {
                         'label227: loop {
                          'label226: loop {
                           'label225: loop {
                            'label224: loop {
                             'label223: loop {
                              'label222: loop {
                               'label221: loop {
                                'label220: loop {
                                 'label219: loop {
                                  'label218: loop {
                                   'label217: loop {
                                    'label216: loop {
                                     'label215: loop {
                                      'label214: loop {
                                       'label213: loop {
                                        'label212: loop {
                                         'label211: loop {
                                          'label210: loop {
                                           'label209: loop {
                                            'label208: loop {
                                             'label207: loop {
                                              'label206: loop {
                                               'label205: loop {
                                                'label204: loop {
                                                 'label203: loop {
                                                  'label202: loop {
                                                   'label201: loop {
                                                    'label200: loop {
                                                     'label199: loop {
                                                      'label198: loop {
                                                       'label197: loop {
                                                        'label196: loop {
                                                         'label195: loop {
                                                          'label194: loop {
                                                           'label193: loop {
                                                            'label192: loop {
                                                             match wi {
                                                              192 => break 'label192,
                                                              193 => break 'label193,
                                                              194 => break 'label194,
                                                              195 => break 'label195,
                                                              196 => break 'label196,
                                                              197 => break 'label197,
                                                              198 => break 'label198,
                                                              199 => break 'label199,
                                                              200 => break 'label200,
                                                              201 => break 'label201,
                                                              202 => break 'label202,
                                                              203 => break 'label203,
                                                              204 => break 'label204,
                                                              205 => break 'label205,
                                                              206 => break 'label206,
                                                              207 => break 'label207,
                                                              208 => break 'label208,
                                                              209 => break 'label209,
                                                              210 => break 'label210,
                                                              211 => break 'label211,
                                                              212 => break 'label212,
                                                              213 => break 'label213,
                                                              214 => break 'label214,
                                                              215 => break 'label215,
                                                              216 => break 'label216,
                                                              217 => break 'label217,
                                                              218 => break 'label218,
                                                              219 => break 'label219,
                                                              220 => break 'label220,
                                                              221 => break 'label221,
                                                              222 => break 'label222,
                                                              223 => break 'label223,
                                                              224 => break 'label224,
                                                              225 => break 'label225,
                                                              226 => break 'label226,
                                                              227 => break 'label227,
                                                              228 => break 'label228,
                                                              229 => break 'label229,
                                                              230 => break 'label230,
                                                              231 => break 'label231,
                                                              232 => break 'label232,
                                                              233 => break 'label233,
                                                              234 => break 'label234,
                                                              235 => break 'label235,
                                                              236 => break 'label236,
                                                              237 => break 'label237,
                                                              238 => break 'label238,
                                                              _ => break 'label239,
                                                             }
                                                            }
                                                            while p < loop_end {
                                                                p = ::b(p);
                                                                *p.offset(prime_ * 0 + 0) &= 223;
                                                                *p.offset(prime_ * 2 + 1) &= 191;
                                                                *p.offset(prime_ * 6 + 4) &= 254;
                                                                *p.offset(prime_ * 12 + 7) &= 247;
                                                                *p.offset(prime_ * 14 + 8) &= 239;
                                                                *p.offset(prime_ * 20 + 11) &= 127;
                                                                *p.offset(prime_ * 24 + 14) &= 253;
                                                                *p.offset(prime_ * 26 + 15) &= 251;
                                                                *p.offset(prime_ * 30 + 17) &= 223;
                                                                *p.offset(prime_ * 32 + 18) &= 191;
                                                                *p.offset(prime_ * 36 + 21) &= 254;
                                                                *p.offset(prime_ * 42 + 24) &= 247;
                                                                *p.offset(prime_ * 44 + 25) &= 239;
                                                                *p.offset(prime_ * 50 + 28) &= 127;
                                                                *p.offset(prime_ * 54 + 31) &= 253;
                                                                *p.offset(prime_ * 56 + 32) &= 251;
                                                                *p.offset(prime_ * 60 + 34) &= 223;
                                                                *p.offset(prime_ * 62 + 35) &= 191;
                                                                *p.offset(prime_ * 66 + 38) &= 254;
                                                                *p.offset(prime_ * 72 + 41) &= 247;
                                                                *p.offset(prime_ * 74 + 42) &= 239;
                                                                *p.offset(prime_ * 80 + 45) &= 127;
                                                                *p.offset(prime_ * 84 + 48) &= 253;
                                                                *p.offset(prime_ * 86 + 49) &= 251;
                                                                *p.offset(prime_ * 90 + 51) &= 223;
                                                                *p.offset(prime_ * 92 + 52) &= 191;
                                                                *p.offset(prime_ * 96 + 55) &= 254;
                                                                *p.offset(prime_ * 102 + 58) &= 247;
                                                                *p.offset(prime_ * 104 + 59) &= 239;
                                                                *p.offset(prime_ * 110 + 62) &= 127;
                                                                *p.offset(prime_ * 114 + 65) &= 253;
                                                                *p.offset(prime_ * 116 + 66) &= 251;
                                                                *p.offset(prime_ * 120 + 68) &= 223;
                                                                *p.offset(prime_ * 122 + 69) &= 191;
                                                                *p.offset(prime_ * 126 + 72) &= 254;
                                                                *p.offset(prime_ * 132 + 75) &= 247;
                                                                *p.offset(prime_ * 134 + 76) &= 239;
                                                                *p.offset(prime_ * 140 + 79) &= 127;
                                                                *p.offset(prime_ * 144 + 82) &= 253;
                                                                *p.offset(prime_ * 146 + 83) &= 251;
                                                                *p.offset(prime_ * 150 + 85) &= 223;
                                                                *p.offset(prime_ * 152 + 86) &= 191;
                                                                *p.offset(prime_ * 156 + 89) &= 254;
                                                                *p.offset(prime_ * 162 + 92) &= 247;
                                                                *p.offset(prime_ * 164 + 93) &= 239;
                                                                *p.offset(prime_ * 170 + 96) &= 127;
                                                                *p.offset(prime_ * 174 + 99) &= 253;
                                                                *p.offset(prime_ * 176 + 100) &= 251;

                                                                p = p.offset(prime_ * 180 + 102)
                                                            }
                                                            if p >= end { wi = 192; break 'outer; }
                                                            *p &= 223; p = p.offset(prime_ * 2 + 1);
                                                            break 'label193
                                                           }
                                                           if p >= end { wi = 193; break 'outer; }
                                                           *p &= 191; p = p.offset(prime_ * 4 + 3);
                                                           break 'label194
                                                          }
                                                          if p >= end { wi = 194; break 'outer; }
                                                          *p &= 254; p = p.offset(prime_ * 6 + 3);
                                                          break 'label195
                                                         }
                                                         if p >= end { wi = 195; break 'outer; }
                                                         *p &= 247; p = p.offset(prime_ * 2 + 1);
                                                         break 'label196
                                                        }
                                                        if p >= end { wi = 196; break 'outer; }
                                                        *p &= 239; p = p.offset(prime_ * 6 + 3);
                                                        break 'label197
                                                       }
                                                       if p >= end { wi = 197; break 'outer; }
                                                       *p &= 127; p = p.offset(prime_ * 4 + 3);
                                                       break 'label198
                                                      }
                                                      if p >= end { wi = 198; break 'outer; }
                                                      *p &= 253; p = p.offset(prime_ * 2 + 1);
                                                      break 'label199
                                                     }
                                                     if p >= end { wi = 199; break 'outer; }
                                                     *p &= 251; p = p.offset(prime_ * 4 + 2);
                                                     break 'label200
                                                    }
                                                    if p >= end { wi = 200; break 'outer; }
                                                    *p &= 223; p = p.offset(prime_ * 2 + 1);
                                                    break 'label201
                                                   }
                                                   if p >= end { wi = 201; break 'outer; }
                                                   *p &= 191; p = p.offset(prime_ * 4 + 3);
                                                   break 'label202
                                                  }
                                                  if p >= end { wi = 202; break 'outer; }
                                                  *p &= 254; p = p.offset(prime_ * 6 + 3);
                                                  break 'label203
                                                 }
                                                 if p >= end { wi = 203; break 'outer; }
                                                 *p &= 247; p = p.offset(prime_ * 2 + 1);
                                                 break 'label204
                                                }
                                                if p >= end { wi = 204; break 'outer; }
                                                *p &= 239; p = p.offset(prime_ * 6 + 3);
                                                break 'label205
                                               }
                                               if p >= end { wi = 205; break 'outer; }
                                               *p &= 127; p = p.offset(prime_ * 4 + 3);
                                               break 'label206
                                              }
                                              if p >= end { wi = 206; break 'outer; }
                                              *p &= 253; p = p.offset(prime_ * 2 + 1);
                                              break 'label207
                                             }
                                             if p >= end { wi = 207; break 'outer; }
                                             *p &= 251; p = p.offset(prime_ * 4 + 2);
                                             break 'label208
                                            }
                                            if p >= end { wi = 208; break 'outer; }
                                            *p &= 223; p = p.offset(prime_ * 2 + 1);
                                            break 'label209
                                           }
                                           if p >= end { wi = 209; break 'outer; }
                                           *p &= 191; p = p.offset(prime_ * 4 + 3);
                                           break 'label210
                                          }
                                          if p >= end { wi = 210; break 'outer; }
                                          *p &= 254; p = p.offset(prime_ * 6 + 3);
                                          break 'label211
                                         }
                                         if p >= end { wi = 211; break 'outer; }
                                         *p &= 247; p = p.offset(prime_ * 2 + 1);
                                         break 'label212
                                        }
                                        if p >= end { wi = 212; break 'outer; }
                                        *p &= 239; p = p.offset(prime_ * 6 + 3);
                                        break 'label213
                                       }
                                       if p >= end { wi = 213; break 'outer; }
                                       *p &= 127; p = p.offset(prime_ * 4 + 3);
                                       break 'label214
                                      }
                                      if p >= end { wi = 214; break 'outer; }
                                      *p &= 253; p = p.offset(prime_ * 2 + 1);
                                      break 'label215
                                     }
                                     if p >= end { wi = 215; break 'outer; }
                                     *p &= 251; p = p.offset(prime_ * 4 + 2);
                                     break 'label216
                                    }
                                    if p >= end { wi = 216; break 'outer; }
                                    *p &= 223; p = p.offset(prime_ * 2 + 1);
                                    break 'label217
                                   }
                                   if p >= end { wi = 217; break 'outer; }
                                   *p &= 191; p = p.offset(prime_ * 4 + 3);
                                   break 'label218
                                  }
                                  if p >= end { wi = 218; break 'outer; }
                                  *p &= 254; p = p.offset(prime_ * 6 + 3);
                                  break 'label219
                                 }
                                 if p >= end { wi = 219; break 'outer; }
                                 *p &= 247; p = p.offset(prime_ * 2 + 1);
                                 break 'label220
                                }
                                if p >= end { wi = 220; break 'outer; }
                                *p &= 239; p = p.offset(prime_ * 6 + 3);
                                break 'label221
                               }
                               if p >= end { wi = 221; break 'outer; }
                               *p &= 127; p = p.offset(prime_ * 4 + 3);
                               break 'label222
                              }
                              if p >= end { wi = 222; break 'outer; }
                              *p &= 253; p = p.offset(prime_ * 2 + 1);
                              break 'label223
                             }
                             if p >= end { wi = 223; break 'outer; }
                             *p &= 251; p = p.offset(prime_ * 4 + 2);
                             break 'label224
                            }
                            if p >= end { wi = 224; break 'outer; }
                            *p &= 223; p = p.offset(prime_ * 2 + 1);
                            break 'label225
                           }
                           if p >= end { wi = 225; break 'outer; }
                           *p &= 191; p = p.offset(prime_ * 4 + 3);
                           break 'label226
                          }
                          if p >= end { wi = 226; break 'outer; }
                          *p &= 254; p = p.offset(prime_ * 6 + 3);
                          break 'label227
                         }
                         if p >= end { wi = 227; break 'outer; }
                         *p &= 247; p = p.offset(prime_ * 2 + 1);
                         break 'label228
                        }
                        if p >= end { wi = 228; break 'outer; }
                        *p &= 239; p = p.offset(prime_ * 6 + 3);
                        break 'label229
                       }
                       if p >= end { wi = 229; break 'outer; }
                       *p &= 127; p = p.offset(prime_ * 4 + 3);
                       break 'label230
                      }
                      if p >= end { wi = 230; break 'outer; }
                      *p &= 253; p = p.offset(prime_ * 2 + 1);
                      break 'label231
                     }
                     if p >= end { wi = 231; break 'outer; }
                     *p &= 251; p = p.offset(prime_ * 4 + 2);
                     break 'label232
                    }
                    if p >= end { wi = 232; break 'outer; }
                    *p &= 223; p = p.offset(prime_ * 2 + 1);
                    break 'label233
                   }
                   if p >= end { wi = 233; break 'outer; }
                   *p &= 191; p = p.offset(prime_ * 4 + 3);
                   break 'label234
                  }
                  if p >= end { wi = 234; break 'outer; }
                  *p &= 254; p = p.offset(prime_ * 6 + 3);
                  break 'label235
                 }
                 if p >= end { wi = 235; break 'outer; }
                 *p &= 247; p = p.offset(prime_ * 2 + 1);
                 break 'label236
                }
                if p >= end { wi = 236; break 'outer; }
                *p &= 239; p = p.offset(prime_ * 6 + 3);
                break 'label237
               }
               if p >= end { wi = 237; break 'outer; }
               *p &= 127; p = p.offset(prime_ * 4 + 3);
               break 'label238
              }
              if p >= end { wi = 238; break 'outer; }
              *p &= 253; p = p.offset(prime_ * 2 + 1);
              break 'label239
             }
             if p >= end { wi = 239; break 'outer; }
             *p &= 251; p = p.offset(prime_ * 4 + 2);
             wi = 192
            }
        }
        240...287 => { // 30 * x + 19
            loop {
             'label287: loop {
              'label286: loop {
               'label285: loop {
                'label284: loop {
                 'label283: loop {
                  'label282: loop {
                   'label281: loop {
                    'label280: loop {
                     'label279: loop {
                      'label278: loop {
                       'label277: loop {
                        'label276: loop {
                         'label275: loop {
                          'label274: loop {
                           'label273: loop {
                            'label272: loop {
                             'label271: loop {
                              'label270: loop {
                               'label269: loop {
                                'label268: loop {
                                 'label267: loop {
                                  'label266: loop {
                                   'label265: loop {
                                    'label264: loop {
                                     'label263: loop {
                                      'label262: loop {
                                       'label261: loop {
                                        'label260: loop {
                                         'label259: loop {
                                          'label258: loop {
                                           'label257: loop {
                                            'label256: loop {
                                             'label255: loop {
                                              'label254: loop {
                                               'label253: loop {
                                                'label252: loop {
                                                 'label251: loop {
                                                  'label250: loop {
                                                   'label249: loop {
                                                    'label248: loop {
                                                     'label247: loop {
                                                      'label246: loop {
                                                       'label245: loop {
                                                        'label244: loop {
                                                         'label243: loop {
                                                          'label242: loop {
                                                           'label241: loop {
                                                            'label240: loop {
                                                             match wi {
                                                              240 => break 'label240,
                                                              241 => break 'label241,
                                                              242 => break 'label242,
                                                              243 => break 'label243,
                                                              244 => break 'label244,
                                                              245 => break 'label245,
                                                              246 => break 'label246,
                                                              247 => break 'label247,
                                                              248 => break 'label248,
                                                              249 => break 'label249,
                                                              250 => break 'label250,
                                                              251 => break 'label251,
                                                              252 => break 'label252,
                                                              253 => break 'label253,
                                                              254 => break 'label254,
                                                              255 => break 'label255,
                                                              256 => break 'label256,
                                                              257 => break 'label257,
                                                              258 => break 'label258,
                                                              259 => break 'label259,
                                                              260 => break 'label260,
                                                              261 => break 'label261,
                                                              262 => break 'label262,
                                                              263 => break 'label263,
                                                              264 => break 'label264,
                                                              265 => break 'label265,
                                                              266 => break 'label266,
                                                              267 => break 'label267,
                                                              268 => break 'label268,
                                                              269 => break 'label269,
                                                              270 => break 'label270,
                                                              271 => break 'label271,
                                                              272 => break 'label272,
                                                              273 => break 'label273,
                                                              274 => break 'label274,
                                                              275 => break 'label275,
                                                              276 => break 'label276,
                                                              277 => break 'label277,
                                                              278 => break 'label278,
                                                              279 => break 'label279,
                                                              280 => break 'label280,
                                                              281 => break 'label281,
                                                              282 => break 'label282,
                                                              283 => break 'label283,
                                                              284 => break 'label284,
                                                              285 => break 'label285,
                                                              286 => break 'label286,
                                                              _ => break 'label287,
                                                             }
                                                            }
                                                            while p < loop_end {
                                                                p = ::b(p);
                                                                *p.offset(prime_ * 0 + 0) &= 254;
                                                                *p.offset(prime_ * 4 + 2) &= 239;
                                                                *p.offset(prime_ * 10 + 6) &= 251;
                                                                *p.offset(prime_ * 12 + 7) &= 223;
                                                                *p.offset(prime_ * 18 + 11) &= 247;
                                                                *p.offset(prime_ * 22 + 13) &= 127;
                                                                *p.offset(prime_ * 24 + 15) &= 253;
                                                                *p.offset(prime_ * 28 + 17) &= 191;
                                                                *p.offset(prime_ * 30 + 19) &= 254;
                                                                *p.offset(prime_ * 34 + 21) &= 239;
                                                                *p.offset(prime_ * 40 + 25) &= 251;
                                                                *p.offset(prime_ * 42 + 26) &= 223;
                                                                *p.offset(prime_ * 48 + 30) &= 247;
                                                                *p.offset(prime_ * 52 + 32) &= 127;
                                                                *p.offset(prime_ * 54 + 34) &= 253;
                                                                *p.offset(prime_ * 58 + 36) &= 191;
                                                                *p.offset(prime_ * 60 + 38) &= 254;
                                                                *p.offset(prime_ * 64 + 40) &= 239;
                                                                *p.offset(prime_ * 70 + 44) &= 251;
                                                                *p.offset(prime_ * 72 + 45) &= 223;
                                                                *p.offset(prime_ * 78 + 49) &= 247;
                                                                *p.offset(prime_ * 82 + 51) &= 127;
                                                                *p.offset(prime_ * 84 + 53) &= 253;
                                                                *p.offset(prime_ * 88 + 55) &= 191;
                                                                *p.offset(prime_ * 90 + 57) &= 254;
                                                                *p.offset(prime_ * 94 + 59) &= 239;
                                                                *p.offset(prime_ * 100 + 63) &= 251;
                                                                *p.offset(prime_ * 102 + 64) &= 223;
                                                                *p.offset(prime_ * 108 + 68) &= 247;
                                                                *p.offset(prime_ * 112 + 70) &= 127;
                                                                *p.offset(prime_ * 114 + 72) &= 253;
                                                                *p.offset(prime_ * 118 + 74) &= 191;
                                                                *p.offset(prime_ * 120 + 76) &= 254;
                                                                *p.offset(prime_ * 124 + 78) &= 239;
                                                                *p.offset(prime_ * 130 + 82) &= 251;
                                                                *p.offset(prime_ * 132 + 83) &= 223;
                                                                *p.offset(prime_ * 138 + 87) &= 247;
                                                                *p.offset(prime_ * 142 + 89) &= 127;
                                                                *p.offset(prime_ * 144 + 91) &= 253;
                                                                *p.offset(prime_ * 148 + 93) &= 191;
                                                                *p.offset(prime_ * 150 + 95) &= 254;
                                                                *p.offset(prime_ * 154 + 97) &= 239;
                                                                *p.offset(prime_ * 160 + 101) &= 251;
                                                                *p.offset(prime_ * 162 + 102) &= 223;
                                                                *p.offset(prime_ * 168 + 106) &= 247;
                                                                *p.offset(prime_ * 172 + 108) &= 127;
                                                                *p.offset(prime_ * 174 + 110) &= 253;
                                                                *p.offset(prime_ * 178 + 112) &= 191;

                                                                p = p.offset(prime_ * 180 + 114)
                                                            }
                                                            if p >= end { wi = 240; break 'outer; }
                                                            *p &= 254; p = p.offset(prime_ * 4 + 2);
                                                            break 'label241
                                                           }
                                                           if p >= end { wi = 241; break 'outer; }
                                                           *p &= 239; p = p.offset(prime_ * 6 + 4);
                                                           break 'label242
                                                          }
                                                          if p >= end { wi = 242; break 'outer; }
                                                          *p &= 251; p = p.offset(prime_ * 2 + 1);
                                                          break 'label243
                                                         }
                                                         if p >= end { wi = 243; break 'outer; }
                                                         *p &= 223; p = p.offset(prime_ * 6 + 4);
                                                         break 'label244
                                                        }
                                                        if p >= end { wi = 244; break 'outer; }
                                                        *p &= 247; p = p.offset(prime_ * 4 + 2);
                                                        break 'label245
                                                       }
                                                       if p >= end { wi = 245; break 'outer; }
                                                       *p &= 127; p = p.offset(prime_ * 2 + 2);
                                                       break 'label246
                                                      }
                                                      if p >= end { wi = 246; break 'outer; }
                                                      *p &= 253; p = p.offset(prime_ * 4 + 2);
                                                      break 'label247
                                                     }
                                                     if p >= end { wi = 247; break 'outer; }
                                                     *p &= 191; p = p.offset(prime_ * 2 + 2);
                                                     break 'label248
                                                    }
                                                    if p >= end { wi = 248; break 'outer; }
                                                    *p &= 254; p = p.offset(prime_ * 4 + 2);
                                                    break 'label249
                                                   }
                                                   if p >= end { wi = 249; break 'outer; }
                                                   *p &= 239; p = p.offset(prime_ * 6 + 4);
                                                   break 'label250
                                                  }
                                                  if p >= end { wi = 250; break 'outer; }
                                                  *p &= 251; p = p.offset(prime_ * 2 + 1);
                                                  break 'label251
                                                 }
                                                 if p >= end { wi = 251; break 'outer; }
                                                 *p &= 223; p = p.offset(prime_ * 6 + 4);
                                                 break 'label252
                                                }
                                                if p >= end { wi = 252; break 'outer; }
                                                *p &= 247; p = p.offset(prime_ * 4 + 2);
                                                break 'label253
                                               }
                                               if p >= end { wi = 253; break 'outer; }
                                               *p &= 127; p = p.offset(prime_ * 2 + 2);
                                               break 'label254
                                              }
                                              if p >= end { wi = 254; break 'outer; }
                                              *p &= 253; p = p.offset(prime_ * 4 + 2);
                                              break 'label255
                                             }
                                             if p >= end { wi = 255; break 'outer; }
                                             *p &= 191; p = p.offset(prime_ * 2 + 2);
                                             break 'label256
                                            }
                                            if p >= end { wi = 256; break 'outer; }
                                            *p &= 254; p = p.offset(prime_ * 4 + 2);
                                            break 'label257
                                           }
                                           if p >= end { wi = 257; break 'outer; }
                                           *p &= 239; p = p.offset(prime_ * 6 + 4);
                                           break 'label258
                                          }
                                          if p >= end { wi = 258; break 'outer; }
                                          *p &= 251; p = p.offset(prime_ * 2 + 1);
                                          break 'label259
                                         }
                                         if p >= end { wi = 259; break 'outer; }
                                         *p &= 223; p = p.offset(prime_ * 6 + 4);
                                         break 'label260
                                        }
                                        if p >= end { wi = 260; break 'outer; }
                                        *p &= 247; p = p.offset(prime_ * 4 + 2);
                                        break 'label261
                                       }
                                       if p >= end { wi = 261; break 'outer; }
                                       *p &= 127; p = p.offset(prime_ * 2 + 2);
                                       break 'label262
                                      }
                                      if p >= end { wi = 262; break 'outer; }
                                      *p &= 253; p = p.offset(prime_ * 4 + 2);
                                      break 'label263
                                     }
                                     if p >= end { wi = 263; break 'outer; }
                                     *p &= 191; p = p.offset(prime_ * 2 + 2);
                                     break 'label264
                                    }
                                    if p >= end { wi = 264; break 'outer; }
                                    *p &= 254; p = p.offset(prime_ * 4 + 2);
                                    break 'label265
                                   }
                                   if p >= end { wi = 265; break 'outer; }
                                   *p &= 239; p = p.offset(prime_ * 6 + 4);
                                   break 'label266
                                  }
                                  if p >= end { wi = 266; break 'outer; }
                                  *p &= 251; p = p.offset(prime_ * 2 + 1);
                                  break 'label267
                                 }
                                 if p >= end { wi = 267; break 'outer; }
                                 *p &= 223; p = p.offset(prime_ * 6 + 4);
                                 break 'label268
                                }
                                if p >= end { wi = 268; break 'outer; }
                                *p &= 247; p = p.offset(prime_ * 4 + 2);
                                break 'label269
                               }
                               if p >= end { wi = 269; break 'outer; }
                               *p &= 127; p = p.offset(prime_ * 2 + 2);
                               break 'label270
                              }
                              if p >= end { wi = 270; break 'outer; }
                              *p &= 253; p = p.offset(prime_ * 4 + 2);
                              break 'label271
                             }
                             if p >= end { wi = 271; break 'outer; }
                             *p &= 191; p = p.offset(prime_ * 2 + 2);
                             break 'label272
                            }
                            if p >= end { wi = 272; break 'outer; }
                            *p &= 254; p = p.offset(prime_ * 4 + 2);
                            break 'label273
                           }
                           if p >= end { wi = 273; break 'outer; }
                           *p &= 239; p = p.offset(prime_ * 6 + 4);
                           break 'label274
                          }
                          if p >= end { wi = 274; break 'outer; }
                          *p &= 251; p = p.offset(prime_ * 2 + 1);
                          break 'label275
                         }
                         if p >= end { wi = 275; break 'outer; }
                         *p &= 223; p = p.offset(prime_ * 6 + 4);
                         break 'label276
                        }
                        if p >= end { wi = 276; break 'outer; }
                        *p &= 247; p = p.offset(prime_ * 4 + 2);
                        break 'label277
                       }
                       if p >= end { wi = 277; break 'outer; }
                       *p &= 127; p = p.offset(prime_ * 2 + 2);
                       break 'label278
                      }
                      if p >= end { wi = 278; break 'outer; }
                      *p &= 253; p = p.offset(prime_ * 4 + 2);
                      break 'label279
                     }
                     if p >= end { wi = 279; break 'outer; }
                     *p &= 191; p = p.offset(prime_ * 2 + 2);
                     break 'label280
                    }
                    if p >= end { wi = 280; break 'outer; }
                    *p &= 254; p = p.offset(prime_ * 4 + 2);
                    break 'label281
                   }
                   if p >= end { wi = 281; break 'outer; }
                   *p &= 239; p = p.offset(prime_ * 6 + 4);
                   break 'label282
                  }
                  if p >= end { wi = 282; break 'outer; }
                  *p &= 251; p = p.offset(prime_ * 2 + 1);
                  break 'label283
                 }
                 if p >= end { wi = 283; break 'outer; }
                 *p &= 223; p = p.offset(prime_ * 6 + 4);
                 break 'label284
                }
                if p >= end { wi = 284; break 'outer; }
                *p &= 247; p = p.offset(prime_ * 4 + 2);
                break 'label285
               }
               if p >= end { wi = 285; break 'outer; }
               *p &= 127; p = p.offset(prime_ * 2 + 2);
               break 'label286
              }
              if p >= end { wi = 286; break 'outer; }
              *p &= 253; p = p.offset(prime_ * 4 + 2);
              break 'label287
             }
             if p >= end { wi = 287; break 'outer; }
             *p &= 191; p = p.offset(prime_ * 2 + 2);
             wi = 240
            }
        }
        288...335 => { // 30 * x + 23
            loop {
             'label335: loop {
              'label334: loop {
               'label333: loop {
                'label332: loop {
                 'label331: loop {
                  'label330: loop {
                   'label329: loop {
                    'label328: loop {
                     'label327: loop {
                      'label326: loop {
                       'label325: loop {
                        'label324: loop {
                         'label323: loop {
                          'label322: loop {
                           'label321: loop {
                            'label320: loop {
                             'label319: loop {
                              'label318: loop {
                               'label317: loop {
                                'label316: loop {
                                 'label315: loop {
                                  'label314: loop {
                                   'label313: loop {
                                    'label312: loop {
                                     'label311: loop {
                                      'label310: loop {
                                       'label309: loop {
                                        'label308: loop {
                                         'label307: loop {
                                          'label306: loop {
                                           'label305: loop {
                                            'label304: loop {
                                             'label303: loop {
                                              'label302: loop {
                                               'label301: loop {
                                                'label300: loop {
                                                 'label299: loop {
                                                  'label298: loop {
                                                   'label297: loop {
                                                    'label296: loop {
                                                     'label295: loop {
                                                      'label294: loop {
                                                       'label293: loop {
                                                        'label292: loop {
                                                         'label291: loop {
                                                          'label290: loop {
                                                           'label289: loop {
                                                            'label288: loop {
                                                             match wi {
                                                              288 => break 'label288,
                                                              289 => break 'label289,
                                                              290 => break 'label290,
                                                              291 => break 'label291,
                                                              292 => break 'label292,
                                                              293 => break 'label293,
                                                              294 => break 'label294,
                                                              295 => break 'label295,
                                                              296 => break 'label296,
                                                              297 => break 'label297,
                                                              298 => break 'label298,
                                                              299 => break 'label299,
                                                              300 => break 'label300,
                                                              301 => break 'label301,
                                                              302 => break 'label302,
                                                              303 => break 'label303,
                                                              304 => break 'label304,
                                                              305 => break 'label305,
                                                              306 => break 'label306,
                                                              307 => break 'label307,
                                                              308 => break 'label308,
                                                              309 => break 'label309,
                                                              310 => break 'label310,
                                                              311 => break 'label311,
                                                              312 => break 'label312,
                                                              313 => break 'label313,
                                                              314 => break 'label314,
                                                              315 => break 'label315,
                                                              316 => break 'label316,
                                                              317 => break 'label317,
                                                              318 => break 'label318,
                                                              319 => break 'label319,
                                                              320 => break 'label320,
                                                              321 => break 'label321,
                                                              322 => break 'label322,
                                                              323 => break 'label323,
                                                              324 => break 'label324,
                                                              325 => break 'label325,
                                                              326 => break 'label326,
                                                              327 => break 'label327,
                                                              328 => break 'label328,
                                                              329 => break 'label329,
                                                              330 => break 'label330,
                                                              331 => break 'label331,
                                                              332 => break 'label332,
                                                              333 => break 'label333,
                                                              334 => break 'label334,
                                                              _ => break 'label335,
                                                             }
                                                            }
                                                            while p < loop_end {
                                                                p = ::b(p);
                                                                *p.offset(prime_ * 0 + 0) &= 223;
                                                                *p.offset(prime_ * 6 + 5) &= 253;
                                                                *p.offset(prime_ * 8 + 6) &= 191;
                                                                *p.offset(prime_ * 14 + 11) &= 251;
                                                                *p.offset(prime_ * 18 + 14) &= 247;
                                                                *p.offset(prime_ * 20 + 15) &= 127;
                                                                *p.offset(prime_ * 24 + 19) &= 254;
                                                                *p.offset(prime_ * 26 + 20) &= 239;
                                                                *p.offset(prime_ * 30 + 23) &= 223;
                                                                *p.offset(prime_ * 36 + 28) &= 253;
                                                                *p.offset(prime_ * 38 + 29) &= 191;
                                                                *p.offset(prime_ * 44 + 34) &= 251;
                                                                *p.offset(prime_ * 48 + 37) &= 247;
                                                                *p.offset(prime_ * 50 + 38) &= 127;
                                                                *p.offset(prime_ * 54 + 42) &= 254;
                                                                *p.offset(prime_ * 56 + 43) &= 239;
                                                                *p.offset(prime_ * 60 + 46) &= 223;
                                                                *p.offset(prime_ * 66 + 51) &= 253;
                                                                *p.offset(prime_ * 68 + 52) &= 191;
                                                                *p.offset(prime_ * 74 + 57) &= 251;
                                                                *p.offset(prime_ * 78 + 60) &= 247;
                                                                *p.offset(prime_ * 80 + 61) &= 127;
                                                                *p.offset(prime_ * 84 + 65) &= 254;
                                                                *p.offset(prime_ * 86 + 66) &= 239;
                                                                *p.offset(prime_ * 90 + 69) &= 223;
                                                                *p.offset(prime_ * 96 + 74) &= 253;
                                                                *p.offset(prime_ * 98 + 75) &= 191;
                                                                *p.offset(prime_ * 104 + 80) &= 251;
                                                                *p.offset(prime_ * 108 + 83) &= 247;
                                                                *p.offset(prime_ * 110 + 84) &= 127;
                                                                *p.offset(prime_ * 114 + 88) &= 254;
                                                                *p.offset(prime_ * 116 + 89) &= 239;
                                                                *p.offset(prime_ * 120 + 92) &= 223;
                                                                *p.offset(prime_ * 126 + 97) &= 253;
                                                                *p.offset(prime_ * 128 + 98) &= 191;
                                                                *p.offset(prime_ * 134 + 103) &= 251;
                                                                *p.offset(prime_ * 138 + 106) &= 247;
                                                                *p.offset(prime_ * 140 + 107) &= 127;
                                                                *p.offset(prime_ * 144 + 111) &= 254;
                                                                *p.offset(prime_ * 146 + 112) &= 239;
                                                                *p.offset(prime_ * 150 + 115) &= 223;
                                                                *p.offset(prime_ * 156 + 120) &= 253;
                                                                *p.offset(prime_ * 158 + 121) &= 191;
                                                                *p.offset(prime_ * 164 + 126) &= 251;
                                                                *p.offset(prime_ * 168 + 129) &= 247;
                                                                *p.offset(prime_ * 170 + 130) &= 127;
                                                                *p.offset(prime_ * 174 + 134) &= 254;
                                                                *p.offset(prime_ * 176 + 135) &= 239;

                                                                p = p.offset(prime_ * 180 + 138)
                                                            }
                                                            if p >= end { wi = 288; break 'outer; }
                                                            *p &= 223; p = p.offset(prime_ * 6 + 5);
                                                            break 'label289
                                                           }
                                                           if p >= end { wi = 289; break 'outer; }
                                                           *p &= 253; p = p.offset(prime_ * 2 + 1);
                                                           break 'label290
                                                          }
                                                          if p >= end { wi = 290; break 'outer; }
                                                          *p &= 191; p = p.offset(prime_ * 6 + 5);
                                                          break 'label291
                                                         }
                                                         if p >= end { wi = 291; break 'outer; }
                                                         *p &= 251; p = p.offset(prime_ * 4 + 3);
                                                         break 'label292
                                                        }
                                                        if p >= end { wi = 292; break 'outer; }
                                                        *p &= 247; p = p.offset(prime_ * 2 + 1);
                                                        break 'label293
                                                       }
                                                       if p >= end { wi = 293; break 'outer; }
                                                       *p &= 127; p = p.offset(prime_ * 4 + 4);
                                                       break 'label294
                                                      }
                                                      if p >= end { wi = 294; break 'outer; }
                                                      *p &= 254; p = p.offset(prime_ * 2 + 1);
                                                      break 'label295
                                                     }
                                                     if p >= end { wi = 295; break 'outer; }
                                                     *p &= 239; p = p.offset(prime_ * 4 + 3);
                                                     break 'label296
                                                    }
                                                    if p >= end { wi = 296; break 'outer; }
                                                    *p &= 223; p = p.offset(prime_ * 6 + 5);
                                                    break 'label297
                                                   }
                                                   if p >= end { wi = 297; break 'outer; }
                                                   *p &= 253; p = p.offset(prime_ * 2 + 1);
                                                   break 'label298
                                                  }
                                                  if p >= end { wi = 298; break 'outer; }
                                                  *p &= 191; p = p.offset(prime_ * 6 + 5);
                                                  break 'label299
                                                 }
                                                 if p >= end { wi = 299; break 'outer; }
                                                 *p &= 251; p = p.offset(prime_ * 4 + 3);
                                                 break 'label300
                                                }
                                                if p >= end { wi = 300; break 'outer; }
                                                *p &= 247; p = p.offset(prime_ * 2 + 1);
                                                break 'label301
                                               }
                                               if p >= end { wi = 301; break 'outer; }
                                               *p &= 127; p = p.offset(prime_ * 4 + 4);
                                               break 'label302
                                              }
                                              if p >= end { wi = 302; break 'outer; }
                                              *p &= 254; p = p.offset(prime_ * 2 + 1);
                                              break 'label303
                                             }
                                             if p >= end { wi = 303; break 'outer; }
                                             *p &= 239; p = p.offset(prime_ * 4 + 3);
                                             break 'label304
                                            }
                                            if p >= end { wi = 304; break 'outer; }
                                            *p &= 223; p = p.offset(prime_ * 6 + 5);
                                            break 'label305
                                           }
                                           if p >= end { wi = 305; break 'outer; }
                                           *p &= 253; p = p.offset(prime_ * 2 + 1);
                                           break 'label306
                                          }
                                          if p >= end { wi = 306; break 'outer; }
                                          *p &= 191; p = p.offset(prime_ * 6 + 5);
                                          break 'label307
                                         }
                                         if p >= end { wi = 307; break 'outer; }
                                         *p &= 251; p = p.offset(prime_ * 4 + 3);
                                         break 'label308
                                        }
                                        if p >= end { wi = 308; break 'outer; }
                                        *p &= 247; p = p.offset(prime_ * 2 + 1);
                                        break 'label309
                                       }
                                       if p >= end { wi = 309; break 'outer; }
                                       *p &= 127; p = p.offset(prime_ * 4 + 4);
                                       break 'label310
                                      }
                                      if p >= end { wi = 310; break 'outer; }
                                      *p &= 254; p = p.offset(prime_ * 2 + 1);
                                      break 'label311
                                     }
                                     if p >= end { wi = 311; break 'outer; }
                                     *p &= 239; p = p.offset(prime_ * 4 + 3);
                                     break 'label312
                                    }
                                    if p >= end { wi = 312; break 'outer; }
                                    *p &= 223; p = p.offset(prime_ * 6 + 5);
                                    break 'label313
                                   }
                                   if p >= end { wi = 313; break 'outer; }
                                   *p &= 253; p = p.offset(prime_ * 2 + 1);
                                   break 'label314
                                  }
                                  if p >= end { wi = 314; break 'outer; }
                                  *p &= 191; p = p.offset(prime_ * 6 + 5);
                                  break 'label315
                                 }
                                 if p >= end { wi = 315; break 'outer; }
                                 *p &= 251; p = p.offset(prime_ * 4 + 3);
                                 break 'label316
                                }
                                if p >= end { wi = 316; break 'outer; }
                                *p &= 247; p = p.offset(prime_ * 2 + 1);
                                break 'label317
                               }
                               if p >= end { wi = 317; break 'outer; }
                               *p &= 127; p = p.offset(prime_ * 4 + 4);
                               break 'label318
                              }
                              if p >= end { wi = 318; break 'outer; }
                              *p &= 254; p = p.offset(prime_ * 2 + 1);
                              break 'label319
                             }
                             if p >= end { wi = 319; break 'outer; }
                             *p &= 239; p = p.offset(prime_ * 4 + 3);
                             break 'label320
                            }
                            if p >= end { wi = 320; break 'outer; }
                            *p &= 223; p = p.offset(prime_ * 6 + 5);
                            break 'label321
                           }
                           if p >= end { wi = 321; break 'outer; }
                           *p &= 253; p = p.offset(prime_ * 2 + 1);
                           break 'label322
                          }
                          if p >= end { wi = 322; break 'outer; }
                          *p &= 191; p = p.offset(prime_ * 6 + 5);
                          break 'label323
                         }
                         if p >= end { wi = 323; break 'outer; }
                         *p &= 251; p = p.offset(prime_ * 4 + 3);
                         break 'label324
                        }
                        if p >= end { wi = 324; break 'outer; }
                        *p &= 247; p = p.offset(prime_ * 2 + 1);
                        break 'label325
                       }
                       if p >= end { wi = 325; break 'outer; }
                       *p &= 127; p = p.offset(prime_ * 4 + 4);
                       break 'label326
                      }
                      if p >= end { wi = 326; break 'outer; }
                      *p &= 254; p = p.offset(prime_ * 2 + 1);
                      break 'label327
                     }
                     if p >= end { wi = 327; break 'outer; }
                     *p &= 239; p = p.offset(prime_ * 4 + 3);
                     break 'label328
                    }
                    if p >= end { wi = 328; break 'outer; }
                    *p &= 223; p = p.offset(prime_ * 6 + 5);
                    break 'label329
                   }
                   if p >= end { wi = 329; break 'outer; }
                   *p &= 253; p = p.offset(prime_ * 2 + 1);
                   break 'label330
                  }
                  if p >= end { wi = 330; break 'outer; }
                  *p &= 191; p = p.offset(prime_ * 6 + 5);
                  break 'label331
                 }
                 if p >= end { wi = 331; break 'outer; }
                 *p &= 251; p = p.offset(prime_ * 4 + 3);
                 break 'label332
                }
                if p >= end { wi = 332; break 'outer; }
                *p &= 247; p = p.offset(prime_ * 2 + 1);
                break 'label333
               }
               if p >= end { wi = 333; break 'outer; }
               *p &= 127; p = p.offset(prime_ * 4 + 4);
               break 'label334
              }
              if p >= end { wi = 334; break 'outer; }
              *p &= 254; p = p.offset(prime_ * 2 + 1);
              break 'label335
             }
             if p >= end { wi = 335; break 'outer; }
             *p &= 239; p = p.offset(prime_ * 4 + 3);
             wi = 288
            }
        }
        336...383 => { // 30 * x + 29
            loop {
             'label383: loop {
              'label382: loop {
               'label381: loop {
                'label380: loop {
                 'label379: loop {
                  'label378: loop {
                   'label377: loop {
                    'label376: loop {
                     'label375: loop {
                      'label374: loop {
                       'label373: loop {
                        'label372: loop {
                         'label371: loop {
                          'label370: loop {
                           'label369: loop {
                            'label368: loop {
                             'label367: loop {
                              'label366: loop {
                               'label365: loop {
                                'label364: loop {
                                 'label363: loop {
                                  'label362: loop {
                                   'label361: loop {
                                    'label360: loop {
                                     'label359: loop {
                                      'label358: loop {
                                       'label357: loop {
                                        'label356: loop {
                                         'label355: loop {
                                          'label354: loop {
                                           'label353: loop {
                                            'label352: loop {
                                             'label351: loop {
                                              'label350: loop {
                                               'label349: loop {
                                                'label348: loop {
                                                 'label347: loop {
                                                  'label346: loop {
                                                   'label345: loop {
                                                    'label344: loop {
                                                     'label343: loop {
                                                      'label342: loop {
                                                       'label341: loop {
                                                        'label340: loop {
                                                         'label339: loop {
                                                          'label338: loop {
                                                           'label337: loop {
                                                            'label336: loop {
                                                             match wi {
                                                              336 => break 'label336,
                                                              337 => break 'label337,
                                                              338 => break 'label338,
                                                              339 => break 'label339,
                                                              340 => break 'label340,
                                                              341 => break 'label341,
                                                              342 => break 'label342,
                                                              343 => break 'label343,
                                                              344 => break 'label344,
                                                              345 => break 'label345,
                                                              346 => break 'label346,
                                                              347 => break 'label347,
                                                              348 => break 'label348,
                                                              349 => break 'label349,
                                                              350 => break 'label350,
                                                              351 => break 'label351,
                                                              352 => break 'label352,
                                                              353 => break 'label353,
                                                              354 => break 'label354,
                                                              355 => break 'label355,
                                                              356 => break 'label356,
                                                              357 => break 'label357,
                                                              358 => break 'label358,
                                                              359 => break 'label359,
                                                              360 => break 'label360,
                                                              361 => break 'label361,
                                                              362 => break 'label362,
                                                              363 => break 'label363,
                                                              364 => break 'label364,
                                                              365 => break 'label365,
                                                              366 => break 'label366,
                                                              367 => break 'label367,
                                                              368 => break 'label368,
                                                              369 => break 'label369,
                                                              370 => break 'label370,
                                                              371 => break 'label371,
                                                              372 => break 'label372,
                                                              373 => break 'label373,
                                                              374 => break 'label374,
                                                              375 => break 'label375,
                                                              376 => break 'label376,
                                                              377 => break 'label377,
                                                              378 => break 'label378,
                                                              379 => break 'label379,
                                                              380 => break 'label380,
                                                              381 => break 'label381,
                                                              382 => break 'label382,
                                                              _ => break 'label383,
                                                             }
                                                            }
                                                            while p < loop_end {
                                                                p = ::b(p);
                                                                *p.offset(prime_ * 0 + 0) &= 254;
                                                                *p.offset(prime_ * 2 + 1) &= 127;
                                                                *p.offset(prime_ * 8 + 7) &= 191;
                                                                *p.offset(prime_ * 12 + 11) &= 223;
                                                                *p.offset(prime_ * 14 + 13) &= 239;
                                                                *p.offset(prime_ * 18 + 17) &= 247;
                                                                *p.offset(prime_ * 20 + 19) &= 251;
                                                                *p.offset(prime_ * 24 + 23) &= 253;
                                                                *p.offset(prime_ * 30 + 29) &= 254;
                                                                *p.offset(prime_ * 32 + 30) &= 127;
                                                                *p.offset(prime_ * 38 + 36) &= 191;
                                                                *p.offset(prime_ * 42 + 40) &= 223;
                                                                *p.offset(prime_ * 44 + 42) &= 239;
                                                                *p.offset(prime_ * 48 + 46) &= 247;
                                                                *p.offset(prime_ * 50 + 48) &= 251;
                                                                *p.offset(prime_ * 54 + 52) &= 253;
                                                                *p.offset(prime_ * 60 + 58) &= 254;
                                                                *p.offset(prime_ * 62 + 59) &= 127;
                                                                *p.offset(prime_ * 68 + 65) &= 191;
                                                                *p.offset(prime_ * 72 + 69) &= 223;
                                                                *p.offset(prime_ * 74 + 71) &= 239;
                                                                *p.offset(prime_ * 78 + 75) &= 247;
                                                                *p.offset(prime_ * 80 + 77) &= 251;
                                                                *p.offset(prime_ * 84 + 81) &= 253;
                                                                *p.offset(prime_ * 90 + 87) &= 254;
                                                                *p.offset(prime_ * 92 + 88) &= 127;
                                                                *p.offset(prime_ * 98 + 94) &= 191;
                                                                *p.offset(prime_ * 102 + 98) &= 223;
                                                                *p.offset(prime_ * 104 + 100) &= 239;
                                                                *p.offset(prime_ * 108 + 104) &= 247;
                                                                *p.offset(prime_ * 110 + 106) &= 251;
                                                                *p.offset(prime_ * 114 + 110) &= 253;
                                                                *p.offset(prime_ * 120 + 116) &= 254;
                                                                *p.offset(prime_ * 122 + 117) &= 127;
                                                                *p.offset(prime_ * 128 + 123) &= 191;
                                                                *p.offset(prime_ * 132 + 127) &= 223;
                                                                *p.offset(prime_ * 134 + 129) &= 239;
                                                                *p.offset(prime_ * 138 + 133) &= 247;
                                                                *p.offset(prime_ * 140 + 135) &= 251;
                                                                *p.offset(prime_ * 144 + 139) &= 253;
                                                                *p.offset(prime_ * 150 + 145) &= 254;
                                                                *p.offset(prime_ * 152 + 146) &= 127;
                                                                *p.offset(prime_ * 158 + 152) &= 191;
                                                                *p.offset(prime_ * 162 + 156) &= 223;
                                                                *p.offset(prime_ * 164 + 158) &= 239;
                                                                *p.offset(prime_ * 168 + 162) &= 247;
                                                                *p.offset(prime_ * 170 + 164) &= 251;
                                                                *p.offset(prime_ * 174 + 168) &= 253;

                                                                p = p.offset(prime_ * 180 + 174)
                                                            }
                                                            if p >= end { wi = 336; break 'outer; }
                                                            *p &= 254; p = p.offset(prime_ * 2 + 1);
                                                            break 'label337
                                                           }
                                                           if p >= end { wi = 337; break 'outer; }
                                                           *p &= 127; p = p.offset(prime_ * 6 + 6);
                                                           break 'label338
                                                          }
                                                          if p >= end { wi = 338; break 'outer; }
                                                          *p &= 191; p = p.offset(prime_ * 4 + 4);
                                                          break 'label339
                                                         }
                                                         if p >= end { wi = 339; break 'outer; }
                                                         *p &= 223; p = p.offset(prime_ * 2 + 2);
                                                         break 'label340
                                                        }
                                                        if p >= end { wi = 340; break 'outer; }
                                                        *p &= 239; p = p.offset(prime_ * 4 + 4);
                                                        break 'label341
                                                       }
                                                       if p >= end { wi = 341; break 'outer; }
                                                       *p &= 247; p = p.offset(prime_ * 2 + 2);
                                                       break 'label342
                                                      }
                                                      if p >= end { wi = 342; break 'outer; }
                                                      *p &= 251; p = p.offset(prime_ * 4 + 4);
                                                      break 'label343
                                                     }
                                                     if p >= end { wi = 343; break 'outer; }
                                                     *p &= 253; p = p.offset(prime_ * 6 + 6);
                                                     break 'label344
                                                    }
                                                    if p >= end { wi = 344; break 'outer; }
                                                    *p &= 254; p = p.offset(prime_ * 2 + 1);
                                                    break 'label345
                                                   }
                                                   if p >= end { wi = 345; break 'outer; }
                                                   *p &= 127; p = p.offset(prime_ * 6 + 6);
                                                   break 'label346
                                                  }
                                                  if p >= end { wi = 346; break 'outer; }
                                                  *p &= 191; p = p.offset(prime_ * 4 + 4);
                                                  break 'label347
                                                 }
                                                 if p >= end { wi = 347; break 'outer; }
                                                 *p &= 223; p = p.offset(prime_ * 2 + 2);
                                                 break 'label348
                                                }
                                                if p >= end { wi = 348; break 'outer; }
                                                *p &= 239; p = p.offset(prime_ * 4 + 4);
                                                break 'label349
                                               }
                                               if p >= end { wi = 349; break 'outer; }
                                               *p &= 247; p = p.offset(prime_ * 2 + 2);
                                               break 'label350
                                              }
                                              if p >= end { wi = 350; break 'outer; }
                                              *p &= 251; p = p.offset(prime_ * 4 + 4);
                                              break 'label351
                                             }
                                             if p >= end { wi = 351; break 'outer; }
                                             *p &= 253; p = p.offset(prime_ * 6 + 6);
                                             break 'label352
                                            }
                                            if p >= end { wi = 352; break 'outer; }
                                            *p &= 254; p = p.offset(prime_ * 2 + 1);
                                            break 'label353
                                           }
                                           if p >= end { wi = 353; break 'outer; }
                                           *p &= 127; p = p.offset(prime_ * 6 + 6);
                                           break 'label354
                                          }
                                          if p >= end { wi = 354; break 'outer; }
                                          *p &= 191; p = p.offset(prime_ * 4 + 4);
                                          break 'label355
                                         }
                                         if p >= end { wi = 355; break 'outer; }
                                         *p &= 223; p = p.offset(prime_ * 2 + 2);
                                         break 'label356
                                        }
                                        if p >= end { wi = 356; break 'outer; }
                                        *p &= 239; p = p.offset(prime_ * 4 + 4);
                                        break 'label357
                                       }
                                       if p >= end { wi = 357; break 'outer; }
                                       *p &= 247; p = p.offset(prime_ * 2 + 2);
                                       break 'label358
                                      }
                                      if p >= end { wi = 358; break 'outer; }
                                      *p &= 251; p = p.offset(prime_ * 4 + 4);
                                      break 'label359
                                     }
                                     if p >= end { wi = 359; break 'outer; }
                                     *p &= 253; p = p.offset(prime_ * 6 + 6);
                                     break 'label360
                                    }
                                    if p >= end { wi = 360; break 'outer; }
                                    *p &= 254; p = p.offset(prime_ * 2 + 1);
                                    break 'label361
                                   }
                                   if p >= end { wi = 361; break 'outer; }
                                   *p &= 127; p = p.offset(prime_ * 6 + 6);
                                   break 'label362
                                  }
                                  if p >= end { wi = 362; break 'outer; }
                                  *p &= 191; p = p.offset(prime_ * 4 + 4);
                                  break 'label363
                                 }
                                 if p >= end { wi = 363; break 'outer; }
                                 *p &= 223; p = p.offset(prime_ * 2 + 2);
                                 break 'label364
                                }
                                if p >= end { wi = 364; break 'outer; }
                                *p &= 239; p = p.offset(prime_ * 4 + 4);
                                break 'label365
                               }
                               if p >= end { wi = 365; break 'outer; }
                               *p &= 247; p = p.offset(prime_ * 2 + 2);
                               break 'label366
                              }
                              if p >= end { wi = 366; break 'outer; }
                              *p &= 251; p = p.offset(prime_ * 4 + 4);
                              break 'label367
                             }
                             if p >= end { wi = 367; break 'outer; }
                             *p &= 253; p = p.offset(prime_ * 6 + 6);
                             break 'label368
                            }
                            if p >= end { wi = 368; break 'outer; }
                            *p &= 254; p = p.offset(prime_ * 2 + 1);
                            break 'label369
                           }
                           if p >= end { wi = 369; break 'outer; }
                           *p &= 127; p = p.offset(prime_ * 6 + 6);
                           break 'label370
                          }
                          if p >= end { wi = 370; break 'outer; }
                          *p &= 191; p = p.offset(prime_ * 4 + 4);
                          break 'label371
                         }
                         if p >= end { wi = 371; break 'outer; }
                         *p &= 223; p = p.offset(prime_ * 2 + 2);
                         break 'label372
                        }
                        if p >= end { wi = 372; break 'outer; }
                        *p &= 239; p = p.offset(prime_ * 4 + 4);
                        break 'label373
                       }
                       if p >= end { wi = 373; break 'outer; }
                       *p &= 247; p = p.offset(prime_ * 2 + 2);
                       break 'label374
                      }
                      if p >= end { wi = 374; break 'outer; }
                      *p &= 251; p = p.offset(prime_ * 4 + 4);
                      break 'label375
                     }
                     if p >= end { wi = 375; break 'outer; }
                     *p &= 253; p = p.offset(prime_ * 6 + 6);
                     break 'label376
                    }
                    if p >= end { wi = 376; break 'outer; }
                    *p &= 254; p = p.offset(prime_ * 2 + 1);
                    break 'label377
                   }
                   if p >= end { wi = 377; break 'outer; }
                   *p &= 127; p = p.offset(prime_ * 6 + 6);
                   break 'label378
                  }
                  if p >= end { wi = 378; break 'outer; }
                  *p &= 191; p = p.offset(prime_ * 4 + 4);
                  break 'label379
                 }
                 if p >= end { wi = 379; break 'outer; }
                 *p &= 223; p = p.offset(prime_ * 2 + 2);
                 break 'label380
                }
                if p >= end { wi = 380; break 'outer; }
                *p &= 239; p = p.offset(prime_ * 4 + 4);
                break 'label381
               }
               if p >= end { wi = 381; break 'outer; }
               *p &= 247; p = p.offset(prime_ * 2 + 2);
               break 'label382
              }
              if p >= end { wi = 382; break 'outer; }
              *p &= 251; p = p.offset(prime_ * 4 + 4);
              break 'label383
             }
             if p >= end { wi = 383; break 'outer; }
             *p &= 253; p = p.offset(prime_ * 6 + 6);
             wi = 336
            }
        }
        _ => unreachable!("{}", wi),
    }
    }
    *si_ = (p as usize).wrapping_sub(end as usize);
    *wi_ = wi;
}
