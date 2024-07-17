# RustyChess
Rust based web service that communicates via either http or websockets to allow for chess game or chess move prediction


# TODO LIST

- [ ] implement basic http server
- [ ] add basic get route for a random chess opener
- [ ] add a based on board state move prediction
- [ ] implement web sockets for game play
- [ ] create basic rooms for game play
- [ ] handle games
<<<<<<< Updated upstream

=======
- [ ] support http 2
- [ ] support https
- [ ] support websockets over https + http2 
- [ ] support http3


## Chess structure 

```
chess_board = [
    ['R`', 'N`', 'B`', 'Q`', 'K`', 'B`', 'N`', 'R`'],
    ['P`', 'P`', 'P`', 'P`', 'P`', 'P`', 'P`', 'P`'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
    ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']
]
```
>>>>>>> Stashed changes
