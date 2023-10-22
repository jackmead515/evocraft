# EVOCraft

EvoCraft is a 2D overworld game where you explore and survive in a procedurally generated world with creatures that evolve and adapt over time.

You are thrown onto an island where you must survive in the same environment as creatures that
may seem harmless at first, but will evolve and adapt to your actions. If you are an easy prey,
they may learn to hunt you. If you are a threat, they may learn to avoid you. If you are a friend, they may learn to trust you.

## Engine

Utilizing macroquad as the source of rendering, EvoCraft is a custom game engine built in Rust. For learning, I try to implement as many custom algorithms as I can.

- Creatures takes using many neural networks in a transfer learning fashion.

- A genetic algorithm is used to evolve the features of the creatures and showcase their genetic diversity.

- A custom pathfinding algorithm is used to find the shortest path between two points.

- Projections are used everywhere to convert between different coordinate systems.

- Shadow casting is used to display fog of war.

- And more over time!