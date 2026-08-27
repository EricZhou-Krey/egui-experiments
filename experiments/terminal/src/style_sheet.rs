pub const ICON: &str = "
                             ...                                      
                        .-+#@#-                                       
              :#%%%##+*@@@@@=                   :....  .              
              +@@@@#. %@@@%.                   .=--..:..... .         
              +@@=    %@@*                     .=:.---:....           
              :=      %@*                       :-===---.             
            .#@.      @#.                      :+++===-.              
           -@@@.      +-                      =***+++==-:::.          
          -@@@@.                          :+*%###***++=-:             
         =@@@@@.     =:                 =%@@@%%%###*=-=+=:            
        -@@@@@@.     #+               =@@@@@@@@%%###*+:  :-.          
       .@@@@@@@.     #+             -@@@@@@@@@@@%%#++**+:             
       +@@@@@@@.     #+       :=:. @@@@@@@@@@@@@@@%=   -*=.           
      .@@@@@@@@.     #+  -@@@@@@@@@@@@@@@@@@@@@@@@@%%=   .=.          
      =@@@@@@@@.     =-  ..-+@@@@@@@@@@@@@@@@@@@@@@@@%+.              
      *@@@@@@@@.               @@@@@@@@@@@@@@@@@@@@@=-##:             
     .@@@@@@@@@.      -:  :+#@@@@@@@@@@@@@@@@@@@@@@@@+.:+:            
     .@@@@@@@@@.      =+#@@@@@@@@@@@@@@@@@@@@@@@@@#=+%*. -:           
      %@@@@@@@@.     -@@@@@@@@@@@@@@@@@@@@@@@@@@@@*=-:*=              
      +@@@@@@@@.   .#@@@@@@@@@@@@@@@@@@@@@@@:#=*@#%=-=--.             
      -@@@@@@@@.  -@@@@@@@@@@@@@@@@@@@@@@@@@+@@@*%+*:+@+              
      .@@@@@@@%:=@@@@@@@@@@@@@@@@@@@@@@@@@@@#@@@%+#-%@@+              
       =@@@@**@@@@@@@@@@@@@+@@@@-=@@@@@@@@@% ..+@%=*##@+              
        +#=@@-..%@@@@@@@@@+=@-@-@@:@@@@@@@@+   =@@.:-@@+              
         :+@@%*@@@#@@@@@#: %:    .:@@@@@@@@:   =@@@@@@@*              
         :@@+@@-:.*@*.@@+         +@@@@@@@@=   =@@@@@@@*      =%.     
          .+%**   #@: @%.        =@@@@@@@@@@ . =@@@@@@@#=%@@@@@:      
         .++=@@.  ##           :-@@@@@@@@@@@%*@@.      %@@@@@%:       
        .*=  =*.  ##          #-@@@@@@@@@@@%#*@@.      %@@@@+         
       :%-    :+: ##        :@#@@@@@@@@@@@@%@@@@.      %@@%.          
      :@:      #@%#*       =@@+@@@@@@@@@@@@%#@@@.      *#.            
     .@.       .%@@-..    %@@@@*@@+@:@@+-:*@@@@@.    =@*              
     *.          *#:@@@*-@@@@@@=@-@@=@@=@@@@@@@@..+@@@@*              
    :=             .--:.  .=@@#@-@@@*@@#@@@@@@+:-======.              
    +                         .#..::.@=...                            
                              *.     #.                               
                              +      #.                               
                                     +=                               
";

