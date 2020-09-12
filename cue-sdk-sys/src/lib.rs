#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::os::raw::{c_char, c_int, c_uint, c_void};

pub const CorsairLedId_CLI_Invalid: CorsairLedId = 0;
pub const CorsairLedId_CLK_Escape: CorsairLedId = 1;
pub const CorsairLedId_CLK_F1: CorsairLedId = 2;
pub const CorsairLedId_CLK_F2: CorsairLedId = 3;
pub const CorsairLedId_CLK_F3: CorsairLedId = 4;
pub const CorsairLedId_CLK_F4: CorsairLedId = 5;
pub const CorsairLedId_CLK_F5: CorsairLedId = 6;
pub const CorsairLedId_CLK_F6: CorsairLedId = 7;
pub const CorsairLedId_CLK_F7: CorsairLedId = 8;
pub const CorsairLedId_CLK_F8: CorsairLedId = 9;
pub const CorsairLedId_CLK_F9: CorsairLedId = 10;
pub const CorsairLedId_CLK_F10: CorsairLedId = 11;
pub const CorsairLedId_CLK_F11: CorsairLedId = 12;
pub const CorsairLedId_CLK_GraveAccentAndTilde: CorsairLedId = 13;
pub const CorsairLedId_CLK_1: CorsairLedId = 14;
pub const CorsairLedId_CLK_2: CorsairLedId = 15;
pub const CorsairLedId_CLK_3: CorsairLedId = 16;
pub const CorsairLedId_CLK_4: CorsairLedId = 17;
pub const CorsairLedId_CLK_5: CorsairLedId = 18;
pub const CorsairLedId_CLK_6: CorsairLedId = 19;
pub const CorsairLedId_CLK_7: CorsairLedId = 20;
pub const CorsairLedId_CLK_8: CorsairLedId = 21;
pub const CorsairLedId_CLK_9: CorsairLedId = 22;
pub const CorsairLedId_CLK_0: CorsairLedId = 23;
pub const CorsairLedId_CLK_MinusAndUnderscore: CorsairLedId = 24;
pub const CorsairLedId_CLK_Tab: CorsairLedId = 25;
pub const CorsairLedId_CLK_Q: CorsairLedId = 26;
pub const CorsairLedId_CLK_W: CorsairLedId = 27;
pub const CorsairLedId_CLK_E: CorsairLedId = 28;
pub const CorsairLedId_CLK_R: CorsairLedId = 29;
pub const CorsairLedId_CLK_T: CorsairLedId = 30;
pub const CorsairLedId_CLK_Y: CorsairLedId = 31;
pub const CorsairLedId_CLK_U: CorsairLedId = 32;
pub const CorsairLedId_CLK_I: CorsairLedId = 33;
pub const CorsairLedId_CLK_O: CorsairLedId = 34;
pub const CorsairLedId_CLK_P: CorsairLedId = 35;
pub const CorsairLedId_CLK_BracketLeft: CorsairLedId = 36;
pub const CorsairLedId_CLK_CapsLock: CorsairLedId = 37;
pub const CorsairLedId_CLK_A: CorsairLedId = 38;
pub const CorsairLedId_CLK_S: CorsairLedId = 39;
pub const CorsairLedId_CLK_D: CorsairLedId = 40;
pub const CorsairLedId_CLK_F: CorsairLedId = 41;
pub const CorsairLedId_CLK_G: CorsairLedId = 42;
pub const CorsairLedId_CLK_H: CorsairLedId = 43;
pub const CorsairLedId_CLK_J: CorsairLedId = 44;
pub const CorsairLedId_CLK_K: CorsairLedId = 45;
pub const CorsairLedId_CLK_L: CorsairLedId = 46;
pub const CorsairLedId_CLK_SemicolonAndColon: CorsairLedId = 47;
pub const CorsairLedId_CLK_ApostropheAndDoubleQuote: CorsairLedId = 48;
pub const CorsairLedId_CLK_LeftShift: CorsairLedId = 49;
pub const CorsairLedId_CLK_NonUsBackslash: CorsairLedId = 50;
pub const CorsairLedId_CLK_Z: CorsairLedId = 51;
pub const CorsairLedId_CLK_X: CorsairLedId = 52;
pub const CorsairLedId_CLK_C: CorsairLedId = 53;
pub const CorsairLedId_CLK_V: CorsairLedId = 54;
pub const CorsairLedId_CLK_B: CorsairLedId = 55;
pub const CorsairLedId_CLK_N: CorsairLedId = 56;
pub const CorsairLedId_CLK_M: CorsairLedId = 57;
pub const CorsairLedId_CLK_CommaAndLessThan: CorsairLedId = 58;
pub const CorsairLedId_CLK_PeriodAndBiggerThan: CorsairLedId = 59;
pub const CorsairLedId_CLK_SlashAndQuestionMark: CorsairLedId = 60;
pub const CorsairLedId_CLK_LeftCtrl: CorsairLedId = 61;
pub const CorsairLedId_CLK_LeftGui: CorsairLedId = 62;
pub const CorsairLedId_CLK_LeftAlt: CorsairLedId = 63;
pub const CorsairLedId_CLK_Lang2: CorsairLedId = 64;
pub const CorsairLedId_CLK_Space: CorsairLedId = 65;
pub const CorsairLedId_CLK_Lang1: CorsairLedId = 66;
pub const CorsairLedId_CLK_International2: CorsairLedId = 67;
pub const CorsairLedId_CLK_RightAlt: CorsairLedId = 68;
pub const CorsairLedId_CLK_RightGui: CorsairLedId = 69;
pub const CorsairLedId_CLK_Application: CorsairLedId = 70;
pub const CorsairLedId_CLK_LedProgramming: CorsairLedId = 71;
pub const CorsairLedId_CLK_Brightness: CorsairLedId = 72;
pub const CorsairLedId_CLK_F12: CorsairLedId = 73;
pub const CorsairLedId_CLK_PrintScreen: CorsairLedId = 74;
pub const CorsairLedId_CLK_ScrollLock: CorsairLedId = 75;
pub const CorsairLedId_CLK_PauseBreak: CorsairLedId = 76;
pub const CorsairLedId_CLK_Insert: CorsairLedId = 77;
pub const CorsairLedId_CLK_Home: CorsairLedId = 78;
pub const CorsairLedId_CLK_PageUp: CorsairLedId = 79;
pub const CorsairLedId_CLK_BracketRight: CorsairLedId = 80;
pub const CorsairLedId_CLK_Backslash: CorsairLedId = 81;
pub const CorsairLedId_CLK_NonUsTilde: CorsairLedId = 82;
pub const CorsairLedId_CLK_Enter: CorsairLedId = 83;
pub const CorsairLedId_CLK_International1: CorsairLedId = 84;
pub const CorsairLedId_CLK_EqualsAndPlus: CorsairLedId = 85;
pub const CorsairLedId_CLK_International3: CorsairLedId = 86;
pub const CorsairLedId_CLK_Backspace: CorsairLedId = 87;
pub const CorsairLedId_CLK_Delete: CorsairLedId = 88;
pub const CorsairLedId_CLK_End: CorsairLedId = 89;
pub const CorsairLedId_CLK_PageDown: CorsairLedId = 90;
pub const CorsairLedId_CLK_RightShift: CorsairLedId = 91;
pub const CorsairLedId_CLK_RightCtrl: CorsairLedId = 92;
pub const CorsairLedId_CLK_UpArrow: CorsairLedId = 93;
pub const CorsairLedId_CLK_LeftArrow: CorsairLedId = 94;
pub const CorsairLedId_CLK_DownArrow: CorsairLedId = 95;
pub const CorsairLedId_CLK_RightArrow: CorsairLedId = 96;
pub const CorsairLedId_CLK_WinLock: CorsairLedId = 97;
pub const CorsairLedId_CLK_Mute: CorsairLedId = 98;
pub const CorsairLedId_CLK_Stop: CorsairLedId = 99;
pub const CorsairLedId_CLK_ScanPreviousTrack: CorsairLedId = 100;
pub const CorsairLedId_CLK_PlayPause: CorsairLedId = 101;
pub const CorsairLedId_CLK_ScanNextTrack: CorsairLedId = 102;
pub const CorsairLedId_CLK_NumLock: CorsairLedId = 103;
pub const CorsairLedId_CLK_KeypadSlash: CorsairLedId = 104;
pub const CorsairLedId_CLK_KeypadAsterisk: CorsairLedId = 105;
pub const CorsairLedId_CLK_KeypadMinus: CorsairLedId = 106;
pub const CorsairLedId_CLK_KeypadPlus: CorsairLedId = 107;
pub const CorsairLedId_CLK_KeypadEnter: CorsairLedId = 108;
pub const CorsairLedId_CLK_Keypad7: CorsairLedId = 109;
pub const CorsairLedId_CLK_Keypad8: CorsairLedId = 110;
pub const CorsairLedId_CLK_Keypad9: CorsairLedId = 111;
pub const CorsairLedId_CLK_KeypadComma: CorsairLedId = 112;
pub const CorsairLedId_CLK_Keypad4: CorsairLedId = 113;
pub const CorsairLedId_CLK_Keypad5: CorsairLedId = 114;
pub const CorsairLedId_CLK_Keypad6: CorsairLedId = 115;
pub const CorsairLedId_CLK_Keypad1: CorsairLedId = 116;
pub const CorsairLedId_CLK_Keypad2: CorsairLedId = 117;
pub const CorsairLedId_CLK_Keypad3: CorsairLedId = 118;
pub const CorsairLedId_CLK_Keypad0: CorsairLedId = 119;
pub const CorsairLedId_CLK_KeypadPeriodAndDelete: CorsairLedId = 120;
pub const CorsairLedId_CLK_G1: CorsairLedId = 121;
pub const CorsairLedId_CLK_G2: CorsairLedId = 122;
pub const CorsairLedId_CLK_G3: CorsairLedId = 123;
pub const CorsairLedId_CLK_G4: CorsairLedId = 124;
pub const CorsairLedId_CLK_G5: CorsairLedId = 125;
pub const CorsairLedId_CLK_G6: CorsairLedId = 126;
pub const CorsairLedId_CLK_G7: CorsairLedId = 127;
pub const CorsairLedId_CLK_G8: CorsairLedId = 128;
pub const CorsairLedId_CLK_G9: CorsairLedId = 129;
pub const CorsairLedId_CLK_G10: CorsairLedId = 130;
pub const CorsairLedId_CLK_VolumeUp: CorsairLedId = 131;
pub const CorsairLedId_CLK_VolumeDown: CorsairLedId = 132;
pub const CorsairLedId_CLK_MR: CorsairLedId = 133;
pub const CorsairLedId_CLK_M1: CorsairLedId = 134;
pub const CorsairLedId_CLK_M2: CorsairLedId = 135;
pub const CorsairLedId_CLK_M3: CorsairLedId = 136;
pub const CorsairLedId_CLK_G11: CorsairLedId = 137;
pub const CorsairLedId_CLK_G12: CorsairLedId = 138;
pub const CorsairLedId_CLK_G13: CorsairLedId = 139;
pub const CorsairLedId_CLK_G14: CorsairLedId = 140;
pub const CorsairLedId_CLK_G15: CorsairLedId = 141;
pub const CorsairLedId_CLK_G16: CorsairLedId = 142;
pub const CorsairLedId_CLK_G17: CorsairLedId = 143;
pub const CorsairLedId_CLK_G18: CorsairLedId = 144;
pub const CorsairLedId_CLK_International5: CorsairLedId = 145;
pub const CorsairLedId_CLK_International4: CorsairLedId = 146;
pub const CorsairLedId_CLK_Fn: CorsairLedId = 147;
pub const CorsairLedId_CLM_1: CorsairLedId = 148;
pub const CorsairLedId_CLM_2: CorsairLedId = 149;
pub const CorsairLedId_CLM_3: CorsairLedId = 150;
pub const CorsairLedId_CLM_4: CorsairLedId = 151;
pub const CorsairLedId_CLH_LeftLogo: CorsairLedId = 152;
pub const CorsairLedId_CLH_RightLogo: CorsairLedId = 153;
pub const CorsairLedId_CLK_Logo: CorsairLedId = 154;
pub const CorsairLedId_CLMM_Zone1: CorsairLedId = 155;
pub const CorsairLedId_CLMM_Zone2: CorsairLedId = 156;
pub const CorsairLedId_CLMM_Zone3: CorsairLedId = 157;
pub const CorsairLedId_CLMM_Zone4: CorsairLedId = 158;
pub const CorsairLedId_CLMM_Zone5: CorsairLedId = 159;
pub const CorsairLedId_CLMM_Zone6: CorsairLedId = 160;
pub const CorsairLedId_CLMM_Zone7: CorsairLedId = 161;
pub const CorsairLedId_CLMM_Zone8: CorsairLedId = 162;
pub const CorsairLedId_CLMM_Zone9: CorsairLedId = 163;
pub const CorsairLedId_CLMM_Zone10: CorsairLedId = 164;
pub const CorsairLedId_CLMM_Zone11: CorsairLedId = 165;
pub const CorsairLedId_CLMM_Zone12: CorsairLedId = 166;
pub const CorsairLedId_CLMM_Zone13: CorsairLedId = 167;
pub const CorsairLedId_CLMM_Zone14: CorsairLedId = 168;
pub const CorsairLedId_CLMM_Zone15: CorsairLedId = 169;
pub const CorsairLedId_CLKLP_Zone1: CorsairLedId = 170;
pub const CorsairLedId_CLKLP_Zone2: CorsairLedId = 171;
pub const CorsairLedId_CLKLP_Zone3: CorsairLedId = 172;
pub const CorsairLedId_CLKLP_Zone4: CorsairLedId = 173;
pub const CorsairLedId_CLKLP_Zone5: CorsairLedId = 174;
pub const CorsairLedId_CLKLP_Zone6: CorsairLedId = 175;
pub const CorsairLedId_CLKLP_Zone7: CorsairLedId = 176;
pub const CorsairLedId_CLKLP_Zone8: CorsairLedId = 177;
pub const CorsairLedId_CLKLP_Zone9: CorsairLedId = 178;
pub const CorsairLedId_CLKLP_Zone10: CorsairLedId = 179;
pub const CorsairLedId_CLKLP_Zone11: CorsairLedId = 180;
pub const CorsairLedId_CLKLP_Zone12: CorsairLedId = 181;
pub const CorsairLedId_CLKLP_Zone13: CorsairLedId = 182;
pub const CorsairLedId_CLKLP_Zone14: CorsairLedId = 183;
pub const CorsairLedId_CLKLP_Zone15: CorsairLedId = 184;
pub const CorsairLedId_CLKLP_Zone16: CorsairLedId = 185;
pub const CorsairLedId_CLKLP_Zone17: CorsairLedId = 186;
pub const CorsairLedId_CLKLP_Zone18: CorsairLedId = 187;
pub const CorsairLedId_CLKLP_Zone19: CorsairLedId = 188;
pub const CorsairLedId_CLM_5: CorsairLedId = 189;
pub const CorsairLedId_CLM_6: CorsairLedId = 190;
pub const CorsairLedId_CLHSS_Zone1: CorsairLedId = 191;
pub const CorsairLedId_CLHSS_Zone2: CorsairLedId = 192;
pub const CorsairLedId_CLHSS_Zone3: CorsairLedId = 193;
pub const CorsairLedId_CLHSS_Zone4: CorsairLedId = 194;
pub const CorsairLedId_CLHSS_Zone5: CorsairLedId = 195;
pub const CorsairLedId_CLHSS_Zone6: CorsairLedId = 196;
pub const CorsairLedId_CLHSS_Zone7: CorsairLedId = 197;
pub const CorsairLedId_CLHSS_Zone8: CorsairLedId = 198;
pub const CorsairLedId_CLHSS_Zone9: CorsairLedId = 199;
pub const CorsairLedId_CLD_C1_1: CorsairLedId = 200;
pub const CorsairLedId_CLD_C1_2: CorsairLedId = 201;
pub const CorsairLedId_CLD_C1_3: CorsairLedId = 202;
pub const CorsairLedId_CLD_C1_4: CorsairLedId = 203;
pub const CorsairLedId_CLD_C1_5: CorsairLedId = 204;
pub const CorsairLedId_CLD_C1_6: CorsairLedId = 205;
pub const CorsairLedId_CLD_C1_7: CorsairLedId = 206;
pub const CorsairLedId_CLD_C1_8: CorsairLedId = 207;
pub const CorsairLedId_CLD_C1_9: CorsairLedId = 208;
pub const CorsairLedId_CLD_C1_10: CorsairLedId = 209;
pub const CorsairLedId_CLD_C1_11: CorsairLedId = 210;
pub const CorsairLedId_CLD_C1_12: CorsairLedId = 211;
pub const CorsairLedId_CLD_C1_13: CorsairLedId = 212;
pub const CorsairLedId_CLD_C1_14: CorsairLedId = 213;
pub const CorsairLedId_CLD_C1_15: CorsairLedId = 214;
pub const CorsairLedId_CLD_C1_16: CorsairLedId = 215;
pub const CorsairLedId_CLD_C1_17: CorsairLedId = 216;
pub const CorsairLedId_CLD_C1_18: CorsairLedId = 217;
pub const CorsairLedId_CLD_C1_19: CorsairLedId = 218;
pub const CorsairLedId_CLD_C1_20: CorsairLedId = 219;
pub const CorsairLedId_CLD_C1_21: CorsairLedId = 220;
pub const CorsairLedId_CLD_C1_22: CorsairLedId = 221;
pub const CorsairLedId_CLD_C1_23: CorsairLedId = 222;
pub const CorsairLedId_CLD_C1_24: CorsairLedId = 223;
pub const CorsairLedId_CLD_C1_25: CorsairLedId = 224;
pub const CorsairLedId_CLD_C1_26: CorsairLedId = 225;
pub const CorsairLedId_CLD_C1_27: CorsairLedId = 226;
pub const CorsairLedId_CLD_C1_28: CorsairLedId = 227;
pub const CorsairLedId_CLD_C1_29: CorsairLedId = 228;
pub const CorsairLedId_CLD_C1_30: CorsairLedId = 229;
pub const CorsairLedId_CLD_C1_31: CorsairLedId = 230;
pub const CorsairLedId_CLD_C1_32: CorsairLedId = 231;
pub const CorsairLedId_CLD_C1_33: CorsairLedId = 232;
pub const CorsairLedId_CLD_C1_34: CorsairLedId = 233;
pub const CorsairLedId_CLD_C1_35: CorsairLedId = 234;
pub const CorsairLedId_CLD_C1_36: CorsairLedId = 235;
pub const CorsairLedId_CLD_C1_37: CorsairLedId = 236;
pub const CorsairLedId_CLD_C1_38: CorsairLedId = 237;
pub const CorsairLedId_CLD_C1_39: CorsairLedId = 238;
pub const CorsairLedId_CLD_C1_40: CorsairLedId = 239;
pub const CorsairLedId_CLD_C1_41: CorsairLedId = 240;
pub const CorsairLedId_CLD_C1_42: CorsairLedId = 241;
pub const CorsairLedId_CLD_C1_43: CorsairLedId = 242;
pub const CorsairLedId_CLD_C1_44: CorsairLedId = 243;
pub const CorsairLedId_CLD_C1_45: CorsairLedId = 244;
pub const CorsairLedId_CLD_C1_46: CorsairLedId = 245;
pub const CorsairLedId_CLD_C1_47: CorsairLedId = 246;
pub const CorsairLedId_CLD_C1_48: CorsairLedId = 247;
pub const CorsairLedId_CLD_C1_49: CorsairLedId = 248;
pub const CorsairLedId_CLD_C1_50: CorsairLedId = 249;
pub const CorsairLedId_CLD_C1_51: CorsairLedId = 250;
pub const CorsairLedId_CLD_C1_52: CorsairLedId = 251;
pub const CorsairLedId_CLD_C1_53: CorsairLedId = 252;
pub const CorsairLedId_CLD_C1_54: CorsairLedId = 253;
pub const CorsairLedId_CLD_C1_55: CorsairLedId = 254;
pub const CorsairLedId_CLD_C1_56: CorsairLedId = 255;
pub const CorsairLedId_CLD_C1_57: CorsairLedId = 256;
pub const CorsairLedId_CLD_C1_58: CorsairLedId = 257;
pub const CorsairLedId_CLD_C1_59: CorsairLedId = 258;
pub const CorsairLedId_CLD_C1_60: CorsairLedId = 259;
pub const CorsairLedId_CLD_C1_61: CorsairLedId = 260;
pub const CorsairLedId_CLD_C1_62: CorsairLedId = 261;
pub const CorsairLedId_CLD_C1_63: CorsairLedId = 262;
pub const CorsairLedId_CLD_C1_64: CorsairLedId = 263;
pub const CorsairLedId_CLD_C1_65: CorsairLedId = 264;
pub const CorsairLedId_CLD_C1_66: CorsairLedId = 265;
pub const CorsairLedId_CLD_C1_67: CorsairLedId = 266;
pub const CorsairLedId_CLD_C1_68: CorsairLedId = 267;
pub const CorsairLedId_CLD_C1_69: CorsairLedId = 268;
pub const CorsairLedId_CLD_C1_70: CorsairLedId = 269;
pub const CorsairLedId_CLD_C1_71: CorsairLedId = 270;
pub const CorsairLedId_CLD_C1_72: CorsairLedId = 271;
pub const CorsairLedId_CLD_C1_73: CorsairLedId = 272;
pub const CorsairLedId_CLD_C1_74: CorsairLedId = 273;
pub const CorsairLedId_CLD_C1_75: CorsairLedId = 274;
pub const CorsairLedId_CLD_C1_76: CorsairLedId = 275;
pub const CorsairLedId_CLD_C1_77: CorsairLedId = 276;
pub const CorsairLedId_CLD_C1_78: CorsairLedId = 277;
pub const CorsairLedId_CLD_C1_79: CorsairLedId = 278;
pub const CorsairLedId_CLD_C1_80: CorsairLedId = 279;
pub const CorsairLedId_CLD_C1_81: CorsairLedId = 280;
pub const CorsairLedId_CLD_C1_82: CorsairLedId = 281;
pub const CorsairLedId_CLD_C1_83: CorsairLedId = 282;
pub const CorsairLedId_CLD_C1_84: CorsairLedId = 283;
pub const CorsairLedId_CLD_C1_85: CorsairLedId = 284;
pub const CorsairLedId_CLD_C1_86: CorsairLedId = 285;
pub const CorsairLedId_CLD_C1_87: CorsairLedId = 286;
pub const CorsairLedId_CLD_C1_88: CorsairLedId = 287;
pub const CorsairLedId_CLD_C1_89: CorsairLedId = 288;
pub const CorsairLedId_CLD_C1_90: CorsairLedId = 289;
pub const CorsairLedId_CLD_C1_91: CorsairLedId = 290;
pub const CorsairLedId_CLD_C1_92: CorsairLedId = 291;
pub const CorsairLedId_CLD_C1_93: CorsairLedId = 292;
pub const CorsairLedId_CLD_C1_94: CorsairLedId = 293;
pub const CorsairLedId_CLD_C1_95: CorsairLedId = 294;
pub const CorsairLedId_CLD_C1_96: CorsairLedId = 295;
pub const CorsairLedId_CLD_C1_97: CorsairLedId = 296;
pub const CorsairLedId_CLD_C1_98: CorsairLedId = 297;
pub const CorsairLedId_CLD_C1_99: CorsairLedId = 298;
pub const CorsairLedId_CLD_C1_100: CorsairLedId = 299;
pub const CorsairLedId_CLD_C1_101: CorsairLedId = 300;
pub const CorsairLedId_CLD_C1_102: CorsairLedId = 301;
pub const CorsairLedId_CLD_C1_103: CorsairLedId = 302;
pub const CorsairLedId_CLD_C1_104: CorsairLedId = 303;
pub const CorsairLedId_CLD_C1_105: CorsairLedId = 304;
pub const CorsairLedId_CLD_C1_106: CorsairLedId = 305;
pub const CorsairLedId_CLD_C1_107: CorsairLedId = 306;
pub const CorsairLedId_CLD_C1_108: CorsairLedId = 307;
pub const CorsairLedId_CLD_C1_109: CorsairLedId = 308;
pub const CorsairLedId_CLD_C1_110: CorsairLedId = 309;
pub const CorsairLedId_CLD_C1_111: CorsairLedId = 310;
pub const CorsairLedId_CLD_C1_112: CorsairLedId = 311;
pub const CorsairLedId_CLD_C1_113: CorsairLedId = 312;
pub const CorsairLedId_CLD_C1_114: CorsairLedId = 313;
pub const CorsairLedId_CLD_C1_115: CorsairLedId = 314;
pub const CorsairLedId_CLD_C1_116: CorsairLedId = 315;
pub const CorsairLedId_CLD_C1_117: CorsairLedId = 316;
pub const CorsairLedId_CLD_C1_118: CorsairLedId = 317;
pub const CorsairLedId_CLD_C1_119: CorsairLedId = 318;
pub const CorsairLedId_CLD_C1_120: CorsairLedId = 319;
pub const CorsairLedId_CLD_C1_121: CorsairLedId = 320;
pub const CorsairLedId_CLD_C1_122: CorsairLedId = 321;
pub const CorsairLedId_CLD_C1_123: CorsairLedId = 322;
pub const CorsairLedId_CLD_C1_124: CorsairLedId = 323;
pub const CorsairLedId_CLD_C1_125: CorsairLedId = 324;
pub const CorsairLedId_CLD_C1_126: CorsairLedId = 325;
pub const CorsairLedId_CLD_C1_127: CorsairLedId = 326;
pub const CorsairLedId_CLD_C1_128: CorsairLedId = 327;
pub const CorsairLedId_CLD_C1_129: CorsairLedId = 328;
pub const CorsairLedId_CLD_C1_130: CorsairLedId = 329;
pub const CorsairLedId_CLD_C1_131: CorsairLedId = 330;
pub const CorsairLedId_CLD_C1_132: CorsairLedId = 331;
pub const CorsairLedId_CLD_C1_133: CorsairLedId = 332;
pub const CorsairLedId_CLD_C1_134: CorsairLedId = 333;
pub const CorsairLedId_CLD_C1_135: CorsairLedId = 334;
pub const CorsairLedId_CLD_C1_136: CorsairLedId = 335;
pub const CorsairLedId_CLD_C1_137: CorsairLedId = 336;
pub const CorsairLedId_CLD_C1_138: CorsairLedId = 337;
pub const CorsairLedId_CLD_C1_139: CorsairLedId = 338;
pub const CorsairLedId_CLD_C1_140: CorsairLedId = 339;
pub const CorsairLedId_CLD_C1_141: CorsairLedId = 340;
pub const CorsairLedId_CLD_C1_142: CorsairLedId = 341;
pub const CorsairLedId_CLD_C1_143: CorsairLedId = 342;
pub const CorsairLedId_CLD_C1_144: CorsairLedId = 343;
pub const CorsairLedId_CLD_C1_145: CorsairLedId = 344;
pub const CorsairLedId_CLD_C1_146: CorsairLedId = 345;
pub const CorsairLedId_CLD_C1_147: CorsairLedId = 346;
pub const CorsairLedId_CLD_C1_148: CorsairLedId = 347;
pub const CorsairLedId_CLD_C1_149: CorsairLedId = 348;
pub const CorsairLedId_CLD_C1_150: CorsairLedId = 349;
pub const CorsairLedId_CLD_C2_1: CorsairLedId = 350;
pub const CorsairLedId_CLD_C2_2: CorsairLedId = 351;
pub const CorsairLedId_CLD_C2_3: CorsairLedId = 352;
pub const CorsairLedId_CLD_C2_4: CorsairLedId = 353;
pub const CorsairLedId_CLD_C2_5: CorsairLedId = 354;
pub const CorsairLedId_CLD_C2_6: CorsairLedId = 355;
pub const CorsairLedId_CLD_C2_7: CorsairLedId = 356;
pub const CorsairLedId_CLD_C2_8: CorsairLedId = 357;
pub const CorsairLedId_CLD_C2_9: CorsairLedId = 358;
pub const CorsairLedId_CLD_C2_10: CorsairLedId = 359;
pub const CorsairLedId_CLD_C2_11: CorsairLedId = 360;
pub const CorsairLedId_CLD_C2_12: CorsairLedId = 361;
pub const CorsairLedId_CLD_C2_13: CorsairLedId = 362;
pub const CorsairLedId_CLD_C2_14: CorsairLedId = 363;
pub const CorsairLedId_CLD_C2_15: CorsairLedId = 364;
pub const CorsairLedId_CLD_C2_16: CorsairLedId = 365;
pub const CorsairLedId_CLD_C2_17: CorsairLedId = 366;
pub const CorsairLedId_CLD_C2_18: CorsairLedId = 367;
pub const CorsairLedId_CLD_C2_19: CorsairLedId = 368;
pub const CorsairLedId_CLD_C2_20: CorsairLedId = 369;
pub const CorsairLedId_CLD_C2_21: CorsairLedId = 370;
pub const CorsairLedId_CLD_C2_22: CorsairLedId = 371;
pub const CorsairLedId_CLD_C2_23: CorsairLedId = 372;
pub const CorsairLedId_CLD_C2_24: CorsairLedId = 373;
pub const CorsairLedId_CLD_C2_25: CorsairLedId = 374;
pub const CorsairLedId_CLD_C2_26: CorsairLedId = 375;
pub const CorsairLedId_CLD_C2_27: CorsairLedId = 376;
pub const CorsairLedId_CLD_C2_28: CorsairLedId = 377;
pub const CorsairLedId_CLD_C2_29: CorsairLedId = 378;
pub const CorsairLedId_CLD_C2_30: CorsairLedId = 379;
pub const CorsairLedId_CLD_C2_31: CorsairLedId = 380;
pub const CorsairLedId_CLD_C2_32: CorsairLedId = 381;
pub const CorsairLedId_CLD_C2_33: CorsairLedId = 382;
pub const CorsairLedId_CLD_C2_34: CorsairLedId = 383;
pub const CorsairLedId_CLD_C2_35: CorsairLedId = 384;
pub const CorsairLedId_CLD_C2_36: CorsairLedId = 385;
pub const CorsairLedId_CLD_C2_37: CorsairLedId = 386;
pub const CorsairLedId_CLD_C2_38: CorsairLedId = 387;
pub const CorsairLedId_CLD_C2_39: CorsairLedId = 388;
pub const CorsairLedId_CLD_C2_40: CorsairLedId = 389;
pub const CorsairLedId_CLD_C2_41: CorsairLedId = 390;
pub const CorsairLedId_CLD_C2_42: CorsairLedId = 391;
pub const CorsairLedId_CLD_C2_43: CorsairLedId = 392;
pub const CorsairLedId_CLD_C2_44: CorsairLedId = 393;
pub const CorsairLedId_CLD_C2_45: CorsairLedId = 394;
pub const CorsairLedId_CLD_C2_46: CorsairLedId = 395;
pub const CorsairLedId_CLD_C2_47: CorsairLedId = 396;
pub const CorsairLedId_CLD_C2_48: CorsairLedId = 397;
pub const CorsairLedId_CLD_C2_49: CorsairLedId = 398;
pub const CorsairLedId_CLD_C2_50: CorsairLedId = 399;
pub const CorsairLedId_CLD_C2_51: CorsairLedId = 400;
pub const CorsairLedId_CLD_C2_52: CorsairLedId = 401;
pub const CorsairLedId_CLD_C2_53: CorsairLedId = 402;
pub const CorsairLedId_CLD_C2_54: CorsairLedId = 403;
pub const CorsairLedId_CLD_C2_55: CorsairLedId = 404;
pub const CorsairLedId_CLD_C2_56: CorsairLedId = 405;
pub const CorsairLedId_CLD_C2_57: CorsairLedId = 406;
pub const CorsairLedId_CLD_C2_58: CorsairLedId = 407;
pub const CorsairLedId_CLD_C2_59: CorsairLedId = 408;
pub const CorsairLedId_CLD_C2_60: CorsairLedId = 409;
pub const CorsairLedId_CLD_C2_61: CorsairLedId = 410;
pub const CorsairLedId_CLD_C2_62: CorsairLedId = 411;
pub const CorsairLedId_CLD_C2_63: CorsairLedId = 412;
pub const CorsairLedId_CLD_C2_64: CorsairLedId = 413;
pub const CorsairLedId_CLD_C2_65: CorsairLedId = 414;
pub const CorsairLedId_CLD_C2_66: CorsairLedId = 415;
pub const CorsairLedId_CLD_C2_67: CorsairLedId = 416;
pub const CorsairLedId_CLD_C2_68: CorsairLedId = 417;
pub const CorsairLedId_CLD_C2_69: CorsairLedId = 418;
pub const CorsairLedId_CLD_C2_70: CorsairLedId = 419;
pub const CorsairLedId_CLD_C2_71: CorsairLedId = 420;
pub const CorsairLedId_CLD_C2_72: CorsairLedId = 421;
pub const CorsairLedId_CLD_C2_73: CorsairLedId = 422;
pub const CorsairLedId_CLD_C2_74: CorsairLedId = 423;
pub const CorsairLedId_CLD_C2_75: CorsairLedId = 424;
pub const CorsairLedId_CLD_C2_76: CorsairLedId = 425;
pub const CorsairLedId_CLD_C2_77: CorsairLedId = 426;
pub const CorsairLedId_CLD_C2_78: CorsairLedId = 427;
pub const CorsairLedId_CLD_C2_79: CorsairLedId = 428;
pub const CorsairLedId_CLD_C2_80: CorsairLedId = 429;
pub const CorsairLedId_CLD_C2_81: CorsairLedId = 430;
pub const CorsairLedId_CLD_C2_82: CorsairLedId = 431;
pub const CorsairLedId_CLD_C2_83: CorsairLedId = 432;
pub const CorsairLedId_CLD_C2_84: CorsairLedId = 433;
pub const CorsairLedId_CLD_C2_85: CorsairLedId = 434;
pub const CorsairLedId_CLD_C2_86: CorsairLedId = 435;
pub const CorsairLedId_CLD_C2_87: CorsairLedId = 436;
pub const CorsairLedId_CLD_C2_88: CorsairLedId = 437;
pub const CorsairLedId_CLD_C2_89: CorsairLedId = 438;
pub const CorsairLedId_CLD_C2_90: CorsairLedId = 439;
pub const CorsairLedId_CLD_C2_91: CorsairLedId = 440;
pub const CorsairLedId_CLD_C2_92: CorsairLedId = 441;
pub const CorsairLedId_CLD_C2_93: CorsairLedId = 442;
pub const CorsairLedId_CLD_C2_94: CorsairLedId = 443;
pub const CorsairLedId_CLD_C2_95: CorsairLedId = 444;
pub const CorsairLedId_CLD_C2_96: CorsairLedId = 445;
pub const CorsairLedId_CLD_C2_97: CorsairLedId = 446;
pub const CorsairLedId_CLD_C2_98: CorsairLedId = 447;
pub const CorsairLedId_CLD_C2_99: CorsairLedId = 448;
pub const CorsairLedId_CLD_C2_100: CorsairLedId = 449;
pub const CorsairLedId_CLD_C2_101: CorsairLedId = 450;
pub const CorsairLedId_CLD_C2_102: CorsairLedId = 451;
pub const CorsairLedId_CLD_C2_103: CorsairLedId = 452;
pub const CorsairLedId_CLD_C2_104: CorsairLedId = 453;
pub const CorsairLedId_CLD_C2_105: CorsairLedId = 454;
pub const CorsairLedId_CLD_C2_106: CorsairLedId = 455;
pub const CorsairLedId_CLD_C2_107: CorsairLedId = 456;
pub const CorsairLedId_CLD_C2_108: CorsairLedId = 457;
pub const CorsairLedId_CLD_C2_109: CorsairLedId = 458;
pub const CorsairLedId_CLD_C2_110: CorsairLedId = 459;
pub const CorsairLedId_CLD_C2_111: CorsairLedId = 460;
pub const CorsairLedId_CLD_C2_112: CorsairLedId = 461;
pub const CorsairLedId_CLD_C2_113: CorsairLedId = 462;
pub const CorsairLedId_CLD_C2_114: CorsairLedId = 463;
pub const CorsairLedId_CLD_C2_115: CorsairLedId = 464;
pub const CorsairLedId_CLD_C2_116: CorsairLedId = 465;
pub const CorsairLedId_CLD_C2_117: CorsairLedId = 466;
pub const CorsairLedId_CLD_C2_118: CorsairLedId = 467;
pub const CorsairLedId_CLD_C2_119: CorsairLedId = 468;
pub const CorsairLedId_CLD_C2_120: CorsairLedId = 469;
pub const CorsairLedId_CLD_C2_121: CorsairLedId = 470;
pub const CorsairLedId_CLD_C2_122: CorsairLedId = 471;
pub const CorsairLedId_CLD_C2_123: CorsairLedId = 472;
pub const CorsairLedId_CLD_C2_124: CorsairLedId = 473;
pub const CorsairLedId_CLD_C2_125: CorsairLedId = 474;
pub const CorsairLedId_CLD_C2_126: CorsairLedId = 475;
pub const CorsairLedId_CLD_C2_127: CorsairLedId = 476;
pub const CorsairLedId_CLD_C2_128: CorsairLedId = 477;
pub const CorsairLedId_CLD_C2_129: CorsairLedId = 478;
pub const CorsairLedId_CLD_C2_130: CorsairLedId = 479;
pub const CorsairLedId_CLD_C2_131: CorsairLedId = 480;
pub const CorsairLedId_CLD_C2_132: CorsairLedId = 481;
pub const CorsairLedId_CLD_C2_133: CorsairLedId = 482;
pub const CorsairLedId_CLD_C2_134: CorsairLedId = 483;
pub const CorsairLedId_CLD_C2_135: CorsairLedId = 484;
pub const CorsairLedId_CLD_C2_136: CorsairLedId = 485;
pub const CorsairLedId_CLD_C2_137: CorsairLedId = 486;
pub const CorsairLedId_CLD_C2_138: CorsairLedId = 487;
pub const CorsairLedId_CLD_C2_139: CorsairLedId = 488;
pub const CorsairLedId_CLD_C2_140: CorsairLedId = 489;
pub const CorsairLedId_CLD_C2_141: CorsairLedId = 490;
pub const CorsairLedId_CLD_C2_142: CorsairLedId = 491;
pub const CorsairLedId_CLD_C2_143: CorsairLedId = 492;
pub const CorsairLedId_CLD_C2_144: CorsairLedId = 493;
pub const CorsairLedId_CLD_C2_145: CorsairLedId = 494;
pub const CorsairLedId_CLD_C2_146: CorsairLedId = 495;
pub const CorsairLedId_CLD_C2_147: CorsairLedId = 496;
pub const CorsairLedId_CLD_C2_148: CorsairLedId = 497;
pub const CorsairLedId_CLD_C2_149: CorsairLedId = 498;
pub const CorsairLedId_CLD_C2_150: CorsairLedId = 499;
pub const CorsairLedId_CLI_Oem1: CorsairLedId = 500;
pub const CorsairLedId_CLI_Oem2: CorsairLedId = 501;
pub const CorsairLedId_CLI_Oem3: CorsairLedId = 502;
pub const CorsairLedId_CLI_Oem4: CorsairLedId = 503;
pub const CorsairLedId_CLI_Oem5: CorsairLedId = 504;
pub const CorsairLedId_CLI_Oem6: CorsairLedId = 505;
pub const CorsairLedId_CLI_Oem7: CorsairLedId = 506;
pub const CorsairLedId_CLI_Oem8: CorsairLedId = 507;
pub const CorsairLedId_CLI_Oem9: CorsairLedId = 508;
pub const CorsairLedId_CLI_Oem10: CorsairLedId = 509;
pub const CorsairLedId_CLI_Oem11: CorsairLedId = 510;
pub const CorsairLedId_CLI_Oem12: CorsairLedId = 511;
pub const CorsairLedId_CLI_Oem13: CorsairLedId = 512;
pub const CorsairLedId_CLI_Oem14: CorsairLedId = 513;
pub const CorsairLedId_CLI_Oem15: CorsairLedId = 514;
pub const CorsairLedId_CLI_Oem16: CorsairLedId = 515;
pub const CorsairLedId_CLI_Oem17: CorsairLedId = 516;
pub const CorsairLedId_CLI_Oem18: CorsairLedId = 517;
pub const CorsairLedId_CLI_Oem19: CorsairLedId = 518;
pub const CorsairLedId_CLI_Oem20: CorsairLedId = 519;
pub const CorsairLedId_CLI_Oem21: CorsairLedId = 520;
pub const CorsairLedId_CLI_Oem22: CorsairLedId = 521;
pub const CorsairLedId_CLI_Oem23: CorsairLedId = 522;
pub const CorsairLedId_CLI_Oem24: CorsairLedId = 523;
pub const CorsairLedId_CLI_Oem25: CorsairLedId = 524;
pub const CorsairLedId_CLI_Oem26: CorsairLedId = 525;
pub const CorsairLedId_CLI_Oem27: CorsairLedId = 526;
pub const CorsairLedId_CLI_Oem28: CorsairLedId = 527;
pub const CorsairLedId_CLI_Oem29: CorsairLedId = 528;
pub const CorsairLedId_CLI_Oem30: CorsairLedId = 529;
pub const CorsairLedId_CLI_Oem31: CorsairLedId = 530;
pub const CorsairLedId_CLI_Oem32: CorsairLedId = 531;
pub const CorsairLedId_CLI_Oem33: CorsairLedId = 532;
pub const CorsairLedId_CLI_Oem34: CorsairLedId = 533;
pub const CorsairLedId_CLI_Oem35: CorsairLedId = 534;
pub const CorsairLedId_CLI_Oem36: CorsairLedId = 535;
pub const CorsairLedId_CLI_Oem37: CorsairLedId = 536;
pub const CorsairLedId_CLI_Oem38: CorsairLedId = 537;
pub const CorsairLedId_CLI_Oem39: CorsairLedId = 538;
pub const CorsairLedId_CLI_Oem40: CorsairLedId = 539;
pub const CorsairLedId_CLI_Oem41: CorsairLedId = 540;
pub const CorsairLedId_CLI_Oem42: CorsairLedId = 541;
pub const CorsairLedId_CLI_Oem43: CorsairLedId = 542;
pub const CorsairLedId_CLI_Oem44: CorsairLedId = 543;
pub const CorsairLedId_CLI_Oem45: CorsairLedId = 544;
pub const CorsairLedId_CLI_Oem46: CorsairLedId = 545;
pub const CorsairLedId_CLI_Oem47: CorsairLedId = 546;
pub const CorsairLedId_CLI_Oem48: CorsairLedId = 547;
pub const CorsairLedId_CLI_Oem49: CorsairLedId = 548;
pub const CorsairLedId_CLI_Oem50: CorsairLedId = 549;
pub const CorsairLedId_CLI_Oem51: CorsairLedId = 550;
pub const CorsairLedId_CLI_Oem52: CorsairLedId = 551;
pub const CorsairLedId_CLI_Oem53: CorsairLedId = 552;
pub const CorsairLedId_CLI_Oem54: CorsairLedId = 553;
pub const CorsairLedId_CLI_Oem55: CorsairLedId = 554;
pub const CorsairLedId_CLI_Oem56: CorsairLedId = 555;
pub const CorsairLedId_CLI_Oem57: CorsairLedId = 556;
pub const CorsairLedId_CLI_Oem58: CorsairLedId = 557;
pub const CorsairLedId_CLI_Oem59: CorsairLedId = 558;
pub const CorsairLedId_CLI_Oem60: CorsairLedId = 559;
pub const CorsairLedId_CLI_Oem61: CorsairLedId = 560;
pub const CorsairLedId_CLI_Oem62: CorsairLedId = 561;
pub const CorsairLedId_CLI_Oem63: CorsairLedId = 562;
pub const CorsairLedId_CLI_Oem64: CorsairLedId = 563;
pub const CorsairLedId_CLI_Oem65: CorsairLedId = 564;
pub const CorsairLedId_CLI_Oem66: CorsairLedId = 565;
pub const CorsairLedId_CLI_Oem67: CorsairLedId = 566;
pub const CorsairLedId_CLI_Oem68: CorsairLedId = 567;
pub const CorsairLedId_CLI_Oem69: CorsairLedId = 568;
pub const CorsairLedId_CLI_Oem70: CorsairLedId = 569;
pub const CorsairLedId_CLI_Oem71: CorsairLedId = 570;
pub const CorsairLedId_CLI_Oem72: CorsairLedId = 571;
pub const CorsairLedId_CLI_Oem73: CorsairLedId = 572;
pub const CorsairLedId_CLI_Oem74: CorsairLedId = 573;
pub const CorsairLedId_CLI_Oem75: CorsairLedId = 574;
pub const CorsairLedId_CLI_Oem76: CorsairLedId = 575;
pub const CorsairLedId_CLI_Oem77: CorsairLedId = 576;
pub const CorsairLedId_CLI_Oem78: CorsairLedId = 577;
pub const CorsairLedId_CLI_Oem79: CorsairLedId = 578;
pub const CorsairLedId_CLI_Oem80: CorsairLedId = 579;
pub const CorsairLedId_CLI_Oem81: CorsairLedId = 580;
pub const CorsairLedId_CLI_Oem82: CorsairLedId = 581;
pub const CorsairLedId_CLI_Oem83: CorsairLedId = 582;
pub const CorsairLedId_CLI_Oem84: CorsairLedId = 583;
pub const CorsairLedId_CLI_Oem85: CorsairLedId = 584;
pub const CorsairLedId_CLI_Oem86: CorsairLedId = 585;
pub const CorsairLedId_CLI_Oem87: CorsairLedId = 586;
pub const CorsairLedId_CLI_Oem88: CorsairLedId = 587;
pub const CorsairLedId_CLI_Oem89: CorsairLedId = 588;
pub const CorsairLedId_CLI_Oem90: CorsairLedId = 589;
pub const CorsairLedId_CLI_Oem91: CorsairLedId = 590;
pub const CorsairLedId_CLI_Oem92: CorsairLedId = 591;
pub const CorsairLedId_CLI_Oem93: CorsairLedId = 592;
pub const CorsairLedId_CLI_Oem94: CorsairLedId = 593;
pub const CorsairLedId_CLI_Oem95: CorsairLedId = 594;
pub const CorsairLedId_CLI_Oem96: CorsairLedId = 595;
pub const CorsairLedId_CLI_Oem97: CorsairLedId = 596;
pub const CorsairLedId_CLI_Oem98: CorsairLedId = 597;
pub const CorsairLedId_CLI_Oem99: CorsairLedId = 598;
pub const CorsairLedId_CLI_Oem100: CorsairLedId = 599;
pub const CorsairLedId_CLDRAM_1: CorsairLedId = 600;
pub const CorsairLedId_CLDRAM_2: CorsairLedId = 601;
pub const CorsairLedId_CLDRAM_3: CorsairLedId = 602;
pub const CorsairLedId_CLDRAM_4: CorsairLedId = 603;
pub const CorsairLedId_CLDRAM_5: CorsairLedId = 604;
pub const CorsairLedId_CLDRAM_6: CorsairLedId = 605;
pub const CorsairLedId_CLDRAM_7: CorsairLedId = 606;
pub const CorsairLedId_CLDRAM_8: CorsairLedId = 607;
pub const CorsairLedId_CLDRAM_9: CorsairLedId = 608;
pub const CorsairLedId_CLDRAM_10: CorsairLedId = 609;
pub const CorsairLedId_CLDRAM_11: CorsairLedId = 610;
pub const CorsairLedId_CLDRAM_12: CorsairLedId = 611;
pub const CorsairLedId_CLD_C3_1: CorsairLedId = 612;
pub const CorsairLedId_CLD_C3_2: CorsairLedId = 613;
pub const CorsairLedId_CLD_C3_3: CorsairLedId = 614;
pub const CorsairLedId_CLD_C3_4: CorsairLedId = 615;
pub const CorsairLedId_CLD_C3_5: CorsairLedId = 616;
pub const CorsairLedId_CLD_C3_6: CorsairLedId = 617;
pub const CorsairLedId_CLD_C3_7: CorsairLedId = 618;
pub const CorsairLedId_CLD_C3_8: CorsairLedId = 619;
pub const CorsairLedId_CLD_C3_9: CorsairLedId = 620;
pub const CorsairLedId_CLD_C3_10: CorsairLedId = 621;
pub const CorsairLedId_CLD_C3_11: CorsairLedId = 622;
pub const CorsairLedId_CLD_C3_12: CorsairLedId = 623;
pub const CorsairLedId_CLD_C3_13: CorsairLedId = 624;
pub const CorsairLedId_CLD_C3_14: CorsairLedId = 625;
pub const CorsairLedId_CLD_C3_15: CorsairLedId = 626;
pub const CorsairLedId_CLD_C3_16: CorsairLedId = 627;
pub const CorsairLedId_CLD_C3_17: CorsairLedId = 628;
pub const CorsairLedId_CLD_C3_18: CorsairLedId = 629;
pub const CorsairLedId_CLD_C3_19: CorsairLedId = 630;
pub const CorsairLedId_CLD_C3_20: CorsairLedId = 631;
pub const CorsairLedId_CLD_C3_21: CorsairLedId = 632;
pub const CorsairLedId_CLD_C3_22: CorsairLedId = 633;
pub const CorsairLedId_CLD_C3_23: CorsairLedId = 634;
pub const CorsairLedId_CLD_C3_24: CorsairLedId = 635;
pub const CorsairLedId_CLD_C3_25: CorsairLedId = 636;
pub const CorsairLedId_CLD_C3_26: CorsairLedId = 637;
pub const CorsairLedId_CLD_C3_27: CorsairLedId = 638;
pub const CorsairLedId_CLD_C3_28: CorsairLedId = 639;
pub const CorsairLedId_CLD_C3_29: CorsairLedId = 640;
pub const CorsairLedId_CLD_C3_30: CorsairLedId = 641;
pub const CorsairLedId_CLD_C3_31: CorsairLedId = 642;
pub const CorsairLedId_CLD_C3_32: CorsairLedId = 643;
pub const CorsairLedId_CLD_C3_33: CorsairLedId = 644;
pub const CorsairLedId_CLD_C3_34: CorsairLedId = 645;
pub const CorsairLedId_CLD_C3_35: CorsairLedId = 646;
pub const CorsairLedId_CLD_C3_36: CorsairLedId = 647;
pub const CorsairLedId_CLD_C3_37: CorsairLedId = 648;
pub const CorsairLedId_CLD_C3_38: CorsairLedId = 649;
pub const CorsairLedId_CLD_C3_39: CorsairLedId = 650;
pub const CorsairLedId_CLD_C3_40: CorsairLedId = 651;
pub const CorsairLedId_CLD_C3_41: CorsairLedId = 652;
pub const CorsairLedId_CLD_C3_42: CorsairLedId = 653;
pub const CorsairLedId_CLD_C3_43: CorsairLedId = 654;
pub const CorsairLedId_CLD_C3_44: CorsairLedId = 655;
pub const CorsairLedId_CLD_C3_45: CorsairLedId = 656;
pub const CorsairLedId_CLD_C3_46: CorsairLedId = 657;
pub const CorsairLedId_CLD_C3_47: CorsairLedId = 658;
pub const CorsairLedId_CLD_C3_48: CorsairLedId = 659;
pub const CorsairLedId_CLD_C3_49: CorsairLedId = 660;
pub const CorsairLedId_CLD_C3_50: CorsairLedId = 661;
pub const CorsairLedId_CLD_C3_51: CorsairLedId = 662;
pub const CorsairLedId_CLD_C3_52: CorsairLedId = 663;
pub const CorsairLedId_CLD_C3_53: CorsairLedId = 664;
pub const CorsairLedId_CLD_C3_54: CorsairLedId = 665;
pub const CorsairLedId_CLD_C3_55: CorsairLedId = 666;
pub const CorsairLedId_CLD_C3_56: CorsairLedId = 667;
pub const CorsairLedId_CLD_C3_57: CorsairLedId = 668;
pub const CorsairLedId_CLD_C3_58: CorsairLedId = 669;
pub const CorsairLedId_CLD_C3_59: CorsairLedId = 670;
pub const CorsairLedId_CLD_C3_60: CorsairLedId = 671;
pub const CorsairLedId_CLD_C3_61: CorsairLedId = 672;
pub const CorsairLedId_CLD_C3_62: CorsairLedId = 673;
pub const CorsairLedId_CLD_C3_63: CorsairLedId = 674;
pub const CorsairLedId_CLD_C3_64: CorsairLedId = 675;
pub const CorsairLedId_CLD_C3_65: CorsairLedId = 676;
pub const CorsairLedId_CLD_C3_66: CorsairLedId = 677;
pub const CorsairLedId_CLD_C3_67: CorsairLedId = 678;
pub const CorsairLedId_CLD_C3_68: CorsairLedId = 679;
pub const CorsairLedId_CLD_C3_69: CorsairLedId = 680;
pub const CorsairLedId_CLD_C3_70: CorsairLedId = 681;
pub const CorsairLedId_CLD_C3_71: CorsairLedId = 682;
pub const CorsairLedId_CLD_C3_72: CorsairLedId = 683;
pub const CorsairLedId_CLD_C3_73: CorsairLedId = 684;
pub const CorsairLedId_CLD_C3_74: CorsairLedId = 685;
pub const CorsairLedId_CLD_C3_75: CorsairLedId = 686;
pub const CorsairLedId_CLD_C3_76: CorsairLedId = 687;
pub const CorsairLedId_CLD_C3_77: CorsairLedId = 688;
pub const CorsairLedId_CLD_C3_78: CorsairLedId = 689;
pub const CorsairLedId_CLD_C3_79: CorsairLedId = 690;
pub const CorsairLedId_CLD_C3_80: CorsairLedId = 691;
pub const CorsairLedId_CLD_C3_81: CorsairLedId = 692;
pub const CorsairLedId_CLD_C3_82: CorsairLedId = 693;
pub const CorsairLedId_CLD_C3_83: CorsairLedId = 694;
pub const CorsairLedId_CLD_C3_84: CorsairLedId = 695;
pub const CorsairLedId_CLD_C3_85: CorsairLedId = 696;
pub const CorsairLedId_CLD_C3_86: CorsairLedId = 697;
pub const CorsairLedId_CLD_C3_87: CorsairLedId = 698;
pub const CorsairLedId_CLD_C3_88: CorsairLedId = 699;
pub const CorsairLedId_CLD_C3_89: CorsairLedId = 700;
pub const CorsairLedId_CLD_C3_90: CorsairLedId = 701;
pub const CorsairLedId_CLD_C3_91: CorsairLedId = 702;
pub const CorsairLedId_CLD_C3_92: CorsairLedId = 703;
pub const CorsairLedId_CLD_C3_93: CorsairLedId = 704;
pub const CorsairLedId_CLD_C3_94: CorsairLedId = 705;
pub const CorsairLedId_CLD_C3_95: CorsairLedId = 706;
pub const CorsairLedId_CLD_C3_96: CorsairLedId = 707;
pub const CorsairLedId_CLD_C3_97: CorsairLedId = 708;
pub const CorsairLedId_CLD_C3_98: CorsairLedId = 709;
pub const CorsairLedId_CLD_C3_99: CorsairLedId = 710;
pub const CorsairLedId_CLD_C3_100: CorsairLedId = 711;
pub const CorsairLedId_CLD_C3_101: CorsairLedId = 712;
pub const CorsairLedId_CLD_C3_102: CorsairLedId = 713;
pub const CorsairLedId_CLD_C3_103: CorsairLedId = 714;
pub const CorsairLedId_CLD_C3_104: CorsairLedId = 715;
pub const CorsairLedId_CLD_C3_105: CorsairLedId = 716;
pub const CorsairLedId_CLD_C3_106: CorsairLedId = 717;
pub const CorsairLedId_CLD_C3_107: CorsairLedId = 718;
pub const CorsairLedId_CLD_C3_108: CorsairLedId = 719;
pub const CorsairLedId_CLD_C3_109: CorsairLedId = 720;
pub const CorsairLedId_CLD_C3_110: CorsairLedId = 721;
pub const CorsairLedId_CLD_C3_111: CorsairLedId = 722;
pub const CorsairLedId_CLD_C3_112: CorsairLedId = 723;
pub const CorsairLedId_CLD_C3_113: CorsairLedId = 724;
pub const CorsairLedId_CLD_C3_114: CorsairLedId = 725;
pub const CorsairLedId_CLD_C3_115: CorsairLedId = 726;
pub const CorsairLedId_CLD_C3_116: CorsairLedId = 727;
pub const CorsairLedId_CLD_C3_117: CorsairLedId = 728;
pub const CorsairLedId_CLD_C3_118: CorsairLedId = 729;
pub const CorsairLedId_CLD_C3_119: CorsairLedId = 730;
pub const CorsairLedId_CLD_C3_120: CorsairLedId = 731;
pub const CorsairLedId_CLD_C3_121: CorsairLedId = 732;
pub const CorsairLedId_CLD_C3_122: CorsairLedId = 733;
pub const CorsairLedId_CLD_C3_123: CorsairLedId = 734;
pub const CorsairLedId_CLD_C3_124: CorsairLedId = 735;
pub const CorsairLedId_CLD_C3_125: CorsairLedId = 736;
pub const CorsairLedId_CLD_C3_126: CorsairLedId = 737;
pub const CorsairLedId_CLD_C3_127: CorsairLedId = 738;
pub const CorsairLedId_CLD_C3_128: CorsairLedId = 739;
pub const CorsairLedId_CLD_C3_129: CorsairLedId = 740;
pub const CorsairLedId_CLD_C3_130: CorsairLedId = 741;
pub const CorsairLedId_CLD_C3_131: CorsairLedId = 742;
pub const CorsairLedId_CLD_C3_132: CorsairLedId = 743;
pub const CorsairLedId_CLD_C3_133: CorsairLedId = 744;
pub const CorsairLedId_CLD_C3_134: CorsairLedId = 745;
pub const CorsairLedId_CLD_C3_135: CorsairLedId = 746;
pub const CorsairLedId_CLD_C3_136: CorsairLedId = 747;
pub const CorsairLedId_CLD_C3_137: CorsairLedId = 748;
pub const CorsairLedId_CLD_C3_138: CorsairLedId = 749;
pub const CorsairLedId_CLD_C3_139: CorsairLedId = 750;
pub const CorsairLedId_CLD_C3_140: CorsairLedId = 751;
pub const CorsairLedId_CLD_C3_141: CorsairLedId = 752;
pub const CorsairLedId_CLD_C3_142: CorsairLedId = 753;
pub const CorsairLedId_CLD_C3_143: CorsairLedId = 754;
pub const CorsairLedId_CLD_C3_144: CorsairLedId = 755;
pub const CorsairLedId_CLD_C3_145: CorsairLedId = 756;
pub const CorsairLedId_CLD_C3_146: CorsairLedId = 757;
pub const CorsairLedId_CLD_C3_147: CorsairLedId = 758;
pub const CorsairLedId_CLD_C3_148: CorsairLedId = 759;
pub const CorsairLedId_CLD_C3_149: CorsairLedId = 760;
pub const CorsairLedId_CLD_C3_150: CorsairLedId = 761;
pub const CorsairLedId_CLLC_C1_1: CorsairLedId = 762;
pub const CorsairLedId_CLLC_C1_2: CorsairLedId = 763;
pub const CorsairLedId_CLLC_C1_3: CorsairLedId = 764;
pub const CorsairLedId_CLLC_C1_4: CorsairLedId = 765;
pub const CorsairLedId_CLLC_C1_5: CorsairLedId = 766;
pub const CorsairLedId_CLLC_C1_6: CorsairLedId = 767;
pub const CorsairLedId_CLLC_C1_7: CorsairLedId = 768;
pub const CorsairLedId_CLLC_C1_8: CorsairLedId = 769;
pub const CorsairLedId_CLLC_C1_9: CorsairLedId = 770;
pub const CorsairLedId_CLLC_C1_10: CorsairLedId = 771;
pub const CorsairLedId_CLLC_C1_11: CorsairLedId = 772;
pub const CorsairLedId_CLLC_C1_12: CorsairLedId = 773;
pub const CorsairLedId_CLLC_C1_13: CorsairLedId = 774;
pub const CorsairLedId_CLLC_C1_14: CorsairLedId = 775;
pub const CorsairLedId_CLLC_C1_15: CorsairLedId = 776;
pub const CorsairLedId_CLLC_C1_16: CorsairLedId = 777;
pub const CorsairLedId_CLLC_C1_17: CorsairLedId = 778;
pub const CorsairLedId_CLLC_C1_18: CorsairLedId = 779;
pub const CorsairLedId_CLLC_C1_19: CorsairLedId = 780;
pub const CorsairLedId_CLLC_C1_20: CorsairLedId = 781;
pub const CorsairLedId_CLLC_C1_21: CorsairLedId = 782;
pub const CorsairLedId_CLLC_C1_22: CorsairLedId = 783;
pub const CorsairLedId_CLLC_C1_23: CorsairLedId = 784;
pub const CorsairLedId_CLLC_C1_24: CorsairLedId = 785;
pub const CorsairLedId_CLLC_C1_25: CorsairLedId = 786;
pub const CorsairLedId_CLLC_C1_26: CorsairLedId = 787;
pub const CorsairLedId_CLLC_C1_27: CorsairLedId = 788;
pub const CorsairLedId_CLLC_C1_28: CorsairLedId = 789;
pub const CorsairLedId_CLLC_C1_29: CorsairLedId = 790;
pub const CorsairLedId_CLLC_C1_30: CorsairLedId = 791;
pub const CorsairLedId_CLLC_C1_31: CorsairLedId = 792;
pub const CorsairLedId_CLLC_C1_32: CorsairLedId = 793;
pub const CorsairLedId_CLLC_C1_33: CorsairLedId = 794;
pub const CorsairLedId_CLLC_C1_34: CorsairLedId = 795;
pub const CorsairLedId_CLLC_C1_35: CorsairLedId = 796;
pub const CorsairLedId_CLLC_C1_36: CorsairLedId = 797;
pub const CorsairLedId_CLLC_C1_37: CorsairLedId = 798;
pub const CorsairLedId_CLLC_C1_38: CorsairLedId = 799;
pub const CorsairLedId_CLLC_C1_39: CorsairLedId = 800;
pub const CorsairLedId_CLLC_C1_40: CorsairLedId = 801;
pub const CorsairLedId_CLLC_C1_41: CorsairLedId = 802;
pub const CorsairLedId_CLLC_C1_42: CorsairLedId = 803;
pub const CorsairLedId_CLLC_C1_43: CorsairLedId = 804;
pub const CorsairLedId_CLLC_C1_44: CorsairLedId = 805;
pub const CorsairLedId_CLLC_C1_45: CorsairLedId = 806;
pub const CorsairLedId_CLLC_C1_46: CorsairLedId = 807;
pub const CorsairLedId_CLLC_C1_47: CorsairLedId = 808;
pub const CorsairLedId_CLLC_C1_48: CorsairLedId = 809;
pub const CorsairLedId_CLLC_C1_49: CorsairLedId = 810;
pub const CorsairLedId_CLLC_C1_50: CorsairLedId = 811;
pub const CorsairLedId_CLLC_C1_51: CorsairLedId = 812;
pub const CorsairLedId_CLLC_C1_52: CorsairLedId = 813;
pub const CorsairLedId_CLLC_C1_53: CorsairLedId = 814;
pub const CorsairLedId_CLLC_C1_54: CorsairLedId = 815;
pub const CorsairLedId_CLLC_C1_55: CorsairLedId = 816;
pub const CorsairLedId_CLLC_C1_56: CorsairLedId = 817;
pub const CorsairLedId_CLLC_C1_57: CorsairLedId = 818;
pub const CorsairLedId_CLLC_C1_58: CorsairLedId = 819;
pub const CorsairLedId_CLLC_C1_59: CorsairLedId = 820;
pub const CorsairLedId_CLLC_C1_60: CorsairLedId = 821;
pub const CorsairLedId_CLLC_C1_61: CorsairLedId = 822;
pub const CorsairLedId_CLLC_C1_62: CorsairLedId = 823;
pub const CorsairLedId_CLLC_C1_63: CorsairLedId = 824;
pub const CorsairLedId_CLLC_C1_64: CorsairLedId = 825;
pub const CorsairLedId_CLLC_C1_65: CorsairLedId = 826;
pub const CorsairLedId_CLLC_C1_66: CorsairLedId = 827;
pub const CorsairLedId_CLLC_C1_67: CorsairLedId = 828;
pub const CorsairLedId_CLLC_C1_68: CorsairLedId = 829;
pub const CorsairLedId_CLLC_C1_69: CorsairLedId = 830;
pub const CorsairLedId_CLLC_C1_70: CorsairLedId = 831;
pub const CorsairLedId_CLLC_C1_71: CorsairLedId = 832;
pub const CorsairLedId_CLLC_C1_72: CorsairLedId = 833;
pub const CorsairLedId_CLLC_C1_73: CorsairLedId = 834;
pub const CorsairLedId_CLLC_C1_74: CorsairLedId = 835;
pub const CorsairLedId_CLLC_C1_75: CorsairLedId = 836;
pub const CorsairLedId_CLLC_C1_76: CorsairLedId = 837;
pub const CorsairLedId_CLLC_C1_77: CorsairLedId = 838;
pub const CorsairLedId_CLLC_C1_78: CorsairLedId = 839;
pub const CorsairLedId_CLLC_C1_79: CorsairLedId = 840;
pub const CorsairLedId_CLLC_C1_80: CorsairLedId = 841;
pub const CorsairLedId_CLLC_C1_81: CorsairLedId = 842;
pub const CorsairLedId_CLLC_C1_82: CorsairLedId = 843;
pub const CorsairLedId_CLLC_C1_83: CorsairLedId = 844;
pub const CorsairLedId_CLLC_C1_84: CorsairLedId = 845;
pub const CorsairLedId_CLLC_C1_85: CorsairLedId = 846;
pub const CorsairLedId_CLLC_C1_86: CorsairLedId = 847;
pub const CorsairLedId_CLLC_C1_87: CorsairLedId = 848;
pub const CorsairLedId_CLLC_C1_88: CorsairLedId = 849;
pub const CorsairLedId_CLLC_C1_89: CorsairLedId = 850;
pub const CorsairLedId_CLLC_C1_90: CorsairLedId = 851;
pub const CorsairLedId_CLLC_C1_91: CorsairLedId = 852;
pub const CorsairLedId_CLLC_C1_92: CorsairLedId = 853;
pub const CorsairLedId_CLLC_C1_93: CorsairLedId = 854;
pub const CorsairLedId_CLLC_C1_94: CorsairLedId = 855;
pub const CorsairLedId_CLLC_C1_95: CorsairLedId = 856;
pub const CorsairLedId_CLLC_C1_96: CorsairLedId = 857;
pub const CorsairLedId_CLLC_C1_97: CorsairLedId = 858;
pub const CorsairLedId_CLLC_C1_98: CorsairLedId = 859;
pub const CorsairLedId_CLLC_C1_99: CorsairLedId = 860;
pub const CorsairLedId_CLLC_C1_100: CorsairLedId = 861;
pub const CorsairLedId_CLLC_C1_101: CorsairLedId = 862;
pub const CorsairLedId_CLLC_C1_102: CorsairLedId = 863;
pub const CorsairLedId_CLLC_C1_103: CorsairLedId = 864;
pub const CorsairLedId_CLLC_C1_104: CorsairLedId = 865;
pub const CorsairLedId_CLLC_C1_105: CorsairLedId = 866;
pub const CorsairLedId_CLLC_C1_106: CorsairLedId = 867;
pub const CorsairLedId_CLLC_C1_107: CorsairLedId = 868;
pub const CorsairLedId_CLLC_C1_108: CorsairLedId = 869;
pub const CorsairLedId_CLLC_C1_109: CorsairLedId = 870;
pub const CorsairLedId_CLLC_C1_110: CorsairLedId = 871;
pub const CorsairLedId_CLLC_C1_111: CorsairLedId = 872;
pub const CorsairLedId_CLLC_C1_112: CorsairLedId = 873;
pub const CorsairLedId_CLLC_C1_113: CorsairLedId = 874;
pub const CorsairLedId_CLLC_C1_114: CorsairLedId = 875;
pub const CorsairLedId_CLLC_C1_115: CorsairLedId = 876;
pub const CorsairLedId_CLLC_C1_116: CorsairLedId = 877;
pub const CorsairLedId_CLLC_C1_117: CorsairLedId = 878;
pub const CorsairLedId_CLLC_C1_118: CorsairLedId = 879;
pub const CorsairLedId_CLLC_C1_119: CorsairLedId = 880;
pub const CorsairLedId_CLLC_C1_120: CorsairLedId = 881;
pub const CorsairLedId_CLLC_C1_121: CorsairLedId = 882;
pub const CorsairLedId_CLLC_C1_122: CorsairLedId = 883;
pub const CorsairLedId_CLLC_C1_123: CorsairLedId = 884;
pub const CorsairLedId_CLLC_C1_124: CorsairLedId = 885;
pub const CorsairLedId_CLLC_C1_125: CorsairLedId = 886;
pub const CorsairLedId_CLLC_C1_126: CorsairLedId = 887;
pub const CorsairLedId_CLLC_C1_127: CorsairLedId = 888;
pub const CorsairLedId_CLLC_C1_128: CorsairLedId = 889;
pub const CorsairLedId_CLLC_C1_129: CorsairLedId = 890;
pub const CorsairLedId_CLLC_C1_130: CorsairLedId = 891;
pub const CorsairLedId_CLLC_C1_131: CorsairLedId = 892;
pub const CorsairLedId_CLLC_C1_132: CorsairLedId = 893;
pub const CorsairLedId_CLLC_C1_133: CorsairLedId = 894;
pub const CorsairLedId_CLLC_C1_134: CorsairLedId = 895;
pub const CorsairLedId_CLLC_C1_135: CorsairLedId = 896;
pub const CorsairLedId_CLLC_C1_136: CorsairLedId = 897;
pub const CorsairLedId_CLLC_C1_137: CorsairLedId = 898;
pub const CorsairLedId_CLLC_C1_138: CorsairLedId = 899;
pub const CorsairLedId_CLLC_C1_139: CorsairLedId = 900;
pub const CorsairLedId_CLLC_C1_140: CorsairLedId = 901;
pub const CorsairLedId_CLLC_C1_141: CorsairLedId = 902;
pub const CorsairLedId_CLLC_C1_142: CorsairLedId = 903;
pub const CorsairLedId_CLLC_C1_143: CorsairLedId = 904;
pub const CorsairLedId_CLLC_C1_144: CorsairLedId = 905;
pub const CorsairLedId_CLLC_C1_145: CorsairLedId = 906;
pub const CorsairLedId_CLLC_C1_146: CorsairLedId = 907;
pub const CorsairLedId_CLLC_C1_147: CorsairLedId = 908;
pub const CorsairLedId_CLLC_C1_148: CorsairLedId = 909;
pub const CorsairLedId_CLLC_C1_149: CorsairLedId = 910;
pub const CorsairLedId_CLLC_C1_150: CorsairLedId = 911;
pub const CorsairLedId_CLD_C1_151: CorsairLedId = 912;
pub const CorsairLedId_CLD_C1_152: CorsairLedId = 913;
pub const CorsairLedId_CLD_C1_153: CorsairLedId = 914;
pub const CorsairLedId_CLD_C1_154: CorsairLedId = 915;
pub const CorsairLedId_CLD_C1_155: CorsairLedId = 916;
pub const CorsairLedId_CLD_C1_156: CorsairLedId = 917;
pub const CorsairLedId_CLD_C1_157: CorsairLedId = 918;
pub const CorsairLedId_CLD_C1_158: CorsairLedId = 919;
pub const CorsairLedId_CLD_C1_159: CorsairLedId = 920;
pub const CorsairLedId_CLD_C1_160: CorsairLedId = 921;
pub const CorsairLedId_CLD_C1_161: CorsairLedId = 922;
pub const CorsairLedId_CLD_C1_162: CorsairLedId = 923;
pub const CorsairLedId_CLD_C1_163: CorsairLedId = 924;
pub const CorsairLedId_CLD_C1_164: CorsairLedId = 925;
pub const CorsairLedId_CLD_C1_165: CorsairLedId = 926;
pub const CorsairLedId_CLD_C1_166: CorsairLedId = 927;
pub const CorsairLedId_CLD_C1_167: CorsairLedId = 928;
pub const CorsairLedId_CLD_C1_168: CorsairLedId = 929;
pub const CorsairLedId_CLD_C1_169: CorsairLedId = 930;
pub const CorsairLedId_CLD_C1_170: CorsairLedId = 931;
pub const CorsairLedId_CLD_C1_171: CorsairLedId = 932;
pub const CorsairLedId_CLD_C1_172: CorsairLedId = 933;
pub const CorsairLedId_CLD_C1_173: CorsairLedId = 934;
pub const CorsairLedId_CLD_C1_174: CorsairLedId = 935;
pub const CorsairLedId_CLD_C1_175: CorsairLedId = 936;
pub const CorsairLedId_CLD_C1_176: CorsairLedId = 937;
pub const CorsairLedId_CLD_C1_177: CorsairLedId = 938;
pub const CorsairLedId_CLD_C1_178: CorsairLedId = 939;
pub const CorsairLedId_CLD_C1_179: CorsairLedId = 940;
pub const CorsairLedId_CLD_C1_180: CorsairLedId = 941;
pub const CorsairLedId_CLD_C1_181: CorsairLedId = 942;
pub const CorsairLedId_CLD_C1_182: CorsairLedId = 943;
pub const CorsairLedId_CLD_C1_183: CorsairLedId = 944;
pub const CorsairLedId_CLD_C1_184: CorsairLedId = 945;
pub const CorsairLedId_CLD_C1_185: CorsairLedId = 946;
pub const CorsairLedId_CLD_C1_186: CorsairLedId = 947;
pub const CorsairLedId_CLD_C1_187: CorsairLedId = 948;
pub const CorsairLedId_CLD_C1_188: CorsairLedId = 949;
pub const CorsairLedId_CLD_C1_189: CorsairLedId = 950;
pub const CorsairLedId_CLD_C1_190: CorsairLedId = 951;
pub const CorsairLedId_CLD_C1_191: CorsairLedId = 952;
pub const CorsairLedId_CLD_C1_192: CorsairLedId = 953;
pub const CorsairLedId_CLD_C1_193: CorsairLedId = 954;
pub const CorsairLedId_CLD_C1_194: CorsairLedId = 955;
pub const CorsairLedId_CLD_C1_195: CorsairLedId = 956;
pub const CorsairLedId_CLD_C1_196: CorsairLedId = 957;
pub const CorsairLedId_CLD_C1_197: CorsairLedId = 958;
pub const CorsairLedId_CLD_C1_198: CorsairLedId = 959;
pub const CorsairLedId_CLD_C1_199: CorsairLedId = 960;
pub const CorsairLedId_CLD_C1_200: CorsairLedId = 961;
pub const CorsairLedId_CLD_C1_201: CorsairLedId = 962;
pub const CorsairLedId_CLD_C1_202: CorsairLedId = 963;
pub const CorsairLedId_CLD_C1_203: CorsairLedId = 964;
pub const CorsairLedId_CLD_C1_204: CorsairLedId = 965;
pub const CorsairLedId_CLD_C1_205: CorsairLedId = 966;
pub const CorsairLedId_CLD_C1_206: CorsairLedId = 967;
pub const CorsairLedId_CLD_C1_207: CorsairLedId = 968;
pub const CorsairLedId_CLD_C1_208: CorsairLedId = 969;
pub const CorsairLedId_CLD_C1_209: CorsairLedId = 970;
pub const CorsairLedId_CLD_C1_210: CorsairLedId = 971;
pub const CorsairLedId_CLD_C1_211: CorsairLedId = 972;
pub const CorsairLedId_CLD_C1_212: CorsairLedId = 973;
pub const CorsairLedId_CLD_C1_213: CorsairLedId = 974;
pub const CorsairLedId_CLD_C1_214: CorsairLedId = 975;
pub const CorsairLedId_CLD_C1_215: CorsairLedId = 976;
pub const CorsairLedId_CLD_C1_216: CorsairLedId = 977;
pub const CorsairLedId_CLD_C1_217: CorsairLedId = 978;
pub const CorsairLedId_CLD_C1_218: CorsairLedId = 979;
pub const CorsairLedId_CLD_C1_219: CorsairLedId = 980;
pub const CorsairLedId_CLD_C1_220: CorsairLedId = 981;
pub const CorsairLedId_CLD_C1_221: CorsairLedId = 982;
pub const CorsairLedId_CLD_C1_222: CorsairLedId = 983;
pub const CorsairLedId_CLD_C1_223: CorsairLedId = 984;
pub const CorsairLedId_CLD_C1_224: CorsairLedId = 985;
pub const CorsairLedId_CLD_C1_225: CorsairLedId = 986;
pub const CorsairLedId_CLD_C1_226: CorsairLedId = 987;
pub const CorsairLedId_CLD_C1_227: CorsairLedId = 988;
pub const CorsairLedId_CLD_C1_228: CorsairLedId = 989;
pub const CorsairLedId_CLD_C1_229: CorsairLedId = 990;
pub const CorsairLedId_CLD_C1_230: CorsairLedId = 991;
pub const CorsairLedId_CLD_C1_231: CorsairLedId = 992;
pub const CorsairLedId_CLD_C1_232: CorsairLedId = 993;
pub const CorsairLedId_CLD_C1_233: CorsairLedId = 994;
pub const CorsairLedId_CLD_C1_234: CorsairLedId = 995;
pub const CorsairLedId_CLD_C1_235: CorsairLedId = 996;
pub const CorsairLedId_CLD_C1_236: CorsairLedId = 997;
pub const CorsairLedId_CLD_C1_237: CorsairLedId = 998;
pub const CorsairLedId_CLD_C1_238: CorsairLedId = 999;
pub const CorsairLedId_CLD_C1_239: CorsairLedId = 1000;
pub const CorsairLedId_CLD_C1_240: CorsairLedId = 1001;
pub const CorsairLedId_CLD_C1_241: CorsairLedId = 1002;
pub const CorsairLedId_CLD_C1_242: CorsairLedId = 1003;
pub const CorsairLedId_CLD_C1_243: CorsairLedId = 1004;
pub const CorsairLedId_CLD_C1_244: CorsairLedId = 1005;
pub const CorsairLedId_CLD_C1_245: CorsairLedId = 1006;
pub const CorsairLedId_CLD_C1_246: CorsairLedId = 1007;
pub const CorsairLedId_CLD_C1_247: CorsairLedId = 1008;
pub const CorsairLedId_CLD_C1_248: CorsairLedId = 1009;
pub const CorsairLedId_CLD_C1_249: CorsairLedId = 1010;
pub const CorsairLedId_CLD_C1_250: CorsairLedId = 1011;
pub const CorsairLedId_CLD_C1_251: CorsairLedId = 1012;
pub const CorsairLedId_CLD_C1_252: CorsairLedId = 1013;
pub const CorsairLedId_CLD_C1_253: CorsairLedId = 1014;
pub const CorsairLedId_CLD_C1_254: CorsairLedId = 1015;
pub const CorsairLedId_CLD_C1_255: CorsairLedId = 1016;
pub const CorsairLedId_CLD_C1_256: CorsairLedId = 1017;
pub const CorsairLedId_CLD_C1_257: CorsairLedId = 1018;
pub const CorsairLedId_CLD_C1_258: CorsairLedId = 1019;
pub const CorsairLedId_CLD_C1_259: CorsairLedId = 1020;
pub const CorsairLedId_CLD_C1_260: CorsairLedId = 1021;
pub const CorsairLedId_CLD_C1_261: CorsairLedId = 1022;
pub const CorsairLedId_CLD_C1_262: CorsairLedId = 1023;
pub const CorsairLedId_CLD_C1_263: CorsairLedId = 1024;
pub const CorsairLedId_CLD_C1_264: CorsairLedId = 1025;
pub const CorsairLedId_CLD_C1_265: CorsairLedId = 1026;
pub const CorsairLedId_CLD_C1_266: CorsairLedId = 1027;
pub const CorsairLedId_CLD_C1_267: CorsairLedId = 1028;
pub const CorsairLedId_CLD_C1_268: CorsairLedId = 1029;
pub const CorsairLedId_CLD_C1_269: CorsairLedId = 1030;
pub const CorsairLedId_CLD_C1_270: CorsairLedId = 1031;
pub const CorsairLedId_CLD_C1_271: CorsairLedId = 1032;
pub const CorsairLedId_CLD_C1_272: CorsairLedId = 1033;
pub const CorsairLedId_CLD_C1_273: CorsairLedId = 1034;
pub const CorsairLedId_CLD_C1_274: CorsairLedId = 1035;
pub const CorsairLedId_CLD_C1_275: CorsairLedId = 1036;
pub const CorsairLedId_CLD_C1_276: CorsairLedId = 1037;
pub const CorsairLedId_CLD_C1_277: CorsairLedId = 1038;
pub const CorsairLedId_CLD_C1_278: CorsairLedId = 1039;
pub const CorsairLedId_CLD_C1_279: CorsairLedId = 1040;
pub const CorsairLedId_CLD_C1_280: CorsairLedId = 1041;
pub const CorsairLedId_CLD_C1_281: CorsairLedId = 1042;
pub const CorsairLedId_CLD_C1_282: CorsairLedId = 1043;
pub const CorsairLedId_CLD_C1_283: CorsairLedId = 1044;
pub const CorsairLedId_CLD_C1_284: CorsairLedId = 1045;
pub const CorsairLedId_CLD_C1_285: CorsairLedId = 1046;
pub const CorsairLedId_CLD_C1_286: CorsairLedId = 1047;
pub const CorsairLedId_CLD_C1_287: CorsairLedId = 1048;
pub const CorsairLedId_CLD_C1_288: CorsairLedId = 1049;
pub const CorsairLedId_CLD_C1_289: CorsairLedId = 1050;
pub const CorsairLedId_CLD_C1_290: CorsairLedId = 1051;
pub const CorsairLedId_CLD_C1_291: CorsairLedId = 1052;
pub const CorsairLedId_CLD_C1_292: CorsairLedId = 1053;
pub const CorsairLedId_CLD_C1_293: CorsairLedId = 1054;
pub const CorsairLedId_CLD_C1_294: CorsairLedId = 1055;
pub const CorsairLedId_CLD_C1_295: CorsairLedId = 1056;
pub const CorsairLedId_CLD_C1_296: CorsairLedId = 1057;
pub const CorsairLedId_CLD_C1_297: CorsairLedId = 1058;
pub const CorsairLedId_CLD_C1_298: CorsairLedId = 1059;
pub const CorsairLedId_CLD_C1_299: CorsairLedId = 1060;
pub const CorsairLedId_CLD_C1_300: CorsairLedId = 1061;
pub const CorsairLedId_CLD_C2_151: CorsairLedId = 1062;
pub const CorsairLedId_CLD_C2_152: CorsairLedId = 1063;
pub const CorsairLedId_CLD_C2_153: CorsairLedId = 1064;
pub const CorsairLedId_CLD_C2_154: CorsairLedId = 1065;
pub const CorsairLedId_CLD_C2_155: CorsairLedId = 1066;
pub const CorsairLedId_CLD_C2_156: CorsairLedId = 1067;
pub const CorsairLedId_CLD_C2_157: CorsairLedId = 1068;
pub const CorsairLedId_CLD_C2_158: CorsairLedId = 1069;
pub const CorsairLedId_CLD_C2_159: CorsairLedId = 1070;
pub const CorsairLedId_CLD_C2_160: CorsairLedId = 1071;
pub const CorsairLedId_CLD_C2_161: CorsairLedId = 1072;
pub const CorsairLedId_CLD_C2_162: CorsairLedId = 1073;
pub const CorsairLedId_CLD_C2_163: CorsairLedId = 1074;
pub const CorsairLedId_CLD_C2_164: CorsairLedId = 1075;
pub const CorsairLedId_CLD_C2_165: CorsairLedId = 1076;
pub const CorsairLedId_CLD_C2_166: CorsairLedId = 1077;
pub const CorsairLedId_CLD_C2_167: CorsairLedId = 1078;
pub const CorsairLedId_CLD_C2_168: CorsairLedId = 1079;
pub const CorsairLedId_CLD_C2_169: CorsairLedId = 1080;
pub const CorsairLedId_CLD_C2_170: CorsairLedId = 1081;
pub const CorsairLedId_CLD_C2_171: CorsairLedId = 1082;
pub const CorsairLedId_CLD_C2_172: CorsairLedId = 1083;
pub const CorsairLedId_CLD_C2_173: CorsairLedId = 1084;
pub const CorsairLedId_CLD_C2_174: CorsairLedId = 1085;
pub const CorsairLedId_CLD_C2_175: CorsairLedId = 1086;
pub const CorsairLedId_CLD_C2_176: CorsairLedId = 1087;
pub const CorsairLedId_CLD_C2_177: CorsairLedId = 1088;
pub const CorsairLedId_CLD_C2_178: CorsairLedId = 1089;
pub const CorsairLedId_CLD_C2_179: CorsairLedId = 1090;
pub const CorsairLedId_CLD_C2_180: CorsairLedId = 1091;
pub const CorsairLedId_CLD_C2_181: CorsairLedId = 1092;
pub const CorsairLedId_CLD_C2_182: CorsairLedId = 1093;
pub const CorsairLedId_CLD_C2_183: CorsairLedId = 1094;
pub const CorsairLedId_CLD_C2_184: CorsairLedId = 1095;
pub const CorsairLedId_CLD_C2_185: CorsairLedId = 1096;
pub const CorsairLedId_CLD_C2_186: CorsairLedId = 1097;
pub const CorsairLedId_CLD_C2_187: CorsairLedId = 1098;
pub const CorsairLedId_CLD_C2_188: CorsairLedId = 1099;
pub const CorsairLedId_CLD_C2_189: CorsairLedId = 1100;
pub const CorsairLedId_CLD_C2_190: CorsairLedId = 1101;
pub const CorsairLedId_CLD_C2_191: CorsairLedId = 1102;
pub const CorsairLedId_CLD_C2_192: CorsairLedId = 1103;
pub const CorsairLedId_CLD_C2_193: CorsairLedId = 1104;
pub const CorsairLedId_CLD_C2_194: CorsairLedId = 1105;
pub const CorsairLedId_CLD_C2_195: CorsairLedId = 1106;
pub const CorsairLedId_CLD_C2_196: CorsairLedId = 1107;
pub const CorsairLedId_CLD_C2_197: CorsairLedId = 1108;
pub const CorsairLedId_CLD_C2_198: CorsairLedId = 1109;
pub const CorsairLedId_CLD_C2_199: CorsairLedId = 1110;
pub const CorsairLedId_CLD_C2_200: CorsairLedId = 1111;
pub const CorsairLedId_CLD_C2_201: CorsairLedId = 1112;
pub const CorsairLedId_CLD_C2_202: CorsairLedId = 1113;
pub const CorsairLedId_CLD_C2_203: CorsairLedId = 1114;
pub const CorsairLedId_CLD_C2_204: CorsairLedId = 1115;
pub const CorsairLedId_CLD_C2_205: CorsairLedId = 1116;
pub const CorsairLedId_CLD_C2_206: CorsairLedId = 1117;
pub const CorsairLedId_CLD_C2_207: CorsairLedId = 1118;
pub const CorsairLedId_CLD_C2_208: CorsairLedId = 1119;
pub const CorsairLedId_CLD_C2_209: CorsairLedId = 1120;
pub const CorsairLedId_CLD_C2_210: CorsairLedId = 1121;
pub const CorsairLedId_CLD_C2_211: CorsairLedId = 1122;
pub const CorsairLedId_CLD_C2_212: CorsairLedId = 1123;
pub const CorsairLedId_CLD_C2_213: CorsairLedId = 1124;
pub const CorsairLedId_CLD_C2_214: CorsairLedId = 1125;
pub const CorsairLedId_CLD_C2_215: CorsairLedId = 1126;
pub const CorsairLedId_CLD_C2_216: CorsairLedId = 1127;
pub const CorsairLedId_CLD_C2_217: CorsairLedId = 1128;
pub const CorsairLedId_CLD_C2_218: CorsairLedId = 1129;
pub const CorsairLedId_CLD_C2_219: CorsairLedId = 1130;
pub const CorsairLedId_CLD_C2_220: CorsairLedId = 1131;
pub const CorsairLedId_CLD_C2_221: CorsairLedId = 1132;
pub const CorsairLedId_CLD_C2_222: CorsairLedId = 1133;
pub const CorsairLedId_CLD_C2_223: CorsairLedId = 1134;
pub const CorsairLedId_CLD_C2_224: CorsairLedId = 1135;
pub const CorsairLedId_CLD_C2_225: CorsairLedId = 1136;
pub const CorsairLedId_CLD_C2_226: CorsairLedId = 1137;
pub const CorsairLedId_CLD_C2_227: CorsairLedId = 1138;
pub const CorsairLedId_CLD_C2_228: CorsairLedId = 1139;
pub const CorsairLedId_CLD_C2_229: CorsairLedId = 1140;
pub const CorsairLedId_CLD_C2_230: CorsairLedId = 1141;
pub const CorsairLedId_CLD_C2_231: CorsairLedId = 1142;
pub const CorsairLedId_CLD_C2_232: CorsairLedId = 1143;
pub const CorsairLedId_CLD_C2_233: CorsairLedId = 1144;
pub const CorsairLedId_CLD_C2_234: CorsairLedId = 1145;
pub const CorsairLedId_CLD_C2_235: CorsairLedId = 1146;
pub const CorsairLedId_CLD_C2_236: CorsairLedId = 1147;
pub const CorsairLedId_CLD_C2_237: CorsairLedId = 1148;
pub const CorsairLedId_CLD_C2_238: CorsairLedId = 1149;
pub const CorsairLedId_CLD_C2_239: CorsairLedId = 1150;
pub const CorsairLedId_CLD_C2_240: CorsairLedId = 1151;
pub const CorsairLedId_CLD_C2_241: CorsairLedId = 1152;
pub const CorsairLedId_CLD_C2_242: CorsairLedId = 1153;
pub const CorsairLedId_CLD_C2_243: CorsairLedId = 1154;
pub const CorsairLedId_CLD_C2_244: CorsairLedId = 1155;
pub const CorsairLedId_CLD_C2_245: CorsairLedId = 1156;
pub const CorsairLedId_CLD_C2_246: CorsairLedId = 1157;
pub const CorsairLedId_CLD_C2_247: CorsairLedId = 1158;
pub const CorsairLedId_CLD_C2_248: CorsairLedId = 1159;
pub const CorsairLedId_CLD_C2_249: CorsairLedId = 1160;
pub const CorsairLedId_CLD_C2_250: CorsairLedId = 1161;
pub const CorsairLedId_CLD_C2_251: CorsairLedId = 1162;
pub const CorsairLedId_CLD_C2_252: CorsairLedId = 1163;
pub const CorsairLedId_CLD_C2_253: CorsairLedId = 1164;
pub const CorsairLedId_CLD_C2_254: CorsairLedId = 1165;
pub const CorsairLedId_CLD_C2_255: CorsairLedId = 1166;
pub const CorsairLedId_CLD_C2_256: CorsairLedId = 1167;
pub const CorsairLedId_CLD_C2_257: CorsairLedId = 1168;
pub const CorsairLedId_CLD_C2_258: CorsairLedId = 1169;
pub const CorsairLedId_CLD_C2_259: CorsairLedId = 1170;
pub const CorsairLedId_CLD_C2_260: CorsairLedId = 1171;
pub const CorsairLedId_CLD_C2_261: CorsairLedId = 1172;
pub const CorsairLedId_CLD_C2_262: CorsairLedId = 1173;
pub const CorsairLedId_CLD_C2_263: CorsairLedId = 1174;
pub const CorsairLedId_CLD_C2_264: CorsairLedId = 1175;
pub const CorsairLedId_CLD_C2_265: CorsairLedId = 1176;
pub const CorsairLedId_CLD_C2_266: CorsairLedId = 1177;
pub const CorsairLedId_CLD_C2_267: CorsairLedId = 1178;
pub const CorsairLedId_CLD_C2_268: CorsairLedId = 1179;
pub const CorsairLedId_CLD_C2_269: CorsairLedId = 1180;
pub const CorsairLedId_CLD_C2_270: CorsairLedId = 1181;
pub const CorsairLedId_CLD_C2_271: CorsairLedId = 1182;
pub const CorsairLedId_CLD_C2_272: CorsairLedId = 1183;
pub const CorsairLedId_CLD_C2_273: CorsairLedId = 1184;
pub const CorsairLedId_CLD_C2_274: CorsairLedId = 1185;
pub const CorsairLedId_CLD_C2_275: CorsairLedId = 1186;
pub const CorsairLedId_CLD_C2_276: CorsairLedId = 1187;
pub const CorsairLedId_CLD_C2_277: CorsairLedId = 1188;
pub const CorsairLedId_CLD_C2_278: CorsairLedId = 1189;
pub const CorsairLedId_CLD_C2_279: CorsairLedId = 1190;
pub const CorsairLedId_CLD_C2_280: CorsairLedId = 1191;
pub const CorsairLedId_CLD_C2_281: CorsairLedId = 1192;
pub const CorsairLedId_CLD_C2_282: CorsairLedId = 1193;
pub const CorsairLedId_CLD_C2_283: CorsairLedId = 1194;
pub const CorsairLedId_CLD_C2_284: CorsairLedId = 1195;
pub const CorsairLedId_CLD_C2_285: CorsairLedId = 1196;
pub const CorsairLedId_CLD_C2_286: CorsairLedId = 1197;
pub const CorsairLedId_CLD_C2_287: CorsairLedId = 1198;
pub const CorsairLedId_CLD_C2_288: CorsairLedId = 1199;
pub const CorsairLedId_CLD_C2_289: CorsairLedId = 1200;
pub const CorsairLedId_CLD_C2_290: CorsairLedId = 1201;
pub const CorsairLedId_CLD_C2_291: CorsairLedId = 1202;
pub const CorsairLedId_CLD_C2_292: CorsairLedId = 1203;
pub const CorsairLedId_CLD_C2_293: CorsairLedId = 1204;
pub const CorsairLedId_CLD_C2_294: CorsairLedId = 1205;
pub const CorsairLedId_CLD_C2_295: CorsairLedId = 1206;
pub const CorsairLedId_CLD_C2_296: CorsairLedId = 1207;
pub const CorsairLedId_CLD_C2_297: CorsairLedId = 1208;
pub const CorsairLedId_CLD_C2_298: CorsairLedId = 1209;
pub const CorsairLedId_CLD_C2_299: CorsairLedId = 1210;
pub const CorsairLedId_CLD_C2_300: CorsairLedId = 1211;
pub const CorsairLedId_CLD_C3_151: CorsairLedId = 1212;
pub const CorsairLedId_CLD_C3_152: CorsairLedId = 1213;
pub const CorsairLedId_CLD_C3_153: CorsairLedId = 1214;
pub const CorsairLedId_CLD_C3_154: CorsairLedId = 1215;
pub const CorsairLedId_CLD_C3_155: CorsairLedId = 1216;
pub const CorsairLedId_CLD_C3_156: CorsairLedId = 1217;
pub const CorsairLedId_CLD_C3_157: CorsairLedId = 1218;
pub const CorsairLedId_CLD_C3_158: CorsairLedId = 1219;
pub const CorsairLedId_CLD_C3_159: CorsairLedId = 1220;
pub const CorsairLedId_CLD_C3_160: CorsairLedId = 1221;
pub const CorsairLedId_CLD_C3_161: CorsairLedId = 1222;
pub const CorsairLedId_CLD_C3_162: CorsairLedId = 1223;
pub const CorsairLedId_CLD_C3_163: CorsairLedId = 1224;
pub const CorsairLedId_CLD_C3_164: CorsairLedId = 1225;
pub const CorsairLedId_CLD_C3_165: CorsairLedId = 1226;
pub const CorsairLedId_CLD_C3_166: CorsairLedId = 1227;
pub const CorsairLedId_CLD_C3_167: CorsairLedId = 1228;
pub const CorsairLedId_CLD_C3_168: CorsairLedId = 1229;
pub const CorsairLedId_CLD_C3_169: CorsairLedId = 1230;
pub const CorsairLedId_CLD_C3_170: CorsairLedId = 1231;
pub const CorsairLedId_CLD_C3_171: CorsairLedId = 1232;
pub const CorsairLedId_CLD_C3_172: CorsairLedId = 1233;
pub const CorsairLedId_CLD_C3_173: CorsairLedId = 1234;
pub const CorsairLedId_CLD_C3_174: CorsairLedId = 1235;
pub const CorsairLedId_CLD_C3_175: CorsairLedId = 1236;
pub const CorsairLedId_CLD_C3_176: CorsairLedId = 1237;
pub const CorsairLedId_CLD_C3_177: CorsairLedId = 1238;
pub const CorsairLedId_CLD_C3_178: CorsairLedId = 1239;
pub const CorsairLedId_CLD_C3_179: CorsairLedId = 1240;
pub const CorsairLedId_CLD_C3_180: CorsairLedId = 1241;
pub const CorsairLedId_CLD_C3_181: CorsairLedId = 1242;
pub const CorsairLedId_CLD_C3_182: CorsairLedId = 1243;
pub const CorsairLedId_CLD_C3_183: CorsairLedId = 1244;
pub const CorsairLedId_CLD_C3_184: CorsairLedId = 1245;
pub const CorsairLedId_CLD_C3_185: CorsairLedId = 1246;
pub const CorsairLedId_CLD_C3_186: CorsairLedId = 1247;
pub const CorsairLedId_CLD_C3_187: CorsairLedId = 1248;
pub const CorsairLedId_CLD_C3_188: CorsairLedId = 1249;
pub const CorsairLedId_CLD_C3_189: CorsairLedId = 1250;
pub const CorsairLedId_CLD_C3_190: CorsairLedId = 1251;
pub const CorsairLedId_CLD_C3_191: CorsairLedId = 1252;
pub const CorsairLedId_CLD_C3_192: CorsairLedId = 1253;
pub const CorsairLedId_CLD_C3_193: CorsairLedId = 1254;
pub const CorsairLedId_CLD_C3_194: CorsairLedId = 1255;
pub const CorsairLedId_CLD_C3_195: CorsairLedId = 1256;
pub const CorsairLedId_CLD_C3_196: CorsairLedId = 1257;
pub const CorsairLedId_CLD_C3_197: CorsairLedId = 1258;
pub const CorsairLedId_CLD_C3_198: CorsairLedId = 1259;
pub const CorsairLedId_CLD_C3_199: CorsairLedId = 1260;
pub const CorsairLedId_CLD_C3_200: CorsairLedId = 1261;
pub const CorsairLedId_CLD_C3_201: CorsairLedId = 1262;
pub const CorsairLedId_CLD_C3_202: CorsairLedId = 1263;
pub const CorsairLedId_CLD_C3_203: CorsairLedId = 1264;
pub const CorsairLedId_CLD_C3_204: CorsairLedId = 1265;
pub const CorsairLedId_CLD_C3_205: CorsairLedId = 1266;
pub const CorsairLedId_CLD_C3_206: CorsairLedId = 1267;
pub const CorsairLedId_CLD_C3_207: CorsairLedId = 1268;
pub const CorsairLedId_CLD_C3_208: CorsairLedId = 1269;
pub const CorsairLedId_CLD_C3_209: CorsairLedId = 1270;
pub const CorsairLedId_CLD_C3_210: CorsairLedId = 1271;
pub const CorsairLedId_CLD_C3_211: CorsairLedId = 1272;
pub const CorsairLedId_CLD_C3_212: CorsairLedId = 1273;
pub const CorsairLedId_CLD_C3_213: CorsairLedId = 1274;
pub const CorsairLedId_CLD_C3_214: CorsairLedId = 1275;
pub const CorsairLedId_CLD_C3_215: CorsairLedId = 1276;
pub const CorsairLedId_CLD_C3_216: CorsairLedId = 1277;
pub const CorsairLedId_CLD_C3_217: CorsairLedId = 1278;
pub const CorsairLedId_CLD_C3_218: CorsairLedId = 1279;
pub const CorsairLedId_CLD_C3_219: CorsairLedId = 1280;
pub const CorsairLedId_CLD_C3_220: CorsairLedId = 1281;
pub const CorsairLedId_CLD_C3_221: CorsairLedId = 1282;
pub const CorsairLedId_CLD_C3_222: CorsairLedId = 1283;
pub const CorsairLedId_CLD_C3_223: CorsairLedId = 1284;
pub const CorsairLedId_CLD_C3_224: CorsairLedId = 1285;
pub const CorsairLedId_CLD_C3_225: CorsairLedId = 1286;
pub const CorsairLedId_CLD_C3_226: CorsairLedId = 1287;
pub const CorsairLedId_CLD_C3_227: CorsairLedId = 1288;
pub const CorsairLedId_CLD_C3_228: CorsairLedId = 1289;
pub const CorsairLedId_CLD_C3_229: CorsairLedId = 1290;
pub const CorsairLedId_CLD_C3_230: CorsairLedId = 1291;
pub const CorsairLedId_CLD_C3_231: CorsairLedId = 1292;
pub const CorsairLedId_CLD_C3_232: CorsairLedId = 1293;
pub const CorsairLedId_CLD_C3_233: CorsairLedId = 1294;
pub const CorsairLedId_CLD_C3_234: CorsairLedId = 1295;
pub const CorsairLedId_CLD_C3_235: CorsairLedId = 1296;
pub const CorsairLedId_CLD_C3_236: CorsairLedId = 1297;
pub const CorsairLedId_CLD_C3_237: CorsairLedId = 1298;
pub const CorsairLedId_CLD_C3_238: CorsairLedId = 1299;
pub const CorsairLedId_CLD_C3_239: CorsairLedId = 1300;
pub const CorsairLedId_CLD_C3_240: CorsairLedId = 1301;
pub const CorsairLedId_CLD_C3_241: CorsairLedId = 1302;
pub const CorsairLedId_CLD_C3_242: CorsairLedId = 1303;
pub const CorsairLedId_CLD_C3_243: CorsairLedId = 1304;
pub const CorsairLedId_CLD_C3_244: CorsairLedId = 1305;
pub const CorsairLedId_CLD_C3_245: CorsairLedId = 1306;
pub const CorsairLedId_CLD_C3_246: CorsairLedId = 1307;
pub const CorsairLedId_CLD_C3_247: CorsairLedId = 1308;
pub const CorsairLedId_CLD_C3_248: CorsairLedId = 1309;
pub const CorsairLedId_CLD_C3_249: CorsairLedId = 1310;
pub const CorsairLedId_CLD_C3_250: CorsairLedId = 1311;
pub const CorsairLedId_CLD_C3_251: CorsairLedId = 1312;
pub const CorsairLedId_CLD_C3_252: CorsairLedId = 1313;
pub const CorsairLedId_CLD_C3_253: CorsairLedId = 1314;
pub const CorsairLedId_CLD_C3_254: CorsairLedId = 1315;
pub const CorsairLedId_CLD_C3_255: CorsairLedId = 1316;
pub const CorsairLedId_CLD_C3_256: CorsairLedId = 1317;
pub const CorsairLedId_CLD_C3_257: CorsairLedId = 1318;
pub const CorsairLedId_CLD_C3_258: CorsairLedId = 1319;
pub const CorsairLedId_CLD_C3_259: CorsairLedId = 1320;
pub const CorsairLedId_CLD_C3_260: CorsairLedId = 1321;
pub const CorsairLedId_CLD_C3_261: CorsairLedId = 1322;
pub const CorsairLedId_CLD_C3_262: CorsairLedId = 1323;
pub const CorsairLedId_CLD_C3_263: CorsairLedId = 1324;
pub const CorsairLedId_CLD_C3_264: CorsairLedId = 1325;
pub const CorsairLedId_CLD_C3_265: CorsairLedId = 1326;
pub const CorsairLedId_CLD_C3_266: CorsairLedId = 1327;
pub const CorsairLedId_CLD_C3_267: CorsairLedId = 1328;
pub const CorsairLedId_CLD_C3_268: CorsairLedId = 1329;
pub const CorsairLedId_CLD_C3_269: CorsairLedId = 1330;
pub const CorsairLedId_CLD_C3_270: CorsairLedId = 1331;
pub const CorsairLedId_CLD_C3_271: CorsairLedId = 1332;
pub const CorsairLedId_CLD_C3_272: CorsairLedId = 1333;
pub const CorsairLedId_CLD_C3_273: CorsairLedId = 1334;
pub const CorsairLedId_CLD_C3_274: CorsairLedId = 1335;
pub const CorsairLedId_CLD_C3_275: CorsairLedId = 1336;
pub const CorsairLedId_CLD_C3_276: CorsairLedId = 1337;
pub const CorsairLedId_CLD_C3_277: CorsairLedId = 1338;
pub const CorsairLedId_CLD_C3_278: CorsairLedId = 1339;
pub const CorsairLedId_CLD_C3_279: CorsairLedId = 1340;
pub const CorsairLedId_CLD_C3_280: CorsairLedId = 1341;
pub const CorsairLedId_CLD_C3_281: CorsairLedId = 1342;
pub const CorsairLedId_CLD_C3_282: CorsairLedId = 1343;
pub const CorsairLedId_CLD_C3_283: CorsairLedId = 1344;
pub const CorsairLedId_CLD_C3_284: CorsairLedId = 1345;
pub const CorsairLedId_CLD_C3_285: CorsairLedId = 1346;
pub const CorsairLedId_CLD_C3_286: CorsairLedId = 1347;
pub const CorsairLedId_CLD_C3_287: CorsairLedId = 1348;
pub const CorsairLedId_CLD_C3_288: CorsairLedId = 1349;
pub const CorsairLedId_CLD_C3_289: CorsairLedId = 1350;
pub const CorsairLedId_CLD_C3_290: CorsairLedId = 1351;
pub const CorsairLedId_CLD_C3_291: CorsairLedId = 1352;
pub const CorsairLedId_CLD_C3_292: CorsairLedId = 1353;
pub const CorsairLedId_CLD_C3_293: CorsairLedId = 1354;
pub const CorsairLedId_CLD_C3_294: CorsairLedId = 1355;
pub const CorsairLedId_CLD_C3_295: CorsairLedId = 1356;
pub const CorsairLedId_CLD_C3_296: CorsairLedId = 1357;
pub const CorsairLedId_CLD_C3_297: CorsairLedId = 1358;
pub const CorsairLedId_CLD_C3_298: CorsairLedId = 1359;
pub const CorsairLedId_CLD_C3_299: CorsairLedId = 1360;
pub const CorsairLedId_CLD_C3_300: CorsairLedId = 1361;
pub const CorsairLedId_CLMB_Zone1: CorsairLedId = 1362;
pub const CorsairLedId_CLMB_Zone2: CorsairLedId = 1363;
pub const CorsairLedId_CLMB_Zone3: CorsairLedId = 1364;
pub const CorsairLedId_CLMB_Zone4: CorsairLedId = 1365;
pub const CorsairLedId_CLMB_Zone5: CorsairLedId = 1366;
pub const CorsairLedId_CLMB_Zone6: CorsairLedId = 1367;
pub const CorsairLedId_CLMB_Zone7: CorsairLedId = 1368;
pub const CorsairLedId_CLMB_Zone8: CorsairLedId = 1369;
pub const CorsairLedId_CLMB_Zone9: CorsairLedId = 1370;
pub const CorsairLedId_CLMB_Zone10: CorsairLedId = 1371;
pub const CorsairLedId_CLMB_Zone11: CorsairLedId = 1372;
pub const CorsairLedId_CLMB_Zone12: CorsairLedId = 1373;
pub const CorsairLedId_CLMB_Zone13: CorsairLedId = 1374;
pub const CorsairLedId_CLMB_Zone14: CorsairLedId = 1375;
pub const CorsairLedId_CLMB_Zone15: CorsairLedId = 1376;
pub const CorsairLedId_CLMB_Zone16: CorsairLedId = 1377;
pub const CorsairLedId_CLMB_Zone17: CorsairLedId = 1378;
pub const CorsairLedId_CLMB_Zone18: CorsairLedId = 1379;
pub const CorsairLedId_CLMB_Zone19: CorsairLedId = 1380;
pub const CorsairLedId_CLMB_Zone20: CorsairLedId = 1381;
pub const CorsairLedId_CLMB_Zone21: CorsairLedId = 1382;
pub const CorsairLedId_CLMB_Zone22: CorsairLedId = 1383;
pub const CorsairLedId_CLMB_Zone23: CorsairLedId = 1384;
pub const CorsairLedId_CLMB_Zone24: CorsairLedId = 1385;
pub const CorsairLedId_CLMB_Zone25: CorsairLedId = 1386;
pub const CorsairLedId_CLMB_Zone26: CorsairLedId = 1387;
pub const CorsairLedId_CLMB_Zone27: CorsairLedId = 1388;
pub const CorsairLedId_CLMB_Zone28: CorsairLedId = 1389;
pub const CorsairLedId_CLMB_Zone29: CorsairLedId = 1390;
pub const CorsairLedId_CLMB_Zone30: CorsairLedId = 1391;
pub const CorsairLedId_CLMB_Zone31: CorsairLedId = 1392;
pub const CorsairLedId_CLMB_Zone32: CorsairLedId = 1393;
pub const CorsairLedId_CLMB_Zone33: CorsairLedId = 1394;
pub const CorsairLedId_CLMB_Zone34: CorsairLedId = 1395;
pub const CorsairLedId_CLMB_Zone35: CorsairLedId = 1396;
pub const CorsairLedId_CLMB_Zone36: CorsairLedId = 1397;
pub const CorsairLedId_CLMB_Zone37: CorsairLedId = 1398;
pub const CorsairLedId_CLMB_Zone38: CorsairLedId = 1399;
pub const CorsairLedId_CLMB_Zone39: CorsairLedId = 1400;
pub const CorsairLedId_CLMB_Zone40: CorsairLedId = 1401;
pub const CorsairLedId_CLMB_Zone41: CorsairLedId = 1402;
pub const CorsairLedId_CLMB_Zone42: CorsairLedId = 1403;
pub const CorsairLedId_CLMB_Zone43: CorsairLedId = 1404;
pub const CorsairLedId_CLMB_Zone44: CorsairLedId = 1405;
pub const CorsairLedId_CLMB_Zone45: CorsairLedId = 1406;
pub const CorsairLedId_CLMB_Zone46: CorsairLedId = 1407;
pub const CorsairLedId_CLMB_Zone47: CorsairLedId = 1408;
pub const CorsairLedId_CLMB_Zone48: CorsairLedId = 1409;
pub const CorsairLedId_CLMB_Zone49: CorsairLedId = 1410;
pub const CorsairLedId_CLMB_Zone50: CorsairLedId = 1411;
pub const CorsairLedId_CLMB_Zone51: CorsairLedId = 1412;
pub const CorsairLedId_CLMB_Zone52: CorsairLedId = 1413;
pub const CorsairLedId_CLMB_Zone53: CorsairLedId = 1414;
pub const CorsairLedId_CLMB_Zone54: CorsairLedId = 1415;
pub const CorsairLedId_CLMB_Zone55: CorsairLedId = 1416;
pub const CorsairLedId_CLMB_Zone56: CorsairLedId = 1417;
pub const CorsairLedId_CLMB_Zone57: CorsairLedId = 1418;
pub const CorsairLedId_CLMB_Zone58: CorsairLedId = 1419;
pub const CorsairLedId_CLMB_Zone59: CorsairLedId = 1420;
pub const CorsairLedId_CLMB_Zone60: CorsairLedId = 1421;
pub const CorsairLedId_CLMB_Zone61: CorsairLedId = 1422;
pub const CorsairLedId_CLMB_Zone62: CorsairLedId = 1423;
pub const CorsairLedId_CLMB_Zone63: CorsairLedId = 1424;
pub const CorsairLedId_CLMB_Zone64: CorsairLedId = 1425;
pub const CorsairLedId_CLMB_Zone65: CorsairLedId = 1426;
pub const CorsairLedId_CLMB_Zone66: CorsairLedId = 1427;
pub const CorsairLedId_CLMB_Zone67: CorsairLedId = 1428;
pub const CorsairLedId_CLMB_Zone68: CorsairLedId = 1429;
pub const CorsairLedId_CLMB_Zone69: CorsairLedId = 1430;
pub const CorsairLedId_CLMB_Zone70: CorsairLedId = 1431;
pub const CorsairLedId_CLMB_Zone71: CorsairLedId = 1432;
pub const CorsairLedId_CLMB_Zone72: CorsairLedId = 1433;
pub const CorsairLedId_CLMB_Zone73: CorsairLedId = 1434;
pub const CorsairLedId_CLMB_Zone74: CorsairLedId = 1435;
pub const CorsairLedId_CLMB_Zone75: CorsairLedId = 1436;
pub const CorsairLedId_CLMB_Zone76: CorsairLedId = 1437;
pub const CorsairLedId_CLMB_Zone77: CorsairLedId = 1438;
pub const CorsairLedId_CLMB_Zone78: CorsairLedId = 1439;
pub const CorsairLedId_CLMB_Zone79: CorsairLedId = 1440;
pub const CorsairLedId_CLMB_Zone80: CorsairLedId = 1441;
pub const CorsairLedId_CLMB_Zone81: CorsairLedId = 1442;
pub const CorsairLedId_CLMB_Zone82: CorsairLedId = 1443;
pub const CorsairLedId_CLMB_Zone83: CorsairLedId = 1444;
pub const CorsairLedId_CLMB_Zone84: CorsairLedId = 1445;
pub const CorsairLedId_CLMB_Zone85: CorsairLedId = 1446;
pub const CorsairLedId_CLMB_Zone86: CorsairLedId = 1447;
pub const CorsairLedId_CLMB_Zone87: CorsairLedId = 1448;
pub const CorsairLedId_CLMB_Zone88: CorsairLedId = 1449;
pub const CorsairLedId_CLMB_Zone89: CorsairLedId = 1450;
pub const CorsairLedId_CLMB_Zone90: CorsairLedId = 1451;
pub const CorsairLedId_CLMB_Zone91: CorsairLedId = 1452;
pub const CorsairLedId_CLMB_Zone92: CorsairLedId = 1453;
pub const CorsairLedId_CLMB_Zone93: CorsairLedId = 1454;
pub const CorsairLedId_CLMB_Zone94: CorsairLedId = 1455;
pub const CorsairLedId_CLMB_Zone95: CorsairLedId = 1456;
pub const CorsairLedId_CLMB_Zone96: CorsairLedId = 1457;
pub const CorsairLedId_CLMB_Zone97: CorsairLedId = 1458;
pub const CorsairLedId_CLMB_Zone98: CorsairLedId = 1459;
pub const CorsairLedId_CLMB_Zone99: CorsairLedId = 1460;
pub const CorsairLedId_CLMB_Zone100: CorsairLedId = 1461;
pub const CorsairLedId_CLGPU_Zone1: CorsairLedId = 1462;
pub const CorsairLedId_CLGPU_Zone2: CorsairLedId = 1463;
pub const CorsairLedId_CLGPU_Zone3: CorsairLedId = 1464;
pub const CorsairLedId_CLGPU_Zone4: CorsairLedId = 1465;
pub const CorsairLedId_CLGPU_Zone5: CorsairLedId = 1466;
pub const CorsairLedId_CLGPU_Zone6: CorsairLedId = 1467;
pub const CorsairLedId_CLGPU_Zone7: CorsairLedId = 1468;
pub const CorsairLedId_CLGPU_Zone8: CorsairLedId = 1469;
pub const CorsairLedId_CLGPU_Zone9: CorsairLedId = 1470;
pub const CorsairLedId_CLGPU_Zone10: CorsairLedId = 1471;
pub const CorsairLedId_CLGPU_Zone11: CorsairLedId = 1472;
pub const CorsairLedId_CLGPU_Zone12: CorsairLedId = 1473;
pub const CorsairLedId_CLGPU_Zone13: CorsairLedId = 1474;
pub const CorsairLedId_CLGPU_Zone14: CorsairLedId = 1475;
pub const CorsairLedId_CLGPU_Zone15: CorsairLedId = 1476;
pub const CorsairLedId_CLGPU_Zone16: CorsairLedId = 1477;
pub const CorsairLedId_CLGPU_Zone17: CorsairLedId = 1478;
pub const CorsairLedId_CLGPU_Zone18: CorsairLedId = 1479;
pub const CorsairLedId_CLGPU_Zone19: CorsairLedId = 1480;
pub const CorsairLedId_CLGPU_Zone20: CorsairLedId = 1481;
pub const CorsairLedId_CLGPU_Zone21: CorsairLedId = 1482;
pub const CorsairLedId_CLGPU_Zone22: CorsairLedId = 1483;
pub const CorsairLedId_CLGPU_Zone23: CorsairLedId = 1484;
pub const CorsairLedId_CLGPU_Zone24: CorsairLedId = 1485;
pub const CorsairLedId_CLGPU_Zone25: CorsairLedId = 1486;
pub const CorsairLedId_CLGPU_Zone26: CorsairLedId = 1487;
pub const CorsairLedId_CLGPU_Zone27: CorsairLedId = 1488;
pub const CorsairLedId_CLGPU_Zone28: CorsairLedId = 1489;
pub const CorsairLedId_CLGPU_Zone29: CorsairLedId = 1490;
pub const CorsairLedId_CLGPU_Zone30: CorsairLedId = 1491;
pub const CorsairLedId_CLGPU_Zone31: CorsairLedId = 1492;
pub const CorsairLedId_CLGPU_Zone32: CorsairLedId = 1493;
pub const CorsairLedId_CLGPU_Zone33: CorsairLedId = 1494;
pub const CorsairLedId_CLGPU_Zone34: CorsairLedId = 1495;
pub const CorsairLedId_CLGPU_Zone35: CorsairLedId = 1496;
pub const CorsairLedId_CLGPU_Zone36: CorsairLedId = 1497;
pub const CorsairLedId_CLGPU_Zone37: CorsairLedId = 1498;
pub const CorsairLedId_CLGPU_Zone38: CorsairLedId = 1499;
pub const CorsairLedId_CLGPU_Zone39: CorsairLedId = 1500;
pub const CorsairLedId_CLGPU_Zone40: CorsairLedId = 1501;
pub const CorsairLedId_CLGPU_Zone41: CorsairLedId = 1502;
pub const CorsairLedId_CLGPU_Zone42: CorsairLedId = 1503;
pub const CorsairLedId_CLGPU_Zone43: CorsairLedId = 1504;
pub const CorsairLedId_CLGPU_Zone44: CorsairLedId = 1505;
pub const CorsairLedId_CLGPU_Zone45: CorsairLedId = 1506;
pub const CorsairLedId_CLGPU_Zone46: CorsairLedId = 1507;
pub const CorsairLedId_CLGPU_Zone47: CorsairLedId = 1508;
pub const CorsairLedId_CLGPU_Zone48: CorsairLedId = 1509;
pub const CorsairLedId_CLGPU_Zone49: CorsairLedId = 1510;
pub const CorsairLedId_CLGPU_Zone50: CorsairLedId = 1511;
pub const CorsairLedId_CLKLP_Zone20: CorsairLedId = 1512;
pub const CorsairLedId_CLKLP_Zone21: CorsairLedId = 1513;
pub const CorsairLedId_CLKLP_Zone22: CorsairLedId = 1514;
pub const CorsairLedId_CLKLP_Zone23: CorsairLedId = 1515;
pub const CorsairLedId_CLKLP_Zone24: CorsairLedId = 1516;
pub const CorsairLedId_CLKLP_Zone25: CorsairLedId = 1517;
pub const CorsairLedId_CLKLP_Zone26: CorsairLedId = 1518;
pub const CorsairLedId_CLKLP_Zone27: CorsairLedId = 1519;
pub const CorsairLedId_CLKLP_Zone28: CorsairLedId = 1520;
pub const CorsairLedId_CLKLP_Zone29: CorsairLedId = 1521;
pub const CorsairLedId_CLKLP_Zone30: CorsairLedId = 1522;
pub const CorsairLedId_CLKLP_Zone31: CorsairLedId = 1523;
pub const CorsairLedId_CLKLP_Zone32: CorsairLedId = 1524;
pub const CorsairLedId_CLKLP_Zone33: CorsairLedId = 1525;
pub const CorsairLedId_CLKLP_Zone34: CorsairLedId = 1526;
pub const CorsairLedId_CLKLP_Zone35: CorsairLedId = 1527;
pub const CorsairLedId_CLKLP_Zone36: CorsairLedId = 1528;
pub const CorsairLedId_CLKLP_Zone37: CorsairLedId = 1529;
pub const CorsairLedId_CLKLP_Zone38: CorsairLedId = 1530;
pub const CorsairLedId_CLKLP_Zone39: CorsairLedId = 1531;
pub const CorsairLedId_CLKLP_Zone40: CorsairLedId = 1532;
pub const CorsairLedId_CLKLP_Zone41: CorsairLedId = 1533;
pub const CorsairLedId_CLKLP_Zone42: CorsairLedId = 1534;
pub const CorsairLedId_CLKLP_Zone43: CorsairLedId = 1535;
pub const CorsairLedId_CLKLP_Zone44: CorsairLedId = 1536;
pub const CorsairLedId_CLKLP_Zone45: CorsairLedId = 1537;
pub const CorsairLedId_CLKLP_Zone46: CorsairLedId = 1538;
pub const CorsairLedId_CLKLP_Zone47: CorsairLedId = 1539;
pub const CorsairLedId_CLKLP_Zone48: CorsairLedId = 1540;
pub const CorsairLedId_CLKLP_Zone49: CorsairLedId = 1541;
pub const CorsairLedId_CLKLP_Zone50: CorsairLedId = 1542;
pub const CorsairLedId_CLK_Profile: CorsairLedId = 1543;
pub const CorsairLedId_CLI_Last: CorsairLedId = 1543;
pub type CorsairLedId = u32;
pub const CorsairKeyId_CorsairKey_Invalid: CorsairKeyId = 0;
pub const CorsairKeyId_CorsairKeyKb_G1: CorsairKeyId = 1;
pub const CorsairKeyId_CorsairKeyKb_G2: CorsairKeyId = 2;
pub const CorsairKeyId_CorsairKeyKb_G3: CorsairKeyId = 3;
pub const CorsairKeyId_CorsairKeyKb_G4: CorsairKeyId = 4;
pub const CorsairKeyId_CorsairKeyKb_G5: CorsairKeyId = 5;
pub const CorsairKeyId_CorsairKeyKb_G6: CorsairKeyId = 6;
pub const CorsairKeyId_CorsairKeyKb_G7: CorsairKeyId = 7;
pub const CorsairKeyId_CorsairKeyKb_G8: CorsairKeyId = 8;
pub const CorsairKeyId_CorsairKeyKb_G9: CorsairKeyId = 9;
pub const CorsairKeyId_CorsairKeyKb_G10: CorsairKeyId = 10;
pub const CorsairKeyId_CorsairKeyKb_G11: CorsairKeyId = 11;
pub const CorsairKeyId_CorsairKeyKb_G12: CorsairKeyId = 12;
pub const CorsairKeyId_CorsairKeyKb_G13: CorsairKeyId = 13;
pub const CorsairKeyId_CorsairKeyKb_G14: CorsairKeyId = 14;
pub const CorsairKeyId_CorsairKeyKb_G15: CorsairKeyId = 15;
pub const CorsairKeyId_CorsairKeyKb_G16: CorsairKeyId = 16;
pub const CorsairKeyId_CorsairKeyKb_G17: CorsairKeyId = 17;
pub const CorsairKeyId_CorsairKeyKb_G18: CorsairKeyId = 18;
pub const CorsairKeyId_CorsairKeyMouse_M1: CorsairKeyId = 19;
pub const CorsairKeyId_CorsairKeyMouse_M2: CorsairKeyId = 20;
pub const CorsairKeyId_CorsairKeyMouse_M3: CorsairKeyId = 21;
pub const CorsairKeyId_CorsairKeyMouse_M4: CorsairKeyId = 22;
pub const CorsairKeyId_CorsairKeyMouse_M5: CorsairKeyId = 23;
pub const CorsairKeyId_CorsairKeyMouse_M6: CorsairKeyId = 24;
pub const CorsairKeyId_CorsairKeyMouse_M7: CorsairKeyId = 25;
pub const CorsairKeyId_CorsairKeyMouse_M8: CorsairKeyId = 26;
pub const CorsairKeyId_CorsairKeyMouse_M9: CorsairKeyId = 27;
pub const CorsairKeyId_CorsairKeyMouse_M10: CorsairKeyId = 28;
pub const CorsairKeyId_CorsairKeyMouse_M11: CorsairKeyId = 29;
pub const CorsairKeyId_CorsairKeyMouse_M12: CorsairKeyId = 30;
pub const CorsairKeyId_CorsairKey_Last: CorsairKeyId = 30;
pub type CorsairKeyId = u32;
pub const CorsairDeviceType_CDT_Unknown: CorsairDeviceType = 0;
pub const CorsairDeviceType_CDT_Mouse: CorsairDeviceType = 1;
pub const CorsairDeviceType_CDT_Keyboard: CorsairDeviceType = 2;
pub const CorsairDeviceType_CDT_Headset: CorsairDeviceType = 3;
pub const CorsairDeviceType_CDT_MouseMat: CorsairDeviceType = 4;
pub const CorsairDeviceType_CDT_HeadsetStand: CorsairDeviceType = 5;
pub const CorsairDeviceType_CDT_CommanderPro: CorsairDeviceType = 6;
pub const CorsairDeviceType_CDT_LightingNodePro: CorsairDeviceType = 7;
pub const CorsairDeviceType_CDT_MemoryModule: CorsairDeviceType = 8;
pub const CorsairDeviceType_CDT_Cooler: CorsairDeviceType = 9;
pub const CorsairDeviceType_CDT_Motherboard: CorsairDeviceType = 10;
pub const CorsairDeviceType_CDT_GraphicsCard: CorsairDeviceType = 11;
pub type CorsairDeviceType = u32;
pub const CorsairPhysicalLayout_CPL_Invalid: CorsairPhysicalLayout = 0;
pub const CorsairPhysicalLayout_CPL_US: CorsairPhysicalLayout = 1;
pub const CorsairPhysicalLayout_CPL_UK: CorsairPhysicalLayout = 2;
pub const CorsairPhysicalLayout_CPL_BR: CorsairPhysicalLayout = 3;
pub const CorsairPhysicalLayout_CPL_JP: CorsairPhysicalLayout = 4;
pub const CorsairPhysicalLayout_CPL_KR: CorsairPhysicalLayout = 5;
pub const CorsairPhysicalLayout_CPL_Zones1: CorsairPhysicalLayout = 6;
pub const CorsairPhysicalLayout_CPL_Zones2: CorsairPhysicalLayout = 7;
pub const CorsairPhysicalLayout_CPL_Zones3: CorsairPhysicalLayout = 8;
pub const CorsairPhysicalLayout_CPL_Zones4: CorsairPhysicalLayout = 9;
pub type CorsairPhysicalLayout = u32;
pub const CorsairLogicalLayout_CLL_Invalid: CorsairLogicalLayout = 0;
pub const CorsairLogicalLayout_CLL_US_Int: CorsairLogicalLayout = 1;
pub const CorsairLogicalLayout_CLL_NA: CorsairLogicalLayout = 2;
pub const CorsairLogicalLayout_CLL_EU: CorsairLogicalLayout = 3;
pub const CorsairLogicalLayout_CLL_UK: CorsairLogicalLayout = 4;
pub const CorsairLogicalLayout_CLL_BE: CorsairLogicalLayout = 5;
pub const CorsairLogicalLayout_CLL_BR: CorsairLogicalLayout = 6;
pub const CorsairLogicalLayout_CLL_CH: CorsairLogicalLayout = 7;
pub const CorsairLogicalLayout_CLL_CN: CorsairLogicalLayout = 8;
pub const CorsairLogicalLayout_CLL_DE: CorsairLogicalLayout = 9;
pub const CorsairLogicalLayout_CLL_ES: CorsairLogicalLayout = 10;
pub const CorsairLogicalLayout_CLL_FR: CorsairLogicalLayout = 11;
pub const CorsairLogicalLayout_CLL_IT: CorsairLogicalLayout = 12;
pub const CorsairLogicalLayout_CLL_ND: CorsairLogicalLayout = 13;
pub const CorsairLogicalLayout_CLL_RU: CorsairLogicalLayout = 14;
pub const CorsairLogicalLayout_CLL_JP: CorsairLogicalLayout = 15;
pub const CorsairLogicalLayout_CLL_KR: CorsairLogicalLayout = 16;
pub const CorsairLogicalLayout_CLL_TW: CorsairLogicalLayout = 17;
pub const CorsairLogicalLayout_CLL_MEX: CorsairLogicalLayout = 18;
pub type CorsairLogicalLayout = u32;
pub const CorsairDeviceCaps_CDC_None: CorsairDeviceCaps = 0;
pub const CorsairDeviceCaps_CDC_Lighting: CorsairDeviceCaps = 1;
pub const CorsairDeviceCaps_CDC_PropertyLookup: CorsairDeviceCaps = 2;
pub type CorsairDeviceCaps = u32;
pub const CorsairAccessMode_CAM_ExclusiveLightingControl: CorsairAccessMode = 0;
pub type CorsairAccessMode = u32;
pub const CorsairError_CE_Success: CorsairError = 0;
pub const CorsairError_CE_ServerNotFound: CorsairError = 1;
pub const CorsairError_CE_NoControl: CorsairError = 2;
pub const CorsairError_CE_ProtocolHandshakeMissing: CorsairError = 3;
pub const CorsairError_CE_IncompatibleProtocol: CorsairError = 4;
pub const CorsairError_CE_InvalidArguments: CorsairError = 5;
pub type CorsairError = u32;
pub const CorsairChannelDeviceType_CCDT_Invalid: CorsairChannelDeviceType = 0;
pub const CorsairChannelDeviceType_CCDT_HD_Fan: CorsairChannelDeviceType = 1;
pub const CorsairChannelDeviceType_CCDT_SP_Fan: CorsairChannelDeviceType = 2;
pub const CorsairChannelDeviceType_CCDT_LL_Fan: CorsairChannelDeviceType = 3;
pub const CorsairChannelDeviceType_CCDT_ML_Fan: CorsairChannelDeviceType = 4;
pub const CorsairChannelDeviceType_CCDT_Strip: CorsairChannelDeviceType = 5;
pub const CorsairChannelDeviceType_CCDT_DAP: CorsairChannelDeviceType = 6;
pub const CorsairChannelDeviceType_CCDT_Pump: CorsairChannelDeviceType = 7;
pub const CorsairChannelDeviceType_CCDT_QL_Fan: CorsairChannelDeviceType = 8;
pub type CorsairChannelDeviceType = u32;
pub const CorsairDevicePropertyType_CDPT_Boolean: CorsairDevicePropertyType = 4096;
pub const CorsairDevicePropertyType_CDPT_Int32: CorsairDevicePropertyType = 8192;
pub type CorsairDevicePropertyType = u32;
pub const CorsairDevicePropertyId_CDPI_Headset_MicEnabled: CorsairDevicePropertyId = 4096;
pub const CorsairDevicePropertyId_CDPI_Headset_SurroundSoundEnabled: CorsairDevicePropertyId = 4097;
pub const CorsairDevicePropertyId_CDPI_Headset_SidetoneEnabled: CorsairDevicePropertyId = 4098;
pub const CorsairDevicePropertyId_CDPI_Headset_EqualizerPreset: CorsairDevicePropertyId = 8192;
pub type CorsairDevicePropertyId = u32;
pub const CorsairEventId_CEI_Invalid: CorsairEventId = 0;
pub const CorsairEventId_CEI_DeviceConnectionStatusChangedEvent: CorsairEventId = 1;
pub const CorsairEventId_CEI_KeyEvent: CorsairEventId = 2;
pub type CorsairEventId = u32;
pub const CORSAIR_DEVICE_ID_MAX: c_uint = 128;
pub type CorsairDeviceId = [c_char; 128usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CorsairChannelDeviceInfo {
    pub type_: CorsairChannelDeviceType,
    pub deviceLedCount: c_int,
}
#[test]
fn bindgen_test_layout_CorsairChannelDeviceInfo() {
    assert_eq!(
        ::std::mem::size_of::<CorsairChannelDeviceInfo>(),
        8usize,
        concat!("Size of: ", stringify!(CorsairChannelDeviceInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<CorsairChannelDeviceInfo>(),
        4usize,
        concat!("Alignment of ", stringify!(CorsairChannelDeviceInfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairChannelDeviceInfo>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairChannelDeviceInfo),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CorsairChannelDeviceInfo>())).deviceLedCount as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairChannelDeviceInfo),
            "::",
            stringify!(deviceLedCount)
        )
    );
}

