//  notty is a new kind of terminal emulator.
//  Copyright (C) 2015 without boats
//  
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU Affero General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//  
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU Affero General Public License for more details.
//  
//  You should have received a copy of the GNU Affero General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.
use datatypes::Color;

// FONTS

pub static FONT: &'static str = "Inconsolatazi4 10";

// LOGGING

pub static LOGFILE: &'static str = "~/.log/notty";

// SCOLLBACK

pub static SCROLLBACK: u32 = 512;

// TABS

pub static TAB_STOP: u32 = 4;

// COLORS

pub static DEFAULT_FG: Color = Color(0xff,0xff,0xff);
pub static DEFAULT_BG: Color = Color(0x00,0x00,0x00);

pub static CURSOR_COLOR: Color = Color(0xbb,0xbb,0xbb);

pub static COLORS_256: [Color; 256] = [
    /*  0    */ Color(0x00,0x00,0x00),
    /*  1    */ Color(0x55,0x55,0xff),
    /*  2    */ Color(0x55,0xff,0x55),
    /*  3    */ Color(0x55,0xff,0xff),
    /*  4    */ Color(0xff,0x55,0x55),
    /*  5    */ Color(0xff,0x55,0xff),
    /*  6    */ Color(0xff,0xff,0x55),
    /*  7    */ Color(0xbb,0xbb,0xbb),
    /*  8    */ Color(0x55,0x55,0x55),
    /*  9    */ Color(0x55,0x55,0xff),
    /*  10   */ Color(0x55,0xff,0x55),
    /*  11   */ Color(0x55,0xff,0xff),
    /*  12   */ Color(0xff,0x55,0x55),
    /*  13   */ Color(0xff,0x55,0xff),
    /*  14   */ Color(0xff,0xff,0x55),
    /*  15   */ Color(0xff,0xff,0xff),
    /*  16   */ Color(0x00,0x00,0x00),
    /*  17   */ Color(0x00,0x00,0x5f),
    /*  18   */ Color(0x00,0x00,0x87),
    /*  19   */ Color(0x00,0x00,0xaf),
    /*  20   */ Color(0x00,0x00,0xd7),
    /*  21   */ Color(0x00,0x00,0xff),
    /*  22   */ Color(0x00,0x5f,0x00),
    /*  23   */ Color(0x00,0x5f,0x5f),
    /*  24   */ Color(0x00,0x5f,0x87),
    /*  25   */ Color(0x00,0x5f,0xaf),
    /*  26   */ Color(0x00,0x5f,0xd7),
    /*  27   */ Color(0x00,0x5f,0xff),
    /*  28   */ Color(0x00,0x87,0x00),
    /*  29   */ Color(0x00,0x87,0x5f),
    /*  30   */ Color(0x00,0x87,0x87),
    /*  31   */ Color(0x00,0x87,0xaf),
    /*  32   */ Color(0x00,0x87,0xd7),
    /*  33   */ Color(0x00,0x87,0xff),
    /*  34   */ Color(0x00,0xaf,0x00),
    /*  35   */ Color(0x00,0xaf,0x5f),
    /*  36   */ Color(0x00,0xaf,0x87),
    /*  37   */ Color(0x00,0xaf,0xaf),
    /*  38   */ Color(0x00,0xaf,0xd7),
    /*  39   */ Color(0x00,0xaf,0xff),
    /*  40   */ Color(0x00,0xd7,0x00),
    /*  41   */ Color(0x00,0xd7,0x5f),
    /*  42   */ Color(0x00,0xd7,0x87),
    /*  43   */ Color(0x00,0xd7,0xaf),
    /*  44   */ Color(0x00,0xd7,0xd7),
    /*  45   */ Color(0x00,0xd7,0xff),
    /*  46   */ Color(0x00,0xff,0x00),
    /*  47   */ Color(0x00,0xff,0x5f),
    /*  48   */ Color(0x00,0xff,0x87),
    /*  49   */ Color(0x00,0xff,0xaf),
    /*  50   */ Color(0x00,0xff,0xd7),
    /*  51   */ Color(0x00,0xff,0xff),
    /*  52   */ Color(0x5f,0x00,0x00),
    /*  53   */ Color(0x5f,0x00,0x5f),
    /*  54   */ Color(0x5f,0x00,0x87),
    /*  55   */ Color(0x5f,0x00,0xaf),
    /*  56   */ Color(0x5f,0x00,0xd7),
    /*  57   */ Color(0x5f,0x00,0xff),
    /*  58   */ Color(0x5f,0x5f,0x00),
    /*  59   */ Color(0x5f,0x5f,0x5f),
    /*  60   */ Color(0x5f,0x5f,0x87),
    /*  61   */ Color(0x5f,0x5f,0xaf),
    /*  62   */ Color(0x5f,0x5f,0xd7),
    /*  63   */ Color(0x5f,0x5f,0xff),
    /*  64   */ Color(0x5f,0x87,0x00),
    /*  65   */ Color(0x5f,0x87,0x5f),
    /*  66   */ Color(0x5f,0x87,0x87),
    /*  67   */ Color(0x5f,0x87,0xaf),
    /*  68   */ Color(0x5f,0x87,0xd7),
    /*  69   */ Color(0x5f,0x87,0xff),
    /*  70   */ Color(0x5f,0xaf,0x00),
    /*  71   */ Color(0x5f,0xaf,0x5f),
    /*  72   */ Color(0x5f,0xaf,0x87),
    /*  73   */ Color(0x5f,0xaf,0xaf),
    /*  74   */ Color(0x5f,0xaf,0xd7),
    /*  75   */ Color(0x5f,0xaf,0xff),
    /*  76   */ Color(0x5f,0xd7,0x00),
    /*  77   */ Color(0x5f,0xd7,0x5f),
    /*  78   */ Color(0x5f,0xd7,0x87),
    /*  79   */ Color(0x5f,0xd7,0xaf),
    /*  80   */ Color(0x5f,0xd7,0xd7),
    /*  81   */ Color(0x5f,0xd7,0xff),
    /*  82   */ Color(0x5f,0xff,0x00),
    /*  83   */ Color(0x5f,0xff,0x5f),
    /*  84   */ Color(0x5f,0xff,0x87),
    /*  85   */ Color(0x5f,0xff,0xaf),
    /*  86   */ Color(0x5f,0xff,0xd7),
    /*  87   */ Color(0x5f,0xff,0xff),
    /*  88   */ Color(0x87,0x00,0x00),
    /*  89   */ Color(0x87,0x00,0x5f),
    /*  90   */ Color(0x87,0x00,0x87),
    /*  91   */ Color(0x87,0x00,0xaf),
    /*  92   */ Color(0x87,0x00,0xd7),
    /*  93   */ Color(0x87,0x00,0xff),
    /*  94   */ Color(0x87,0x5f,0x00),
    /*  95   */ Color(0x87,0x5f,0x5f),
    /*  96   */ Color(0x87,0x5f,0x87),
    /*  97   */ Color(0x87,0x5f,0xaf),
    /*  98   */ Color(0x87,0x5f,0xd7),
    /*  99   */ Color(0x87,0x5f,0xff),
    /*  100  */ Color(0x87,0x87,0x00),
    /*  101  */ Color(0x87,0x87,0x5f),
    /*  102  */ Color(0x87,0x87,0x87),
    /*  103  */ Color(0x87,0x87,0xaf),
    /*  104  */ Color(0x87,0x87,0xd7),
    /*  105  */ Color(0x87,0x87,0xff),
    /*  106  */ Color(0x87,0xaf,0x00),
    /*  107  */ Color(0x87,0xaf,0x5f),
    /*  108  */ Color(0x87,0xaf,0x87),
    /*  109  */ Color(0x87,0xaf,0xaf),
    /*  110  */ Color(0x87,0xaf,0xd7),
    /*  111  */ Color(0x87,0xaf,0xff),
    /*  112  */ Color(0x87,0xd7,0x00),
    /*  113  */ Color(0x87,0xd7,0x5f),
    /*  114  */ Color(0x87,0xd7,0x87),
    /*  115  */ Color(0x87,0xd7,0xaf),
    /*  116  */ Color(0x87,0xd7,0xd7),
    /*  117  */ Color(0x87,0xd7,0xff),
    /*  118  */ Color(0x87,0xff,0x00),
    /*  119  */ Color(0x87,0xff,0x5f),
    /*  120  */ Color(0x87,0xff,0x87),
    /*  121  */ Color(0x87,0xff,0xaf),
    /*  122  */ Color(0x87,0xff,0xd7),
    /*  123  */ Color(0x87,0xff,0xff),
    /*  124  */ Color(0xaf,0x00,0x00),
    /*  125  */ Color(0xaf,0x00,0x5f),
    /*  126  */ Color(0xaf,0x00,0x87),
    /*  127  */ Color(0xaf,0x00,0xaf),
    /*  128  */ Color(0xaf,0x00,0xd7),
    /*  129  */ Color(0xaf,0x00,0xff),
    /*  130  */ Color(0xaf,0x5f,0x00),
    /*  131  */ Color(0xaf,0x5f,0x5f),
    /*  132  */ Color(0xaf,0x5f,0x87),
    /*  133  */ Color(0xaf,0x5f,0xaf),
    /*  134  */ Color(0xaf,0x5f,0xd7),
    /*  135  */ Color(0xaf,0x5f,0xff),
    /*  136  */ Color(0xaf,0x87,0x00),
    /*  137  */ Color(0xaf,0x87,0x5f),
    /*  138  */ Color(0xaf,0x87,0x87),
    /*  139  */ Color(0xaf,0x87,0xaf),
    /*  140  */ Color(0xaf,0x87,0xd7),
    /*  141  */ Color(0xaf,0x87,0xff),
    /*  142  */ Color(0xaf,0xaf,0x00),
    /*  143  */ Color(0xaf,0xaf,0x5f),
    /*  144  */ Color(0xaf,0xaf,0x87),
    /*  145  */ Color(0xaf,0xaf,0xaf),
    /*  146  */ Color(0xaf,0xaf,0xd7),
    /*  147  */ Color(0xaf,0xaf,0xff),
    /*  148  */ Color(0xaf,0xd7,0x00),
    /*  149  */ Color(0xaf,0xd7,0x5f),
    /*  150  */ Color(0xaf,0xd7,0x87),
    /*  151  */ Color(0xaf,0xd7,0xaf),
    /*  152  */ Color(0xaf,0xd7,0xd7),
    /*  153  */ Color(0xaf,0xd7,0xff),
    /*  154  */ Color(0xaf,0xff,0x00),
    /*  155  */ Color(0xaf,0xff,0x5f),
    /*  156  */ Color(0xaf,0xff,0x87),
    /*  157  */ Color(0xaf,0xff,0xaf),
    /*  158  */ Color(0xaf,0xff,0xd7),
    /*  159  */ Color(0xaf,0xff,0xff),
    /*  160  */ Color(0xd7,0x00,0x00),
    /*  161  */ Color(0xd7,0x00,0x5f),
    /*  162  */ Color(0xd7,0x00,0x87),
    /*  163  */ Color(0xd7,0x00,0xaf),
    /*  164  */ Color(0xd7,0x00,0xd7),
    /*  165  */ Color(0xd7,0x00,0xff),
    /*  166  */ Color(0xd7,0x5f,0x00),
    /*  167  */ Color(0xd7,0x5f,0x5f),
    /*  168  */ Color(0xd7,0x5f,0x87),
    /*  169  */ Color(0xd7,0x5f,0xaf),
    /*  170  */ Color(0xd7,0x5f,0xd7),
    /*  171  */ Color(0xd7,0x5f,0xff),
    /*  172  */ Color(0xd7,0x87,0x00),
    /*  173  */ Color(0xd7,0x87,0x5f),
    /*  174  */ Color(0xd7,0x87,0x87),
    /*  175  */ Color(0xd7,0x87,0xaf),
    /*  176  */ Color(0xd7,0x87,0xd7),
    /*  177  */ Color(0xd7,0x87,0xff),
    /*  178  */ Color(0xd7,0xaf,0x00),
    /*  179  */ Color(0xd7,0xaf,0x5f),
    /*  180  */ Color(0xd7,0xaf,0x87),
    /*  181  */ Color(0xd7,0xaf,0xaf),
    /*  182  */ Color(0xd7,0xaf,0xd7),
    /*  183  */ Color(0xd7,0xaf,0xff),
    /*  184  */ Color(0xd7,0xd7,0x00),
    /*  185  */ Color(0xd7,0xd7,0x5f),
    /*  186  */ Color(0xd7,0xd7,0x87),
    /*  187  */ Color(0xd7,0xd7,0xaf),
    /*  188  */ Color(0xd7,0xd7,0xd7),
    /*  189  */ Color(0xd7,0xd7,0xff),
    /*  190  */ Color(0xd7,0xff,0x00),
    /*  191  */ Color(0xd7,0xff,0x5f),
    /*  192  */ Color(0xd7,0xff,0x87),
    /*  193  */ Color(0xd7,0xff,0xaf),
    /*  194  */ Color(0xd7,0xff,0xd7),
    /*  195  */ Color(0xd7,0xff,0xff),
    /*  196  */ Color(0xff,0x00,0x00),
    /*  197  */ Color(0xff,0x00,0x5f),
    /*  198  */ Color(0xff,0x00,0x87),
    /*  199  */ Color(0xff,0x00,0xaf),
    /*  200  */ Color(0xff,0x00,0xd7),
    /*  201  */ Color(0xff,0x00,0xff),
    /*  202  */ Color(0xff,0x5f,0x00),
    /*  203  */ Color(0xff,0x5f,0x5f),
    /*  204  */ Color(0xff,0x5f,0x87),
    /*  205  */ Color(0xff,0x5f,0xaf),
    /*  206  */ Color(0xff,0x5f,0xd7),
    /*  207  */ Color(0xff,0x5f,0xff),
    /*  208  */ Color(0xff,0x87,0x00),
    /*  209  */ Color(0xff,0x87,0x5f),
    /*  210  */ Color(0xff,0x87,0x87),
    /*  211  */ Color(0xff,0x87,0xaf),
    /*  212  */ Color(0xff,0x87,0xd7),
    /*  213  */ Color(0xff,0x87,0xff),
    /*  214  */ Color(0xff,0xaf,0x00),
    /*  215  */ Color(0xff,0xaf,0x5f),
    /*  216  */ Color(0xff,0xaf,0x87),
    /*  217  */ Color(0xff,0xaf,0xaf),
    /*  218  */ Color(0xff,0xaf,0xd7),
    /*  219  */ Color(0xff,0xaf,0xff),
    /*  220  */ Color(0xff,0xd7,0x00),
    /*  221  */ Color(0xff,0xd7,0x5f),
    /*  222  */ Color(0xff,0xd7,0x87),
    /*  223  */ Color(0xff,0xd7,0xaf),
    /*  224  */ Color(0xff,0xd7,0xd7),
    /*  225  */ Color(0xff,0xd7,0xff),
    /*  226  */ Color(0xff,0xff,0x00),
    /*  227  */ Color(0xff,0xff,0x5f),
    /*  228  */ Color(0xff,0xff,0x87),
    /*  229  */ Color(0xff,0xff,0xaf),
    /*  230  */ Color(0xff,0xff,0xd7),
    /*  231  */ Color(0xff,0xff,0xff),
    /*  232  */ Color(0x08,0x08,0x08),
    /*  233  */ Color(0x12,0x12,0x12),
    /*  234  */ Color(0x1c,0x1c,0x1c),
    /*  235  */ Color(0x26,0x26,0x26),
    /*  236  */ Color(0x30,0x30,0x30),
    /*  237  */ Color(0x3a,0x3a,0x3a),
    /*  238  */ Color(0x44,0x44,0x44),
    /*  239  */ Color(0x4e,0x4e,0x4e),
    /*  240  */ Color(0x58,0x58,0x58),
    /*  241  */ Color(0x60,0x60,0x60),
    /*  242  */ Color(0x66,0x66,0x66),
    /*  243  */ Color(0x76,0x76,0x76),
    /*  244  */ Color(0x80,0x80,0x80),
    /*  245  */ Color(0x8a,0x8a,0x8a),
    /*  246  */ Color(0x94,0x94,0x94),
    /*  247  */ Color(0x9e,0x9e,0x9e),
    /*  248  */ Color(0xa8,0xa8,0xa8),
    /*  249  */ Color(0xb2,0xb2,0xb2),
    /*  250  */ Color(0xbc,0xbc,0xbc),
    /*  251  */ Color(0xc6,0xc6,0xc6),
    /*  252  */ Color(0xd0,0xd0,0xd0),
    /*  253  */ Color(0xda,0xda,0xda),
    /*  254  */ Color(0xe4,0xe4,0xe4),
    /*  255  */ Color(0xee,0xee,0xee),
];
