let make_preamble : string =
  {whatever|
  %\usepackage[margin=0.5cm]{geometry}
\usepackage[left=1cm,right=1cm,top=1cm,bottom=2cm]{geometry}
\usepackage[utf8x]{inputenc}
%\usepackage[light, largesmallcaps]{kpfonts}
\usepackage{mwe}
%\usepackage{geometry}
\usepackage{lipsum}
\usepackage{microtype}
\usepackage{grid-system}
\usepackage{babel}
\usepackage{listings}
\usepackage{graphicx}
\usepackage{verse}

\usepackage{aurical}
\usepackage[T1]{fontenc}
\usepackage[table,x11names]{xcolor}
\usepackage{titlesec}
%\usepackage{color}
\usepackage{longtable}
\usepackage{multirow}
%\usepackage[full]{leadsheets}

% grilles
\newlength\gridheight\setlength\gridheight{215.0mm}
\newlength\gridwidth\setlength\gridwidth{180.0mm}
\def\mygridbox#1{\parbox[c][\gridheight][c]{\gridwidth}{\center #1}}
\def\mytitlebox#1{\parbox[c][{40.0mm}][c]{\gridwidth}{\center #1}}
\setlength{\tabcolsep}{0.0mm}

\usepackage{fancyhdr}
\usepackage{lastpage}
\pagestyle{fancy}
\fancyhf{}
%\usepackage{tabularx}
\usepackage[most]{tcolorbox}

\usepackage{booktabs}
\usepackage{array}
\usepackage{paracol}

%\setlength{\headheight}{10pt}

\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrulewidth}{2pt}

%\lhead{\includegraphics[width=1cm]{example-image-a}}
\rhead{}

\rfoot{\thepage/\pageref{LastPage}}
\cfoot{\today}


\usepackage{multicol}

\definecolor{bg}{HTML}{ADFF2F}
%\definecolor{bgred}{\color{Seashell3}}

\usepackage{fontspec}
\setmainfont{Garamond Libre}

% pour les lignes pointillées dans les tableaux
\usepackage{arydshln}

