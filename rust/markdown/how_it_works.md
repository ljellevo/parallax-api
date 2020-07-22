## How does it work?
By utilizing machine learning, image prossessing and content aware alorithms, parallax manages to convert 2D images to 3D images. This is what sets us apart from the rest. Most smartphones that have more than one camera has the ability get disparity when taking a picture. This data is stored with the image. When you have the disparity, you can calculate the depth. But if it only was that simple...

This data is not fetched when you take something like a screenshot, or an image with snapchat, or many other sources for that matter. And as it turns out, most social media oriented people rearly takes images with the native camera app. And even less uses the correct settings so that the disparity is captured.

This renders the competition useless. They need this information to create all these cool effects, we dont. We use advanced technology to give you these effects on "pure" 2D images.

But... how is that possible?

We apply the power of machine learning to create a depth map of your image. This depth map lets us identify which part of the image belongs where on the new z-axis. And voila, a 3D image is born. When you set the focus, we can locate where on the z-axis the main object is. Knowing these positions, we can cut out the different parts and by applying another machine learning model, and fill in the holes that is created with a content aware algorithm. 

![This is what a generated depth map looks like](resource:assets/images/depth_map.png)  
  
This image shows a before and after the depth map is generated. The source image does not contain any disparity data. And the depth map is generated on the "pure" 2D image.  
  

### mura masa effect
The mura masa effect comes from mura masas music video for "what if I go?", and is not a result of advanced technology. Way back when, a camera called the Nishika N8000 was made. This camera had four object which tok a picture at the same time, but from a different perspective. When these four pictures was presented after eachother in a video format, a 3D effect would be apparent as the perspective changes, while the object stays in the same place. Resulting in a moving foreground and background, while the object remains static.

We wanted to simulte the same effect as it is very cool. Unfortunatley smartphones does not contain four cameras yet (probably they will in the near future), and the position of these cameras is not correct. Therefore we applied some other techniques to produce this effect.

The result should be multiple images layered ontop of eachother. These layers are animated in respect to the perspective. By moving the further away layers more than the closer layers, a 3D effect is created. This is the mura masa effect. Pretty cool right?
