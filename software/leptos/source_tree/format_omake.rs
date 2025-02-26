pub fn format_string(input: &String) -> String {
    let  pairs = [
        (r"[0m", "</span>"), // Text Reset

        // Regular Colors
        (r"[0;30m", "Black"), // Black
        (r"[0;31m", "Red"), // Red
        (r"[0;32m", "Green"), // Green
        (r"[0;33m", "Yellow"), // Yellow
        (r"[0;34m", "Blue"), // Blue
        (r"[0;35m", "Purple"), // Purple
        (r"[0;36m", "Cyan"), // Cyan
        (r"[0;37m", "White"), // White

        // Bold
        (r"[1;30m", "BBlack"), // Black
        (r"[1;31m", "BRed"), // Red
        (r"[1;32m", "BGreen"), // Green
        (r"[1;33m", "BYellow"), // Yellow
        (r"[1;34m", r#"<span style="color:blue""#), // Blue
        (r"[1;35m", "BPurple"), // Purple
        (r"[1;36m", "BCyan"), // Cyan
        (r"[1;37m", "BWhite"), // White

        // Underline
        (r"[4;30m", "UBlack"), // Black
        (r"[4;31m", "URed"), // Red
        (r"[4;32m", "UGreen"), // Green
        (r"[4;33m", "UYellow"), // Yellow
        (r"[4;34m", "UBlue"), // Blue
        (r"[4;35m", "UPurple"), // Purple
        (r"[4;36m", "UCyan"), // Cyan
        (r"[4;37m", "UWhite"), // White

        // Background
        (r"[40m", "On_Black"), // Black
        (r"[41m", "On_Red"), // Red
        (r"[42m", "On_Green"), // Green
        (r"[43m", "On_Yellow"), // Yellow
        (r"[44m", "On_Blue"), // Blue
        (r"[45m", "On_Purple"), // Purple
        (r"[46m", "On_Cyan"), // Cyan
        (r"[47m", "On_White"), // White

        // High Intensity
        (r"[0;90m", "IBlack"), // Black
        (r"[0;91m", "IRed"), // Red
        (r"[0;92m", "IGreen"), // Green
        (r"[0;93m", "IYellow"), // Yellow
        (r"[0;94m", "IBlue"), // Blue
        (r"[0;95m", "IPurple"), // Purple
        (r"[0;96m", "ICyan"), // Cyan
        (r"[0;97m", "IWhite"), // White

        // Bold High Intensity
        (r"[1;90m", "BIBlack"), // Black
        (r"[1;91m", "BIRed"), // Red
        (r"[1;92m", "BIGreen"), // Green
        (r"[1;93m", "BIYellow"), // Yellow
        (r"[1;94m", "BIBlue"), // Blue
        (r"[1;95m", "BIPurple"), // Purple
        (r"[1;96m", "BICyan"), // Cyan
        (r"[1;97m", "BIWhite"), // White

        // High Intensity backgrounds
        (r"[0;100m", "On_IBlack"), // Black
        (r"[0;101m", "On_IRed"), // Red
        (r"[0;102m", "On_IGreen"), // Green
        (r"[0;103m", "On_IYellow"), // Yellow
        (r"[0;104m", "On_IBlue"), // Blue
        (r"[0;105m", "On_IPurple"), // Purple
        (r"[0;106m", "On_ICyan"), // Cyan
        (r"[0;107m", "On_IWhite"), // White
    ]
 ;
    let mut ret = input.clone() ;
    for pair in pairs {
        ret = ret.replace(pair.0,pair.1) ;
    }
    ret

}