unsafe impl Send for CorsairChannelInfo {}
unsafe impl Sync for CorsairChannelInfo {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CorsairChannelInfo {
    pub totalLedsCount: c_int,
    pub devicesCount: c_int,
    pub devices: *mut CorsairChannelDeviceInfo,
}
#[test]
fn bindgen_test_layout_CorsairChannelInfo() {
    assert_eq!(
        ::std::mem::size_of::<CorsairChannelInfo>(),
        16usize,
        concat!("Size of: ", stringify!(CorsairChannelInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<CorsairChannelInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(CorsairChannelInfo))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CorsairChannelInfo>())).totalLedsCount as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairChannelInfo),
            "::",
            stringify!(totalLedsCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairChannelInfo>())).devicesCount as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairChannelInfo),
            "::",
            stringify!(devicesCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairChannelInfo>())).devices as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairChannelInfo),
            "::",
            stringify!(devices)
        )
    );
}

unsafe impl Send for CorsairChannelsInfo {}
unsafe impl Sync for CorsairChannelsInfo {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CorsairChannelsInfo {
    pub channelsCount: c_int,
    pub channels: *mut CorsairChannelInfo,
}
#[test]
fn bindgen_test_layout_CorsairChannelsInfo() {
    assert_eq!(
        ::std::mem::size_of::<CorsairChannelsInfo>(),
        16usize,
        concat!("Size of: ", stringify!(CorsairChannelsInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<CorsairChannelsInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(CorsairChannelsInfo))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CorsairChannelsInfo>())).channelsCount as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairChannelsInfo),
            "::",
            stringify!(channelsCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairChannelsInfo>())).channels as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairChannelsInfo),
            "::",
            stringify!(channels)
        )
    );
}

