#curl --header "Content-Type: application/json" --request POST --data '{"effect":"test effect"}' https://parallax-api-dev.herokuapp.com/api/image

curl -X POST "localhost:8000/api/image" -H  "accept: */*" -H  "Content-Type: multipart/form-data" -F data="{'effect': 'mura masa'}" -F "image=@woman.PNG;type=image/png"