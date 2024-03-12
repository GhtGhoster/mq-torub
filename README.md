# Torub puzzle
## Rules
This puzzle was inspired by a puzzle featured on Czech TV version of the show "Survivor".
It features a limited rule-set, which limits the originally arbitrary swapping of 2 nodes
to shifting rows and columns of the puzzle matrix.

One must scramble the puzzle until no row or column contains the same letter or color twice.
(Color is replaced with numbers in higher size variations)

An inverse version of this puzzle is planned, where the player starts with a "solved"
configuration of the puzzle and must unscramble the puzzle to reach the initial state.
This might help with visualization as well as provide a useful tool in search of a solution
from both ends, similar to the way Rubik's cube is solved. The proposed algorithms,
if they exist, are symmetrical, so either solution counts.

## Proof
I am yet to prove whether this puzzle has a solution or not, my current approach is to
assemble a set of "algorithms" (not dissimilar to Rubik's cube algorithms),
map their inputs and outputs, and then construct arbitrary combinations to see whether
swapping 2 arbitrary nodes is possible.

While swapping 2 arbitrary nodes might not be strictly required to solve the puzzle,
it is a simple target to assses the completion of, and were it to be possible,
any configuration is reachable, including the solution. That being said,
even if there is no algorithm to swap 2 arbitrary nodes, the puzzle may still be solvable.
Furthermore, were the swapping of 2 arbitrary nodes possible, find a solution to a random
configuration of the puzzle would open up the replayability of the puzzle to near infinity.

It is also good to keep in mind that the theoretical realm of the puzzle is an infinitely
repeating tiled plane, meaning an algorithm found for row or column of the puzzle should
translate to any other row or column respectively.

Even if the puzzle isn't solvable in the current form, finding proof will shed light onto
how to generate seemingly random scrambles that belong to the subset of scrambles that
are solvable.

Current predicitons:
- No arbitrary node swapping for even-sized matrices.
- Solvability dependent on arbitrary node swapping.

## Name
It's a play on the word "torus" and the name "Rubik", because that's how I vizualize the
puzzle in finite 3D space. That vizualization might not be entirely accurate, but it's
good enough for picking a name.

### [`setup.ps1`](setup.ps1)
This script installs `wasm-bindgen-cli` (version 0.2.84), `basic-http-server`
and adds `wasm32-unknown-unknown` to possible compilation targets.
Note that this version of `wasm-bindgen-cli` is required for the pipeline
defined in this repository.

On linux, this script also installs `libssl-dev` and `libasound2-dev`,
which are required for `basic-http-server` and `macroquad` to run respectively.

(This is only necessary to run once on a single computer as the effects
of this script are global.)

### [`build.ps1`](build.ps1)
This script builds the project for the `wasm32-unknown-unknown` target in
`--release` mode, generates WASM bindings, and patches the generated JavaScript
file. It also moves the relevant files to their appropriate directories
in preparation for running the project on a local server or on GitHub Pages.

### [`run.ps1`](run.ps1)
This script hosts the built project on a local `basic-http-server`
server and opens a browser at its location.

(One does not need to restart the server after building the project again,
reloading the webpage in the browser is sufficent.)

(This is necessary over just opening the [`index.html`](index.html)
file in your browser so that the required resources load properly.)

## Sources:
### JS patching scripts:
- https://gist.github.com/tgolsson/d78f7887a8542f3fd6f125070e5e22d6
- https://gist.github.com/nobbele/0d932a993786ed081632254fb6b01e25
- https://gist.github.com/olefasting/15ae263da4cf1ba308ce55c15c9b221b

## License & Contribution

I haven't figured out the licensing for this properly yet. The main point is this:
This is to be open-source, freely accessible,
but NOT to be used commercially by anyone other than me - the author.
That's to say, on the off-chance this puzzle actually makes sense and takes off,
I don't want to end it up like Tetris or the Rubik's cube.
I want to make money off of it (greedy, I know) and not get scammed like Pajitnov did.
I also don't want to limit the community were it to form, like TTC does.
And oh god I'll be smad if this thing already exists..