unsafe impl Send for CorsairDeviceInfo {}
unsafe impl Sync for CorsairDeviceInfo {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CorsairDeviceInfo {
    pub type_: CorsairDeviceType,
    pub model: *const c_char,
    pub physicalLayout: CorsairPhysicalLayout,
    pub logicalLayout: CorsairLogicalLayout,
    pub capsMask: c_int,
    pub ledsCount: c_int,
    pub channels: CorsairChannelsInfo,
    pub deviceId: CorsairDeviceId,
}
#[test]
fn bindgen_test_layout_CorsairDeviceInfo() {
    assert_eq!(
        ::std::mem::size_of::<CorsairDeviceInfo>(),
        176usize,
        concat!("Size of: ", stringify!(CorsairDeviceInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<CorsairDeviceInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(CorsairDeviceInfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairDeviceInfo>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairDeviceInfo),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairDeviceInfo>())).model as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairDeviceInfo),
            "::",
            stringify!(model)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CorsairDeviceInfo>())).physicalLayout as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairDeviceInfo),
            "::",
            stringify!(physicalLayout)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairDeviceInfo>())).logicalLayout as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairDeviceInfo),
            "::",
            stringify!(logicalLayout)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairDeviceInfo>())).capsMask as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairDeviceInfo),
            "::",
            stringify!(capsMask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairDeviceInfo>())).ledsCount as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairDeviceInfo),
            "::",
            stringify!(ledsCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairDeviceInfo>())).channels as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairDeviceInfo),
            "::",
            stringify!(channels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairDeviceInfo>())).deviceId as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairDeviceInfo),
            "::",
            stringify!(deviceId)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CorsairLedPosition {
    pub ledId: CorsairLedId,
    pub top: f64,
    pub left: f64,
    pub height: f64,
    pub width: f64,
}
#[test]
fn bindgen_test_layout_CorsairLedPosition() {
    assert_eq!(
        ::std::mem::size_of::<CorsairLedPosition>(),
        40usize,
        concat!("Size of: ", stringify!(CorsairLedPosition))
    );
    assert_eq!(
        ::std::mem::align_of::<CorsairLedPosition>(),
        8usize,
        concat!("Alignment of ", stringify!(CorsairLedPosition))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairLedPosition>())).ledId as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairLedPosition),
            "::",
            stringify!(ledId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairLedPosition>())).top as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairLedPosition),
            "::",
            stringify!(top)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairLedPosition>())).left as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairLedPosition),
            "::",
            stringify!(left)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairLedPosition>())).height as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairLedPosition),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairLedPosition>())).width as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairLedPosition),
            "::",
            stringify!(width)
        )
    );
}

