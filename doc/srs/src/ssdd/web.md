# web

We will have web pages delivered, but with no web server.

## no web server ?

Well, no developement of a web server, we will only have `nginx` running, and using a nice feature of nginx : the ability to
trigger sh scripts. This will allow us to have post actions, the two main ones are :
1. trigger a build
2. write a source file

this feature is configured in the `/etc/nginx/sites-enabled/songbook.conf` config file of nginx, in which we have :

       location ~ (\.sh)$ {
           types  { application/json sh; }
           gzip off;
           root /var/www/songbook/scripts;
           autoindex on;
           fastcgi_pass unix:/var/run/fcgiwrap.socket;
           include /etc/nginx/fastcgi_params;
           fastcgi_param DOCUMENT_ROOT /var/www/$server_name/scripts ;
           fastcgi_param SCRIPT_FILENAME /var/www/$server_name$fastcgi_script_name;
           fastcgi_param QUERY_STRING    $query_string;
       }



## web rendering

We will do that with `rust-leptos` create, that allows to write code in rust, that is translated to javascript and runs
in the browser : there is no server side code.

## why ?

just because in our workflow, we don't really need a web server. We just need to trigger ``omake`` to build the outputs,
and

## songbook user and www-data

Let's assume the tool is installed under `songbook` user.
the web server runs under `www-data` user, has no home directory, and has no right to change the songbook user files.
The only thing `www-data` can do is run the `request.sh` script in the `/var/www/songbook/scripts` directory

    client web browser
      |
      +-->  action --> http transfer --> nginx server
                                                |
    request.sh  <-------------------------------|
        |
        +---> push request ( songbook-client running under www-data )
                     |
                   ZMQ messaging service
                     |
                     +---> songbook-server running under songbook user
                                 polls the request
                                        +---> action taken

# [ZeroMQ](https://zeromq.org/)

this is free, easy to use and performant messaging library.
There is a rust crate for that, and no server to configure.
