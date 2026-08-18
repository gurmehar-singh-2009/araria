information for me:

roblox constructs its shit using "parts"/"studs"

game wise:
- studs are given an AABB and when they update roblox sort of "merges" them in nice ways to reduce checks

net wise:
- since shit is "linked" (studs are linked together to form "assemblies" or some shit) you associate an id with an assembly and you just send a transformation matrix.

water/liquids:
- roblox splits the map into 4x4x4 studs and has a bitflag to say if its occupied or not
- all the fancy animations and wave movements are client side, even the shit where its like sort of on land

scripting:
- lwk i hate lua so likeeee
- experimenting with a new embedded lang called "Rhai": https://github.com/rhaiscript/rhai
- technically lua is better cus it compiles to bytecode and uses the jit compiler, but its not gonna make or break the game yet
- later i can always change my mind
