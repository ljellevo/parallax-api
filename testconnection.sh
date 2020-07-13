#curl --header "Content-Type: application/json" --request POST --data '{"effect":"test effect"}' https://parallax-api-dev.herokuapp.com/api/image

#curl -X POST "https://parallax-api-dev.herokuapp.com/api/mura_masa" -H  "accept: */*" -H  "Content-Type: multipart/form-data" -F "image=@woman.PNG;type=image/png"

curl -X POST "localhost:8000/api/muramasa" -H  "accept: */*" -H  "Content-Type: multipart/form-data" -F "image=@woman.PNG;type=image/png"