\newcommand{\lolocomment}[1]{{
    \fontsize{8pt}{8pt}\selectfont
    \textcolor{red}{#1}}
}

\newcommand{\basecouplet}[3]{
%\colorbox{bg}{\section*{#1}}
    \colorbox{#1}{
        \fontsize{12pt}{12pt}\selectfont
        #2
    }
    {
            {
            \fontsize{10pt}{10pt}\selectfont
            #3
        }
    }

}
\newcommand{\lolocolorcouplet}{Chartreuse1}
\newcommand{\couplet}[2]{\basecouplet{\lolocolorcouplet}{#1}{#2}}
\newcommand{\coupletred}[2]{\basecouplet{Firebrick1}{#1}{#2}}
\newcommand{\lolocolorprerefrain}{DarkOrange1}
\newcommand{\prerefrain}[2]{\basecouplet{\lolocolorprerefrain}{#1}{#2}}
\newcommand{\lolocolorrefrain}{CadetBlue1}
\newcommand{\refrain}[2]{\basecouplet{\lolocolorrefrain}{#1}{#2}}
\newcommand{\lolocolorpont}{PeachPuff1}
\newcommand{\pont}[2]{\basecouplet{\lolocolorpont}{#1}{#2}}


\newcommand{\songtitle}[2] {
    {
        \Fontskrivan\bfseries\slshape
        \fontsize{60pt}{50pt}\selectfont
        \color{blue}
        #1
    } \\


    {
        \Fontskrivan\bfseries\slshape
        \fontsize{20pt}{10pt}\selectfont
        \color{orange}
        #2
    }\\
}


%\newcolumntype{C}{>{\centering\arraybackslash}m{2cm}}
\newcolumntype{C}{>{\centering\arraybackslash}m{3.5cm}}
\newcolumntype{R}{r}
\newcolumntype{D}{>{\centering\arraybackslash}m{1cm}}
%\newcolumntype{C}{>{\centering}p{0.28cm}}

\newcommand{\beforegrille}{
%\Fontskrivan\bfseries\slshape
%\fontsize{55pt}{52pt}
    \fontsize{10pt}{8pt}\selectfont
    \renewcommand{\arraystretch}{2}

}

\newcommand{\beforelyrics}{
%\Fontskrivan\bfseries\slshape
    \Fontskrivan
    \fontsize{55pt}{52pt}
%   \fontsize{10pt}{8pt}\selectfont
    \renewcommand{\arraystretch}{2}

}


\usepackage[thinlines]{easytable}


%\usepackage{paracol}

\newfontfamily\lolo[%%basic weight: 100, "bold" weight: 70
    Extension      = .ttf,
    ItalicFont     = lolo,
    BoldFont       = lolo,
    BoldItalicFont = lolo]
{lolo}



\newfontfamily\loloflat[%%basic weight: 50, "bold" weight: 70\overline{}
    Extension      = .ttf,
    ItalicFont     = lolo_flat,
    BoldFont       = lolo_flat,
    BoldItalicFont = lolo_flat]
{lolo_flat}


\newfontfamily\lolosharp[%%basic weight: 50, "bold" weight: 70
    Extension      = .ttf,
    ItalicFont     = lolo_sharp,
    BoldFont       = lolo_sharp,
    BoldItalicFont = lolo_sharp]
{lolo_sharp}


% pour les livres
\newcommand{\fakesection}[1]{%
    \par\refstepcounter{section}% Increase section counter
    \sectionmark{#1}% Add section mark (header)
    \addcontentsline{toc}{section}{\protect\numberline{\thesection}#1}% Add section to ToC
% Add more content here, if needed.
}


%\usepackage{color, colortbl}
%\usepackage[table]{xcolor}
\definecolor{Gray}{gray}{0.9}
\definecolor{LightCyan}{rgb}{0.88,1,1}
\definecolor{Row1}{rgb}{0.77,9,9}
\definecolor{Row2}{rgb}{0.9,0.77,9}

%LightGoldenrodYellow 	  	FA 	FA 	D2 	250 	250 	210
\definecolor{LightGoldenrodYellow}{RGB}{250,250,210}
\newcommand{\rowcolora}{\rowcolor{Thistle1}}
%\newcommand{\rowcolorb}{\rowcolor{Seashell3}}
\newcommand{\rowcolorb}{\rowcolor{Coral1}}


\newcounter{bar}

% bar counter
\newcommand{\loloinitcounter}[1]{
    \setcounter{bar}{#1}
}

\newcommand{\loloshowcounter}[1]{%
    \fontsize{16pt}{6pt}
    \cellcolor{white}$_{\arabic{bar}}$
    \addtocounter{bar}{#1}
}



\newcommand{\loloshowarrowcounter}[1]{%
    \fontsize{16pt}{6pt}
    \cellcolor{white}$_{\arabic{bar} \rightarrow \addtocounter{bar}{#1} \addtocounter{bar}{-1} \arabic{bar}}$
    \addtocounter{bar}{1}
}

\newcommand{\loloprintbpm}[3]{%
% min sec bars

    \def\x{#1}
    \def\y{#2}
    \def\z{#3}
    % $\x \div \y =$
    \newcount\a\a=\number\x
    \newcount\b\b=\number\y
    \multiply\a by 60
    \advance\a \y
    \def\nseconds{\the\a}
    %number of seconds : $\nseconds$

    \newcount\c\c=\number\z
    \multiply\c by 4
    \def\nbbars{\the\c}
    %number of beats : $\nbbars$

    \multiply\c by 60
    \divide\c by \nseconds
    \def\bpm{\the\c}
    %number of beats per minute : $\bpm$

    durée : #1' #2'' ; #3 mesures ; tempo = $\bpm$

% #1' #2'/$\bpm$

}

\newcommand{\makefooter}[2]{
    \lfoot{#1/#2}
%/\loloprintbpm{#3}{#4}{#5}}
}

\newcommand{\lolomakerowanc}[1]{
    & \multicolumn{4}{l}{#1}
}


\newcommand{\lolomakerowbnc}[1]{
    & \multicolumn{1}{l}{}         & \multicolumn{3}{l}{#1}
}

\newcommand{\lolomakerowcnc}[1]{
    & \multicolumn{1}{l}{}
    & \multicolumn{1}{l}{}         & \multicolumn{2}{l}{#1}
}

\newcommand{\lolomakerowdnc}[1]{
    & \multicolumn{1}{l}{}
    & \multicolumn{1}{l}{}
    & \multicolumn{1}{l}{}         & \multicolumn{1}{l}{#1}
}

\newcommand{\lolomakerowa}[2]{
    \loloshowarrowcounter{#1}
    & \multicolumn{4}{l}{#2}
}


\newcommand{\lolomakerowb}[2]{
    \loloshowarrowcounter{#1}
    & \multicolumn{1}{l}{}         & \multicolumn{3}{l}{#2}
}

\newcommand{\lolomakerowc}[2]{
    \loloshowarrowcounter{#1}
    & \multicolumn{1}{l}{}
    & \multicolumn{1}{l}{}         & \multicolumn{2}{l}{#2}
}

\newcommand{\lolomakerowd}[2]{
    \loloshowarrowcounter{#1}
    & \multicolumn{1}{l}{}
    & \multicolumn{1}{l}{}
    & \multicolumn{1}{l}{}         & \multicolumn{1}{l}{#2}
}
  |whatever}
