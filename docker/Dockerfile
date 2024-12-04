FROM ubuntu

RUN groupadd -r john_doe && useradd -r -g john_doe john_doe
RUN mkdir -p /home/john_doe
RUN chown john_doe:john_doe /home/john_doe

WORKDIR /home/john_doe/work
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install -y curl omake texlive-full
RUN apt-get install -y gcc

# rust install, official doc does not work...
#RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
RUN curl https://sh.rustup.rs -sSf -o rustup.sh
USER john_doe
ENV HOME=/home/john_doe
RUN bash rustup.sh -y
ENV PATH="$HOME/.cargo/bin:$PATH"


# lilypond
WORKDIR /home/john_doe/lilypond
RUN chown -R john_doe:john_doe /home/john_doe/lilypond
USER john_doe
RUN curl -L https://gitlab.com/lilypond/lilypond/-/releases/v2.24.4/downloads/lilypond-2.24.4-linux-x86_64.tar.gz -o lilypond.tar.gz
RUN tar xvzf lilypond.tar.gz
RUN rm lilypond.tar.gz
ENV PATH="$HOME/lilypond/lilypond-2.24.4/bin:$PATH"




WORKDIR /home/john_doe/work
COPY software/others software/others
COPY software/src software/src
COPY software/Cargo.toml software/Cargo.toml

# install the project fonts
RUN mkdir -p $HOME/.local/share/fonts
COPY software/fonts/* $HOME/.local/share/fonts/.
RUN fc-cache -f -v

# build the tool
USER root
#RUN chown -R john_doe:john_doe /home/john_doe/*
#USER john_doe
#ORKDIR /home/john_doe/work
#RUN cargo install --path software
RUN mkdir /songbin
ENV PATH="/songbin:$PATH"

#RUN rm -rf software

USER root
RUN usermod -aG ubuntu john_doe
RUN mkdir -p /home/john_doe
RUN ln -s $HOME/work/songs /songs
RUN ln -s $HOME/work/books /books
RUN chown -R john_doe:john_doe /home/john_doe/*
RUN mkdir /build
RUN chown -R john_doe:john_doe /build

USER john_doe
# USER root

#RUN mkdir songs
#RUN mkdir books
WORKDIR /home/john_doe/work
COPY software/run.sh run.sh
CMD ["bash","run.sh"]
