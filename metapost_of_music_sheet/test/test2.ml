open Printf
module Log = Dolog.Log

let maintex : string =
  {whatever|
\documentclass[11pt]{article}
\usepackage{graphicx}

\begin{document}

    \begin{center}
      \includegraphics[width=\linewidth]{test2}
    \end{center}


\end{document}

|whatever}

let _ =
  Log.set_log_level Log.DEBUG;
  Log.set_output stdout;
  Log.color_on ();

  let _ = Totolib.pdf.make_pdf() in
  let () = printf "test2 passed.\n" in
  ()
