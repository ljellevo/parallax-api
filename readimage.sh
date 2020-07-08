base64 -w 0 woman.PNG

base64 -d woman.PNG > woman2.PNG
base64 -w 0 woman.PNG > woman.PNG.base64

openssl base64 -in woman.PNG -out woman.base64

base64 -i woman.PNG -o woman.PNG.base64