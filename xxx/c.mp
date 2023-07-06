prologues:=3;
%outputtemplate := "c-%c.png";
%outputformat := "png";
outputtemplate := "%j-%c.pdf";
outputformat := "pdf";
%outputtemplate := "a-%c.mps";
%outputformat := "mps";

input mp-tool.mpiv

verbatimtex
\documentclass{minimal}
\begin{document}
etex



beginfig(1);
    numeric u;
    u := 1cm; % Define the unit length

%    picture p ;
%    % Load the external image
%    p := image (externalfigure "/home/laurent/work/legendary-memory/xxx/ochard.png" scaled 1cm) ;
%    %clip p to unitcircle scaled 3cm ;
%    %circular_shade(boundingbox p enlarged 1pt, 0, .2red, .9red) ;
%    addto currentpicture also  p ;
%    draw p ;

    externalfigure "cover.jpg" scaled 10cm  ;
    externalfigure "xorchard.png" scaled 2cm ;


    draw (-1cm,-1cm) -- (1cm,1cm) ;

    % Draw a rectangle around the image
    draw (0, 0) -- (5u, 0) -- (5u, 3u) -- (0, 3u) -- cycle;

    % Add a label
    label("My External Image", (2.5u, 1.5u));

    label("origin",origin)
endfig;

end.