unsafe impl Send for CorsairLedPositions {}
unsafe impl Sync for CorsairLedPositions {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CorsairLedPositions {
    pub numberOfLed: c_int,
    pub pLedPosition: *mut CorsairLedPosition,
}
#[test]
fn bindgen_test_layout_CorsairLedPositions() {
    assert_eq!(
        ::std::mem::size_of::<CorsairLedPositions>(),
        16usize,
        concat!("Size of: ", stringify!(CorsairLedPositions))
    );
    assert_eq!(
        ::std::mem::align_of::<CorsairLedPositions>(),
        8usize,
        concat!("Alignment of ", stringify!(CorsairLedPositions))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairLedPositions>())).numberOfLed as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairLedPositions),
            "::",
            stringify!(numberOfLed)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CorsairLedPositions>())).pLedPosition as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairLedPositions),
            "::",
            stringify!(pLedPosition)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CorsairLedColor {
    pub ledId: CorsairLedId,
    pub r: c_int,
    pub g: c_int,
    pub b: c_int,
}
#[test]
fn bindgen_test_layout_CorsairLedColor() {
    assert_eq!(
        ::std::mem::size_of::<CorsairLedColor>(),
        16usize,
        concat!("Size of: ", stringify!(CorsairLedColor))
    );
    assert_eq!(
        ::std::mem::align_of::<CorsairLedColor>(),
        4usize,
        concat!("Alignment of ", stringify!(CorsairLedColor))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairLedColor>())).ledId as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairLedColor),
            "::",
            stringify!(ledId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairLedColor>())).r as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairLedColor),
            "::",
            stringify!(r)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairLedColor>())).g as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairLedColor),
            "::",
            stringify!(g)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairLedColor>())).b as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairLedColor),
            "::",
            stringify!(b)
        )
    );
}

