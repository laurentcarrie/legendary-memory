set -e
set -x

thisfile=$(realpath $0)
here=$(dirname $thisfile)

put_on_remote() {
	scp $thisfile aws:/home/ubuntu/start.sh
	ssh aws chmod +x /home/ubuntu/start.sh
	scp ~/.ssh/2025 aws:/home/ubuntu/.ssh/2025
}

config_ssh(){
	configfile=/home/ubuntu/.ssh/config
	rm -f $configfile
	echo "Host mygit" > $configfile
	echo "User laurentcarrie" >> $configfile
	echo "HostName github.com" >> $configfile
	echo "IdentityFile ~/.ssh/2025" >> $configfile
	chmod 400 ~/.ssh/2025
	ssh-add ~/.ssh/2025
}


rust(){
	sudo apt update
	sudo apt install --fix-missing
	sudo apt install g++ -y
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
}

lilypond() {
	sudo apt update
	sudo apt install --fix-missing
	sudo apt install lilypond -y
}

repo() {
	ssh -T git@mygit || true
	mkdir $HOME/work || true
	(
	cd $HOME/work ;
	rm -rf legendary-memory ;
	git clone git@mygit:laurentcarrie/legendary-memory.git ;
	cd legendary-memory ;
	git checkout work ;
	mkdir $HOME/.fonts ;
	cp software/fonts/*.ttf $HOME/.fonts/. ;

)
}

awscli() {

	curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
	unzip awscliv2.zip
	sudo ./aws/install
}

s3() {
	mkdir -p ~/.aws
	rm -f  ~/.aws/credentials
	echo "[laurent]" > ~/.aws/credentials
	echo "aws_access_key_id=$aws_access_key_id" >>  ~/.aws/credentials
	echo "aws_secret_access_key=$aws_secret_access_key" >>  ~/.aws/credentials
}


texlive() {
	sudo apt update
	sudo apt install --fix-missing
	sudo apt install texlive-full -y
}



case $1 in
	'por' )
		put_on_remote
		;;
	'config_ssh')
		config_ssh
		;;
	'repo')
		repo
		;;
	'rust')
		rust
		;;
	'lilypond')
		lilypond
		;;
	'texlive')
		texlive
		;;
	'awscli')
		awscli
		;;
	's3')
		s3
		;;
	*)
		echo "no such command : $1"
esac
