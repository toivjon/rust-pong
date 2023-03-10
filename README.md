# rust-pong

A Rust implementation of the classic Pong game.

This implementation uses Windows API and runs only on Windows machines.

## Scenes

Game is split into following scenes:

1. A main menu scene, which contains the start and quit selections.
2. A court scene, which contains the actual gameplay.
3. An end game scene, which contains the results from the court scene.

A list of scene transitions:

- 1 to 2, when a player starts the game.
- 2 to 3, when either player receives the 10th point (i.e. the game is over).
- 3 to 1, when the enter key is being pressed.

## Features

This Pong implementation contains the following features.

- Each game lasts until either player receives the 10th point.
- Both paddles are controlled by human players.
- Both paddles are returned to their default position after each reset.
- Ball movement is being stopped for half second after each reset.
- Ball velocity is increased on a hit with a paddle.
- Ball velocity does not exceed the pre-defined maximum velocity.

## Screenshots

![alt text](https://github.com/toivjon/rust-pong/blob/main/screenshots/mainmenu.png "MainManu")
![alt text](https://github.com/toivjon/rust-pong/blob/main/screenshots/court.png "Court")
![alt text](https://github.com/toivjon/rust-pong/blob/main/screenshots/endgame.png "EndGame")
