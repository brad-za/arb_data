# arb_data

In South Africa there is a natural and very risk averse abritrage. The route is as follows, ZAR -> USD -> BTC -> ZAR.

This API sets out to give crypto price info from 2 exchanges as well as all forex pairs.

todo: implement ARB endpoint.

# arb_data

In South Africa there is a natural and very risk averse abritrage. The route is as follows, ZAR -> USD -> BTC -> ZAR.

This API sets out to give crypto price info from 2 exchanges as well as all forex pairs.

todo: implement ARB endpoint.

# docker

docker build command
`docker build -t arb_data .`
docker run command
`docker run -p 8000:8000 --rm --name arb_data arb_data`

I am trying to dockerise this simple server as part of a learning exercise. I am having 3 issues.

1. I can't figure out how to work on code in a docker image.
   a) must I rebuild the image each time?
   b) can I access the files that are present IN the docker image?

2. When using a very simple Dockerfile with one build step my build/compile time is very long. Am I using the correct method to bring down compile times?

3. When this finishes building there is no output to log. Makes me think the exe never got executed.
   a) when using the simple Dockerfile the server started but was not reachable. I tried localhost:8000 0.0.0.0:8000 and 192.168.99.100:8000 to no avail.