unsafe impl Send for CorsairProtocolDetails {}
unsafe impl Sync for CorsairProtocolDetails {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CorsairProtocolDetails {
    pub sdkVersion: *const c_char,
    pub serverVersion: *const c_char,
    pub sdkProtocolVersion: c_int,
    pub serverProtocolVersion: c_int,
    pub breakingChanges: bool,
}
#[test]
fn bindgen_test_layout_CorsairProtocolDetails() {
    assert_eq!(
        ::std::mem::size_of::<CorsairProtocolDetails>(),
        32usize,
        concat!("Size of: ", stringify!(CorsairProtocolDetails))
    );
    assert_eq!(
        ::std::mem::align_of::<CorsairProtocolDetails>(),
        8usize,
        concat!("Alignment of ", stringify!(CorsairProtocolDetails))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CorsairProtocolDetails>())).sdkVersion as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairProtocolDetails),
            "::",
            stringify!(sdkVersion)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CorsairProtocolDetails>())).serverVersion as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairProtocolDetails),
            "::",
            stringify!(serverVersion)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CorsairProtocolDetails>())).sdkProtocolVersion as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairProtocolDetails),
            "::",
            stringify!(sdkProtocolVersion)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CorsairProtocolDetails>())).serverProtocolVersion as *const _
                as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairProtocolDetails),
            "::",
            stringify!(serverProtocolVersion)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CorsairProtocolDetails>())).breakingChanges as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairProtocolDetails),
            "::",
            stringify!(breakingChanges)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CorsairDeviceConnectionStatusChangedEvent {
    pub deviceId: CorsairDeviceId,
    pub isConnected: bool,
}
#[test]
fn bindgen_test_layout_CorsairDeviceConnectionStatusChangedEvent() {
    assert_eq!(
        ::std::mem::size_of::<CorsairDeviceConnectionStatusChangedEvent>(),
        129usize,
        concat!(
            "Size of: ",
            stringify!(CorsairDeviceConnectionStatusChangedEvent)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<CorsairDeviceConnectionStatusChangedEvent>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(CorsairDeviceConnectionStatusChangedEvent)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CorsairDeviceConnectionStatusChangedEvent>())).deviceId
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairDeviceConnectionStatusChangedEvent),
            "::",
            stringify!(deviceId)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CorsairDeviceConnectionStatusChangedEvent>())).isConnected
                as *const _ as usize
        },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairDeviceConnectionStatusChangedEvent),
            "::",
            stringify!(isConnected)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CorsairKeyEvent {
    pub deviceId: CorsairDeviceId,
    pub keyId: CorsairKeyId,
    pub isPressed: bool,
}
#[test]
fn bindgen_test_layout_CorsairKeyEvent() {
    assert_eq!(
        ::std::mem::size_of::<CorsairKeyEvent>(),
        136usize,
        concat!("Size of: ", stringify!(CorsairKeyEvent))
    );
    assert_eq!(
        ::std::mem::align_of::<CorsairKeyEvent>(),
        4usize,
        concat!("Alignment of ", stringify!(CorsairKeyEvent))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairKeyEvent>())).deviceId as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairKeyEvent),
            "::",
            stringify!(deviceId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairKeyEvent>())).keyId as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairKeyEvent),
            "::",
            stringify!(keyId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairKeyEvent>())).isPressed as *const _ as usize },
        132usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairKeyEvent),
            "::",
            stringify!(isPressed)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CorsairEvent {
    pub id: CorsairEventId,
    pub event_union: CorsairEventUnion,
}