pub const NEOFETCH_SPECS: &[&str] = &[
    "\x1b[90m-------------------\x1b[0m",
    "\x1b[1;36mOS\x1b[0m: \x1b[3mRook OS (Avian Edition)\x1b[0m",
    "\x1b[1;36mHost\x1b[0m: Twig-Based Nest Architecture v4.2",
    "\x1b[1;36mKernel\x1b[0m: 6.9.0-telepathic-flock",
    "\x1b[1;36mUptime\x1b[0m: 3 months since last migration",
    "\x1b[1;36mPackages\x1b[0m: 9001 (seeds), 42 (worms)",
    "\x1b[1;36mShell\x1b[0m: beak-sh (loud squawk mode)",
    "\x1b[1;36mResolution\x1b[0m: 8K Ultraviolet Avian Vision",
    "\x1b[1;36mDE\x1b[0m: Branch-top Environment",
    "\x1b[1;36mWM\x1b[0m: FeatherWM (Drafty)",
    "\x1b[1;36mWM Theme\x1b[0m: Iridescent Plumage",
    "\x1b[1;36mTheme\x1b[0m: Corvidae-Goth [Twigs3]",
    "\x1b[1;36mIcons\x1b[0m: Stolen-Shiny-Wrappers",
    "\x1b[1;36mTerminal\x1b[0m: rook-term (squawk emulator)",
    "\x1b[1;36mTerminal Font\x1b[0m: Crows Foot Sans 14",
    "\x1b[1;36mCPU\x1b[0m: 16x Hamster Wheel Core @ 1.2 Seeds/sec",
    "\x1b[1;36mGPU\x1b[0m: 2x Shiny Bottle Caps in SLI",
    "\x1b[1;36mMemory\x1b[0m: 3 Acorns / 10 Acorns Total",
    "\x1b[1;36mDisk (/)\x1b[0m: 1.2 Sticks / 5 Sticks (Birdhouse Full)",
    "\x1b[1;36mNetwork\x1b[0m: IP over Avian Carriers (RFC 1149)",
    "\x1b[1;36mBattery\x1b[0m: \x1b[32m100%\x1b[0m [Powered by Breadcrumbs]",
    "",
    "\x1b[40m   \x1b[41m   \x1b[42m   \x1b[43m   \x1b[44m   \x1b[45m   \x1b[46m   \x1b[47m   \x1b[0m",
    "\x1b[100m   \x1b[101m   \x1b[102m   \x1b[103m   \x1b[104m   \x1b[105m   \x1b[106m   \x1b[107m   \x1b[0m",
];

pub const ICON_PADDING: usize = 4;
pub const TEXT_COLOR: egui::Color32 = egui::Color32::from_rgb(200, 200, 200);
pub const PROMPT_TEXT_COLOR: egui::Color32 = egui::Color32::from_rgb(80, 250, 120);
pub const SELECTION_COLOR: egui::Color32 =
    egui::Color32::from_rgba_premultiplied(100, 100, 100, 100);
pub const BACKGROUND_CORNER_RADIUS: f32 = 0.0;
pub const BACKGROUND_COLOR: egui::Color32 = egui::Color32::from_rgb(15, 15, 15);
pub const TEXT_STYLE: egui::TextStyle = egui::TextStyle::Monospace;
pub const USER: &str = "bird";
pub const HOST: &str = "rook-os";

pub const ANSI_BLACK: egui::Color32 = egui::Color32::from_rgb(0, 0, 0);
pub const ANSI_RED: egui::Color32 = egui::Color32::from_rgb(204, 36, 29);
pub const ANSI_GREEN: egui::Color32 = egui::Color32::from_rgb(152, 151, 26);
pub const ANSI_YELLOW: egui::Color32 = egui::Color32::from_rgb(215, 153, 33);
pub const ANSI_BLUE: egui::Color32 = egui::Color32::from_rgb(69, 133, 136);
pub const ANSI_MAGENTA: egui::Color32 = egui::Color32::from_rgb(177, 98, 134);
pub const ANSI_CYAN: egui::Color32 = egui::Color32::from_rgb(104, 157, 106);
pub const ANSI_WHITE: egui::Color32 = egui::Color32::from_rgb(235, 219, 178);

pub const ANSI_BRIGHT_BLACK: egui::Color32 = egui::Color32::from_rgb(146, 131, 116);
pub const ANSI_BRIGHT_RED: egui::Color32 = egui::Color32::from_rgb(251, 73, 52);
pub const ANSI_BRIGHT_GREEN: egui::Color32 = egui::Color32::from_rgb(184, 187, 38);
pub const ANSI_BRIGHT_YELLOW: egui::Color32 = egui::Color32::from_rgb(250, 189, 47);
pub const ANSI_BRIGHT_BLUE: egui::Color32 = egui::Color32::from_rgb(131, 165, 152);
pub const ANSI_BRIGHT_MAGENTA: egui::Color32 = egui::Color32::from_rgb(211, 134, 155);
pub const ANSI_BRIGHT_CYAN: egui::Color32 = egui::Color32::from_rgb(142, 192, 124);
pub const ANSI_BRIGHT_WHITE: egui::Color32 = egui::Color32::from_rgb(253, 244, 193);

