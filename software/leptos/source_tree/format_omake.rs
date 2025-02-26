pub fn format_string(s:&String) -> String {
    let colormap= HashMap::from([

("Color_Off",r"\033[0m"), // Text Reset

    // Regular Colors
("Black",r"\033[0;30m"), // Black
("Red",r"\033[0;31m"), // Red
("Green",r"\033[0;32m"), // Green
("Yellow",r"\033[0;33m"), // Yellow
("Blue",r"\033[0;34m"), // Blue
("Purple",r"\033[0;35m"), // Purple
("Cyan",r"\033[0;36m"), // Cyan
("White",r"\033[0;37m"), // White

    // Bold
("BBlack",r"\033[1;30m"), // Black
("BRed",r"\033[1;31m"), // Red
("BGreen",r"\033[1;32m"), // Green
("BYellow",r"\033[1;33m"), // Yellow
("BBlue",r"\033[1;34m"), // Blue
("BPurple",r"\033[1;35m"), // Purple
("BCyan",r"\033[1;36m"), // Cyan
("BWhite",r"\033[1;37m"), // White

    // Underline
("UBlack",r"\033[4;30m"), // Black
("URed",r"\033[4;31m"), // Red
("UGreen",r"\033[4;32m"), // Green
("UYellow",r"\033[4;33m"), // Yellow
("UBlue",r"\033[4;34m"), // Blue
("UPurple",r"\033[4;35m"), // Purple
("UCyan",r"\033[4;36m"), // Cyan
("UWhite",r"\033[4;37m"), // White

    // Background
("On_Black",r"\033[40m"), // Black
("On_Red",r"\033[41m"), // Red
("On_Green",r"\033[42m"), // Green
("On_Yellow",r"\033[43m"), // Yellow
("On_Blue",r"\033[44m"), // Blue
("On_Purple",r"\033[45m"), // Purple
("On_Cyan",r"\033[46m"), // Cyan
("On_White",r"\033[47m"), // White

    // High Intensity
("IBlack",r"\033[0;90m"), // Black
("IRed",r"\033[0;91m"), // Red
("IGreen",r"\033[0;92m"), // Green
("IYellow",r"\033[0;93m"), // Yellow
("IBlue",r"\033[0;94m"), // Blue
("IPurple",r"\033[0;95m"), // Purple
("ICyan",r"\033[0;96m"), // Cyan
("IWhite",r"\033[0;97m"), // White

    // Bold High Intensity
("BIBlack",r"\033[1;90m"), // Black
("BIRed",r"\033[1;91m"), // Red
("BIGreen",r"\033[1;92m"), // Green
("BIYellow",r"\033[1;93m"), // Yellow
("BIBlue",r"\033[1;94m"), // Blue
("BIPurple",r"\033[1;95m"), // Purple
("BICyan",r"\033[1;96m"), // Cyan
("BIWhite",r"\033[1;97m"), // White

    // High Intensity backgrounds
("On_IBlack",r"\033[0;100m"), // Black
("On_IRed",r"\033[0;101m"), // Red
("On_IGreen",r"\033[0;102m"), // Green
("On_IYellow",r"\033[0;103m"), // Yellow
("On_IBlue",r"\033[0;104m"), // Blue
("On_IPurple",r"\033[0;105m"), // Purple
("On_ICyan",r"\033[0;106m"), // Cyan
("On_IWhite",r"\033[0;107m"), // White
]
    ) ;


}