unsafe impl Send for CorsairEventUnion {}
unsafe impl Sync for CorsairEventUnion {}

#[repr(C)]
#[derive(Copy, Clone)]
pub union CorsairEventUnion {
    pub deviceConnectionStatusChangedEvent: *const CorsairDeviceConnectionStatusChangedEvent,
    pub keyEvent: *const CorsairKeyEvent,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_CorsairEvent__CorsairEventUnion() {
    assert_eq!(
        ::std::mem::size_of::<CorsairEventUnion>(),
        8usize,
        concat!("Size of: ", stringify!(CorsairEventUnion))
    );
    assert_eq!(
        ::std::mem::align_of::<CorsairEventUnion>(),
        8usize,
        concat!("Alignment of ", stringify!(CorsairEventUnion))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CorsairEventUnion>())).deviceConnectionStatusChangedEvent
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairEventUnion),
            "::",
            stringify!(deviceConnectionStatusChangedEvent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairEventUnion>())).keyEvent as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairEventUnion),
            "::",
            stringify!(keyEvent)
        )
    );
}
#[test]
fn bindgen_test_layout_CorsairEvent() {
    assert_eq!(
        ::std::mem::size_of::<CorsairEvent>(),
        16usize,
        concat!("Size of: ", stringify!(CorsairEvent))
    );
    assert_eq!(
        ::std::mem::align_of::<CorsairEvent>(),
        8usize,
        concat!("Alignment of ", stringify!(CorsairEvent))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CorsairEvent>())).id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CorsairEvent),
            "::",
            stringify!(id)
        )
    );
}