pub fn apply_ansi_code(
    code_string: &str,
    text_format: &mut egui::text::TextFormat,
    default_text_color: egui::Color32,
    default_background_color: egui::Color32,
) {
    match code_string {
        "0" => {
            text_format.color = default_text_color;
            text_format.background = default_background_color;
            text_format.italics = false;
            text_format.underline = egui::Stroke::NONE;
            text_format.strikethrough = egui::Stroke::NONE;
        }
        "1" => {}
        "3" => {
            text_format.italics = true;
        }
        "4" => {
            text_format.underline = egui::Stroke::new(1.0, text_format.color);
        }
        "9" => {
            text_format.strikethrough = egui::Stroke::new(1.0, text_format.color);
        }
        "22" => {}
        "23" => {
            text_format.italics = false;
        }
        "24" => {
            text_format.underline = egui::Stroke::NONE;
        }
        "29" => {
            text_format.strikethrough = egui::Stroke::NONE;
        }
        "30" => {
            text_format.color = ANSI_BLACK;
        }
        "31" => {
            text_format.color = ANSI_RED;
        }
        "32" => {
            text_format.color = ANSI_GREEN;
        }
        "33" => {
            text_format.color = ANSI_YELLOW;
        }
        "34" => {
            text_format.color = ANSI_BLUE;
        }
        "35" => {
            text_format.color = ANSI_MAGENTA;
        }
        "36" => {
            text_format.color = ANSI_CYAN;
        }
        "37" => {
            text_format.color = ANSI_WHITE;
        }
        "39" => {
            text_format.color = default_text_color;
        }
        "40" => {
            text_format.background = ANSI_BLACK;
        }
        "41" => {
            text_format.background = ANSI_RED;
        }
        "42" => {
            text_format.background = ANSI_GREEN;
        }
        "43" => {
            text_format.background = ANSI_YELLOW;
        }
        "44" => {
            text_format.background = ANSI_BLUE;
        }
        "45" => {
            text_format.background = ANSI_MAGENTA;
        }
        "46" => {
            text_format.background = ANSI_CYAN;
        }
        "47" => {
            text_format.background = ANSI_WHITE;
        }
        "49" => {
            text_format.background = default_background_color;
        }
        "90" => {
            text_format.color = ANSI_BRIGHT_BLACK;
        }
        "91" => {
            text_format.color = ANSI_BRIGHT_RED;
        }
        "92" => {
            text_format.color = ANSI_BRIGHT_GREEN;
        }
        "93" => {
            text_format.color = ANSI_BRIGHT_YELLOW;
        }
        "94" => {
            text_format.color = ANSI_BRIGHT_BLUE;
        }
        "95" => {
            text_format.color = ANSI_BRIGHT_MAGENTA;
        }
        "96" => {
            text_format.color = ANSI_BRIGHT_CYAN;
        }
        "97" => {
            text_format.color = ANSI_BRIGHT_WHITE;
        }
        "100" => {
            text_format.background = ANSI_BRIGHT_BLACK;
        }
        "101" => {
            text_format.background = ANSI_BRIGHT_RED;
        }
        "102" => {
            text_format.background = ANSI_BRIGHT_GREEN;
        }
        "103" => {
            text_format.background = ANSI_BRIGHT_YELLOW;
        }
        "104" => {
            text_format.background = ANSI_BRIGHT_BLUE;
        }
        "105" => {
            text_format.background = ANSI_BRIGHT_MAGENTA;
        }
        "106" => {
            text_format.background = ANSI_BRIGHT_CYAN;
        }
        "107" => {
            text_format.background = ANSI_BRIGHT_WHITE;
        }
        _ => {}
    }
}
