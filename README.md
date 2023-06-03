# Harss.net client

This project aims to provide a web client for the Harss API. Like the said 
API, it's more a pet project to discover Rust and the frontend framework 
than a serious stuff, so don't expect production ready code. But please, 
if you want to help and give advice, they are more than welcome


## Authentication

Since it was one on my first difficulty on this project, let's explain 
how the authentication works in a few words.

Please note that this workflow is far from perfect and will need some
adjustment  (like sending/reading the refresh token as a HttpOnly 
cookie)

### API side

The API provide an unauthenticated endpoint to allow user to identify
themself with their user and password.
In case of successfully authenticated, the API returns in the response
two tokens:
 * One access JSON web token (aka JWT). It has a short TTL (15 minutes)
 * One refresh token with a long TTL

The API provide a second endpoint that expect the refresh token and return
a new access token.

### Client side

Once the client is authenticated using the login endpoint of the API, each
request will provide the JWT in the Authorization header. 

If the API returns a 401 HTTP Status, the HTTP client will try to obtain
a new JWT by calling the API's refresh endpoint, and try again the request
