# Eno Number

> Work in Progress!

Eno Number is a project inspired by the previously popular 
[Bacon Number](https://en.wikipedia.org/wiki/Six_Degrees_of_Kevin_Bacon) concept, and the idea that Kevin Bacon
is never more than 6 jumps away from any other movie. Eno Number is the same thing, but with albums and Brian Eno.

The server is written in Rust, and connects directly to a copy of the MusicBrainz database. 
For this reason, you will likely need to [host your own copy of the database](https://musicbrainz.org/doc/MusicBrainz_Database/Download). 

There is also an official [Docker version](https://github.com/metabrainz/musicbrainz-docker) of the server.
I have included `musicbrainz-docker.patch` to allow the database to be accessed over `localhost`.

You will need a Redis server to connect to (separate from the MusicBrainz one). 

Database connection settings can be changed in `Rocket.toml`.
