# qauldroid

Arguably the primary client of the project exists on Android.  Being
able to create mesh networks with people around you, without needing
dedicated infrastructure was born out of the idea that a lot of people
have phones, and that creating small scale networks with phones around
you, without having to rely to SIM cards, cell towers or pre-setup
WiFi networks can enable people to avoid censorship and surveillance.

At the moment the qaul android client is a prototype!


## Things to do

This app is by no means even functional, and so needs a lot of work.
If you know about Android and would like to help, cool!  There are
some things below you can work on.


### Make the bottom navigation layout work

The idea is to have four main screens, that can be switched between
with a bottom navigation.  This is partially implemented, but doesn't
really work. It crashes when trying to open one of the custom views.


### Chat view and chat list view

One of the main abilities of the qaul app is a chat.  There are
already fragment layout XML files for the actual chat view, and the
chat list view, but none of that is working.


### Call log view

Similar to the chat list view, but for calls and different metadata
when opening the view.  none of these exist yet


### Contacts view

A view to list other users on the network with their names ("handle"
and "real name"), plus their ID and maybe avatar.  When opening
a user profile a screen should open with more details, and buttons to
start a chat, call, befriend, block, etc.

The list itself should be able to filter for "only friends", and similar,
maybe with sliding side tabs?  Maybe something else works better.


### Filesharing view

The filesharing view shows all advertised and local files that are known by
qaul.  Files that are only advertised should have a "get" button,
and files that are local should be able to be swiped away.

The list should be filterable by "files by friends"


### Wifi Direct 

The app needs to be able to connect to other WiFi Direct devices around it
and establish a simple protocol by which they keep pinging each other to 
see if they are in range, and can transfer "Frames", which are opaque and
encrypted shards of data.

Currently the WiFi Direct code starts looking for peers, but never finds
any, and also there's no service to actually use this to transfer any data.