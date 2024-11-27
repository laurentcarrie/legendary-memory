pub fn make_preamble() -> String {
    let ret = r###"%\usepackage[margin=0.5cm]{geometry}
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
%\cfoot{\today}
%\cfoot{derni\`ere modif le \songlastupdate, g\'en\'er\'e le \songtoday }
\cfoot{derni\`ere modif le \songlastupdate }


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

\newcommand{\basetitle}[2]{
	%\colorbox{bg}{\section*{#1}}
	\colorbox{#1}{
		\fontsize{12pt}{12pt}\selectfont
		#2
	}
}

\newcommand{\lolocolorcouplet}{Chartreuse1}
\newcommand{\couplet}[2]{\basecouplet{\lolocolorcouplet}{#1}{#2}}
\newcommand{\couplettitle}[1]{\basetitle{\lolocolorcouplet}{#1}}

\newcommand{\coupletred}[2]{\basecouplet{Firebrick1}{#1}{#2}}
\newcommand{\lolocolorprerefrain}{DarkOrange1}
\newcommand{\prerefrain}[2]{\basecouplet{\lolocolorprerefrain}{#1}{#2}}

\newcommand{\lolocolorrefrain}{CadetBlue1}
\newcommand{\refrain}[2]{\basecouplet{\lolocolorrefrain}{#1}{#2}}
\newcommand{\refraintitle}[1]{\basetitle{\lolocolorrefrain}{#1}}


\newcommand{\lolocolorpont}{PeachPuff1}
\newcommand{\pont}[2]{\basecouplet{\lolocolorpont}{#1}{#2}}
\newcommand{\lolocolorsolo}{PeachPuff1}
\newcommand{\solo}[2]{\basecouplet{\lolocolorsolo}{#1}{#2}}



\newcommand{\xxmakesongtitle}[2] {
	\begin{center}
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
	\end{center}

}


\newcolumntype{C}{>{\centering\arraybackslash}m{1.5cm}}
%\newcolumntype{C}{>{\centering\arraybackslash}m{3.5cm}}
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


\newcommand\lolotwocolumns[2]{
	\begin{minipage}{0.40\linewidth}
		\vspace{0pt}
		#1
	\end{minipage}
	\begin{minipage}{0.40\linewidth}
		\vspace{0pt}
		#2
	\end{minipage}
}

\newcommand\lolothreecolumns[3]{
	\begin{minipage}{0.40\linewidth}
		\vspace{0pt}
		#1
	\end{minipage}
	\begin{minipage}{0.10\linewidth}
		\vspace{0pt}
		#2
	\end{minipage}
	\begin{minipage}{0.50\linewidth}
		\vspace{0pt}
		#3
	\end{minipage}
}

\newcommand\lolohr{
	\begin{center}
		\line(1,0){450}
	\end{center}
}

\newcommand\lolovspace{\vspace{0.5cm plus 0.5ex}}"###;
    ret.to_string()
}
pub fn make_chords() -> String {
    let ret = r###"\newcommand\chordA{{\lolo\fontsize{18pt}{18pt}\selectfont A ~ }}
\newcommand\chordB{{\lolo\fontsize{18pt}{18pt}\selectfont B ~ }}
\newcommand\chordC{{\lolo\fontsize{18pt}{18pt}\selectfont C ~ }}
\newcommand\chordD{{\lolo\fontsize{18pt}{18pt}\selectfont D ~ }}
\newcommand\chordE{{\lolo\fontsize{18pt}{18pt}\selectfont E ~ }}
\newcommand\chordF{{\lolo\fontsize{18pt}{18pt}\selectfont F ~ }}
\newcommand\chordG{{\lolo\fontsize{18pt}{18pt}\selectfont G ~ }}
\newcommand\chordAm{{\lolo\fontsize{18pt}{18pt}\selectfont H ~ }}
\newcommand\chordBm{{\lolo\fontsize{18pt}{18pt}\selectfont I ~ }}
\newcommand\chordCm{{\lolo\fontsize{18pt}{18pt}\selectfont J ~ }}
\newcommand\chordDm{{\lolo\fontsize{18pt}{18pt}\selectfont K ~ }}
\newcommand\chordEm{{\lolo\fontsize{18pt}{18pt}\selectfont L ~ }}
\newcommand\chordFm{{\lolo\fontsize{18pt}{18pt}\selectfont M ~ }}
\newcommand\chordGm{{\lolo\fontsize{18pt}{18pt}\selectfont N ~ }}
\newcommand\chordAsept{{\lolo\fontsize{18pt}{18pt}\selectfont O ~ }}
\newcommand\chordBsept{{\lolo\fontsize{18pt}{18pt}\selectfont P ~ }}
\newcommand\chordCsept{{\lolo\fontsize{18pt}{18pt}\selectfont Q ~ }}
\newcommand\chordDsept{{\lolo\fontsize{18pt}{18pt}\selectfont R ~ }}
\newcommand\chordEsept{{\lolo\fontsize{18pt}{18pt}\selectfont S ~ }}
\newcommand\chordFsept{{\lolo\fontsize{18pt}{18pt}\selectfont T ~ }}
\newcommand\chordGsept{{\lolo\fontsize{18pt}{18pt}\selectfont U ~ }}
\newcommand\chordAmsept{{\lolo\fontsize{18pt}{18pt}\selectfont V ~ }}
\newcommand\chordBmsept{{\lolo\fontsize{18pt}{18pt}\selectfont W ~ }}
\newcommand\chordCmsept{{\lolo\fontsize{18pt}{18pt}\selectfont X ~ }}
\newcommand\chordDmsept{{\lolo\fontsize{18pt}{18pt}\selectfont Y ~ }}
\newcommand\chordEmsept{{\lolo\fontsize{18pt}{18pt}\selectfont Z ~ }}
\newcommand\chordFmsept{{\lolo\fontsize{18pt}{18pt}\selectfont [ ~ }}
\newcommand\chordGmsept{{\lolo\fontsize{18pt}{18pt}\selectfont \ ~ }}
\newcommand\chordAseptM{{\lolo\fontsize{18pt}{18pt}\selectfont c ~ }}
\newcommand\chordBseptM{{\lolo\fontsize{18pt}{18pt}\selectfont d ~ }}
\newcommand\chordCseptM{{\lolo\fontsize{18pt}{18pt}\selectfont e ~ }}
\newcommand\chordDseptM{{\lolo\fontsize{18pt}{18pt}\selectfont f ~ }}
\newcommand\chordEseptM{{\lolo\fontsize{18pt}{18pt}\selectfont g ~ }}
\newcommand\chordFseptM{{\lolo\fontsize{18pt}{18pt}\selectfont h ~ }}
\newcommand\chordGseptM{{\lolo\fontsize{18pt}{18pt}\selectfont i ~ }}
\newcommand\chordAf{{\loloflat\fontsize{18pt}{18pt}\selectfont A ~ }}
\newcommand\chordBf{{\loloflat\fontsize{18pt}{18pt}\selectfont B ~ }}
\newcommand\chordCf{{\loloflat\fontsize{18pt}{18pt}\selectfont C ~ }}
\newcommand\chordDf{{\loloflat\fontsize{18pt}{18pt}\selectfont D ~ }}
\newcommand\chordEf{{\loloflat\fontsize{18pt}{18pt}\selectfont E ~ }}
\newcommand\chordFf{{\loloflat\fontsize{18pt}{18pt}\selectfont F ~ }}
\newcommand\chordGf{{\loloflat\fontsize{18pt}{18pt}\selectfont G ~ }}
\newcommand\chordAfm{{\loloflat\fontsize{18pt}{18pt}\selectfont H ~ }}
\newcommand\chordBfm{{\loloflat\fontsize{18pt}{18pt}\selectfont I ~ }}
\newcommand\chordCfm{{\loloflat\fontsize{18pt}{18pt}\selectfont J ~ }}
\newcommand\chordDfm{{\loloflat\fontsize{18pt}{18pt}\selectfont K ~ }}
\newcommand\chordEfm{{\loloflat\fontsize{18pt}{18pt}\selectfont L ~ }}
\newcommand\chordFfm{{\loloflat\fontsize{18pt}{18pt}\selectfont M ~ }}
\newcommand\chordGfm{{\loloflat\fontsize{18pt}{18pt}\selectfont N ~ }}
\newcommand\chordAfsept{{\loloflat\fontsize{18pt}{18pt}\selectfont O ~ }}
\newcommand\chordBfsept{{\loloflat\fontsize{18pt}{18pt}\selectfont P ~ }}
\newcommand\chordCfsept{{\loloflat\fontsize{18pt}{18pt}\selectfont Q ~ }}
\newcommand\chordDfsept{{\loloflat\fontsize{18pt}{18pt}\selectfont R ~ }}
\newcommand\chordEfsept{{\loloflat\fontsize{18pt}{18pt}\selectfont S ~ }}
\newcommand\chordFfsept{{\loloflat\fontsize{18pt}{18pt}\selectfont T ~ }}
\newcommand\chordGfsept{{\loloflat\fontsize{18pt}{18pt}\selectfont U ~ }}
\newcommand\chordAmseptf{{\loloflat\fontsize{18pt}{18pt}\selectfont V ~ }}
\newcommand\chordBmseptf{{\loloflat\fontsize{18pt}{18pt}\selectfont W ~ }}
\newcommand\chordCmseptf{{\loloflat\fontsize{18pt}{18pt}\selectfont X ~ }}
\newcommand\chordDmseptf{{\loloflat\fontsize{18pt}{18pt}\selectfont Y ~ }}
\newcommand\chordEmseptf{{\loloflat\fontsize{18pt}{18pt}\selectfont Z ~ }}
\newcommand\chordFmseptf{{\loloflat\fontsize{18pt}{18pt}\selectfont [ ~ }}
\newcommand\chordGmseptf{{\loloflat\fontsize{18pt}{18pt}\selectfont \ ~ }}
\newcommand\chordAseptMf{{\loloflat\fontsize{18pt}{18pt}\selectfont c ~ }}
\newcommand\chordBseptMf{{\loloflat\fontsize{18pt}{18pt}\selectfont d ~ }}
\newcommand\chordCseptMf{{\loloflat\fontsize{18pt}{18pt}\selectfont e ~ }}
\newcommand\chordDseptMf{{\loloflat\fontsize{18pt}{18pt}\selectfont f ~ }}
\newcommand\chordEseptMf{{\loloflat\fontsize{18pt}{18pt}\selectfont g ~ }}
\newcommand\chordFseptMf{{\loloflat\fontsize{18pt}{18pt}\selectfont h ~ }}
\newcommand\chordGseptMf{{\loloflat\fontsize{18pt}{18pt}\selectfont i ~ }}
\newcommand\chordAs{{\lolosharp\fontsize{18pt}{18pt}\selectfont A ~ }}
\newcommand\chordBs{{\lolosharp\fontsize{18pt}{18pt}\selectfont B ~ }}
\newcommand\chordCs{{\lolosharp\fontsize{18pt}{18pt}\selectfont C ~ }}
\newcommand\chordDs{{\lolosharp\fontsize{18pt}{18pt}\selectfont D ~ }}
\newcommand\chordEs{{\lolosharp\fontsize{18pt}{18pt}\selectfont E ~ }}
\newcommand\chordFs{{\lolosharp\fontsize{18pt}{18pt}\selectfont F ~ }}
\newcommand\chordGs{{\lolosharp\fontsize{18pt}{18pt}\selectfont G ~ }}
\newcommand\chordAsm{{\lolosharp\fontsize{18pt}{18pt}\selectfont H ~ }}
\newcommand\chordBsm{{\lolosharp\fontsize{18pt}{18pt}\selectfont I ~ }}
\newcommand\chordCsm{{\lolosharp\fontsize{18pt}{18pt}\selectfont J ~ }}
\newcommand\chordDsm{{\lolosharp\fontsize{18pt}{18pt}\selectfont K ~ }}
\newcommand\chordEsm{{\lolosharp\fontsize{18pt}{18pt}\selectfont L ~ }}
\newcommand\chordFsm{{\lolosharp\fontsize{18pt}{18pt}\selectfont M ~ }}
\newcommand\chordGsm{{\lolosharp\fontsize{18pt}{18pt}\selectfont N ~ }}
\newcommand\chordAssept{{\lolosharp\fontsize{18pt}{18pt}\selectfont O ~ }}
\newcommand\chordBssept{{\lolosharp\fontsize{18pt}{18pt}\selectfont P ~ }}
\newcommand\chordCssept{{\lolosharp\fontsize{18pt}{18pt}\selectfont Q ~ }}
\newcommand\chordDssept{{\lolosharp\fontsize{18pt}{18pt}\selectfont R ~ }}
\newcommand\chordEssept{{\lolosharp\fontsize{18pt}{18pt}\selectfont S ~ }}
\newcommand\chordFssept{{\lolosharp\fontsize{18pt}{18pt}\selectfont T ~ }}
\newcommand\chordGssept{{\lolosharp\fontsize{18pt}{18pt}\selectfont U ~ }}
\newcommand\chordAmsepts{{\lolosharp\fontsize{18pt}{18pt}\selectfont V ~ }}
\newcommand\chordBmsepts{{\lolosharp\fontsize{18pt}{18pt}\selectfont W ~ }}
\newcommand\chordCmsepts{{\lolosharp\fontsize{18pt}{18pt}\selectfont X ~ }}
\newcommand\chordDmsepts{{\lolosharp\fontsize{18pt}{18pt}\selectfont Y ~ }}
\newcommand\chordEmsepts{{\lolosharp\fontsize{18pt}{18pt}\selectfont Z ~ }}
\newcommand\chordFmsepts{{\lolosharp\fontsize{18pt}{18pt}\selectfont [ ~ }}
\newcommand\chordGmsepts{{\lolosharp\fontsize{18pt}{18pt}\selectfont \ ~ }}
\newcommand\chordAseptMs{{\lolosharp\fontsize{18pt}{18pt}\selectfont c ~ }}
\newcommand\chordBseptMs{{\lolosharp\fontsize{18pt}{18pt}\selectfont d ~ }}
\newcommand\chordCseptMs{{\lolosharp\fontsize{18pt}{18pt}\selectfont e ~ }}
\newcommand\chordDseptMs{{\lolosharp\fontsize{18pt}{18pt}\selectfont f ~ }}
\newcommand\chordEseptMs{{\lolosharp\fontsize{18pt}{18pt}\selectfont g ~ }}
\newcommand\chordFseptMs{{\lolosharp\fontsize{18pt}{18pt}\selectfont h ~ }}
\newcommand\chordGseptMs{{\lolosharp\fontsize{18pt}{18pt}\selectfont i ~ }}
\newcommand\chordERest{{\lolosharp\fontsize{18pt}{18pt}\selectfont q ~ }}
\newcommand\chordQRest{{\lolosharp\fontsize{18pt}{18pt}\selectfont r ~ }}
\newcommand\chordHRest{{\lolosharp\fontsize{18pt}{18pt}\selectfont s ~ }}
\newcommand\chordQHRest{{\lolosharp\fontsize{18pt}{18pt}\selectfont t ~ }}
\newcommand\chordRepeatDeux{{\lolo\fontsize{18pt}{24pt}\selectfont 2 ~ }}
\newcommand\chordRepeatTrois{{\lolo\fontsize{18pt}{24pt}\selectfont 3 ~ }}"###;
    ret.to_string()
}
