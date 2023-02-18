# rust-pong

A Rust implementation of the classic Pong game.

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
