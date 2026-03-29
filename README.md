# Dreams

This is a 2D Singleplayer Top-Down Procedural Survival RPG, where the player explores an open, procedurally generated world, engaging in real-time combat (in the style of Zelda), gathering resources, completing quests, and interacting with NPCs. Each in-game “day” lasts approximately 30 minutes and ends at Midnight, where the entire world resets with a new procedural seed.

The world is a fractured mind attempting to reconstruct reality. Each reset is the brain trying to “solve” itself. The player, unknowingly acting as a disruptive force, causes this process to fail every cycle.

Player progression persists across loops (gear, stats, items), while the world, NPC placements, and terrain regenerate. Key structures and story elements are guaranteed to reappear in different forms each cycle.

Each cycle encourages exploration, resource gathering, quest completion, and character strengthening under the belief that the player is progressing toward saving the world.

## TODO 
Below is my ToDo list for this project, there's still many other things that could be improved toward a better experience overall but these are my top priorities.

### MAP 
#### Map bitmasking sometimes fails
(grass example)

tr

mc tr

mc tc tr

mc mc mc tr

the tc above should have been a mc

#### Map water-grass connection
right now when water is beside grass both generates the border, grass should instead generate grass_mc and connect with water_grass.

#### Map doesn't despwn entities
map doesn't despawn entities that out of render distance

#### Tree sprite getting cut off
the top of the big trees doesn't generate and is cut off
the top and right of small tree doesn't generate and is cut off

#### Boulders and Large Bushes are spawning together as a bundle
the bottom two boulder sprites and the top two large bush sprite are being spawned togther as a bundle for some reason

#### Tree placement errors
tree in water, tree too close (on top of each other), in very small patches

### Player 
#### Player jump is animation only
when the player jump w/ space bar it's a animation that plays but nothing really happens

#### Player z-sorting logic is not working
the z-sorting for the player is not work, most likely the map decors are loading on the wrong z-layer

### Other 
#### NPC & Pathfinding
both folders are place holders, both empty
