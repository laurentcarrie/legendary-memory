#FROM public.ecr.aws/lambda/provided:al2.2025.07.03.10
FROM ubuntu


RUN	apt update
RUN	apt install --fix-missing
RUN	apt install g++ curl -y


RUN	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
RUN	sh rustup.sh -y




RUN	apt update
RUN	apt install --fix-missing
RUN	apt install lilypond -y
# RUN	curl -L -o lilypond.tar.gz https://gitlab.com/lilypond/lilypond/-/releases/v2.24.4/downloads/lilypond-2.24.4-linux-x86_64.tar.gz
# RUN	tar -xzf lilypond.tar.gz
# RUN	mv lilypond-2.24.4 lilypond
# RUN	chmod +x lilypond/usr/bin/lilypond
# RUN	cp -r lilypond/usr/* /usr/
# RUN	rm -rf lilypond lilypond.tar.gz 



# # RUN	sudo apt update
# # RUN	sudo apt install --fix-missing
# # RUN	sudo apt install texlive-full -y
# RUN	curl -L -o install-tl-unx.tar.gz https://mirror.ctan.org/systems/texlive/tlnet/install-tl-unx.tar.gz
# RUN	tar -xzf install-tl-unx.tar.gz
# RUN	cd install-tl-*/ && ./install-tl    

#EXEC bash
# ENTRYPOINT bash

#CMD [ "function.handler" ]