pub type CorsairEventHandler =
    ::std::option::Option<unsafe extern "C" fn(context: *mut c_void, event: *const CorsairEvent)>;

extern "C" {
    #[must_use]
    pub fn CorsairSetLedsColors(size: c_int, ledsColors: *mut CorsairLedColor) -> bool;
    #[must_use]
    pub fn CorsairSetLedsColorsBufferByDeviceIndex(
        deviceIndex: c_int,
        size: c_int,
        ledsColors: *mut CorsairLedColor,
    ) -> bool;
    #[must_use]
    pub fn CorsairSetLedsColorsFlushBuffer() -> bool;
    #[must_use]
    pub fn CorsairSetLedsColorsFlushBufferAsync(
        callback: ::std::option::Option<
            unsafe extern "C" fn(context: *mut c_void, result: bool, error: CorsairError),
        >,
        context: *mut c_void,
    ) -> bool;
    #[must_use]
    pub fn CorsairGetLedsColors(size: c_int, ledsColors: *mut CorsairLedColor) -> bool;
    #[must_use]
    pub fn CorsairGetLedsColorsByDeviceIndex(
        deviceIndex: c_int,
        size: c_int,
        ledsColors: *mut CorsairLedColor,
    ) -> bool;
    #[must_use]
    pub fn CorsairSetLedsColorsAsync(
        size: c_int,
        ledsColors: *mut CorsairLedColor,
        CallbackType: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut c_void, arg2: bool, arg3: CorsairError),
        >,
        context: *mut c_void,
    ) -> bool;
    pub fn CorsairGetDeviceCount() -> c_int;
    pub fn CorsairGetDeviceInfo(deviceIndex: c_int) -> *mut CorsairDeviceInfo;
    pub fn CorsairGetLedPositions() -> *mut CorsairLedPositions;
    pub fn CorsairGetLedPositionsByDeviceIndex(deviceIndex: c_int) -> *mut CorsairLedPositions;
    pub fn CorsairGetLedIdForKeyName(keyName: c_char) -> CorsairLedId;
    #[must_use]
    pub fn CorsairRequestControl(accessMode: CorsairAccessMode) -> bool;
    pub fn CorsairPerformProtocolHandshake() -> CorsairProtocolDetails;
    pub fn CorsairGetLastError() -> CorsairError;
    #[must_use]
    pub fn CorsairReleaseControl(accessMode: CorsairAccessMode) -> bool;
    #[must_use]
    pub fn CorsairSetLayerPriority(priority: c_int) -> bool;
    #[must_use]
    pub fn CorsairRegisterKeypressCallback(
        CallbackType: ::std::option::Option<
            unsafe extern "C" fn(context: *mut c_void, keyId: CorsairKeyId, pressed: bool),
        >,
        context: *mut c_void,
    ) -> bool;
    #[must_use]
    pub fn CorsairGetBoolPropertyValue(
        deviceIndex: c_int,
        propertyId: CorsairDevicePropertyId,
        propertyValue: *mut bool,
    ) -> bool;
    #[must_use]
    pub fn CorsairGetInt32PropertyValue(
        deviceIndex: c_int,
        propertyId: CorsairDevicePropertyId,
        propertyValue: *mut c_int,
    ) -> bool;
    #[must_use]
    pub fn CorsairSubscribeForEvents(onEvent: CorsairEventHandler, context: *mut c_void) -> bool;
    #[must_use]
    pub fn CorsairUnsubscribeFromEvents() -> bool;
}
