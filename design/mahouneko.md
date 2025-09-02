FILE FORMAT: Markdown

# Magical ACatemy

This design document describes implementation roadmap for making a sims/dwarf-fortress inspired game about magical cats.
It's built from bottom to top for easisess of implementation.

General information about the game:

- It'll be made in bevy (rust engine)
- For the easiness of test, in test build random should be as deterministic as it can despite bevy parallelization.
- The goal is to make a fun and interesting game about resource management and levelling up cats
- Kittens come and go: "general education" may last no more than N seasons.
- Some kittens can stay for "specific education" for more than that, they can also became teachers
- The player starts with one cat (let's call it the founding mother)

## General attributes.

Every cat(cat and kitten alike) have the attributes, that are separated into priamry and secondary.

Primary attributes are attributes on their own. Secondary attributes are based upon primary attributes and other secondary attributes and unlike given number, kittens are born with bonuses and penalties to them. "Based means" that secondary attributes are calculated from "main" and "aux" attributes.

"Main" attribute effects secondary attribute more than "aux". For example `constitution` and `speed` are bohth based on `strength` and `agility`, but for `constitution` main attribute is `strength` and for `speed` main attribute is `agility`, so even if both attributes are based on the same primary attributes, even discarding bonuses, we have different attribute values.

### Primary attributes

- Strength - how much weights can cat carry
- Intelligence - how smart kitten is
- Luck - how lucky cat is
- Agility - how dexterous cat is or how ditzy
- Magic - affection with general magic
- Charm - Personality

### Secondary attributes

Secondary attribute are based upon other attributes.

- Constitution (Based on strength, agility) - affects maximum health and its regeneration
- Speed (Based on agility, Strength) - affects movement speed
- Perception (Based on intelligence, Luck) - affects how much cat can see
- Melee combat (Based on strength, agility) - affects melee attacks
- Ranged combat (Based on agility, luck) - affects ranged attacks
- Magic combat (Based on magic, luck)
- Willpower (Based on intelligence, charm) - affects mental resistance

### Secondary attributes related to school of magic.

There are several schools of magic:

- elemental (fire, water, wind, ground),
- medical(transmutation(mainly others), healing, necromancy)
- alchemy(potion making, chemistry, enchantment of tools and books)
- evocation (summoning, control, transformation(primary self), illusion)
- divination (astral projection, prophecy, prediction, divine shielding)

School also have base attributes:

- Elemental: base: magic
- Medical: base: intelligence, aux: perception
- Alchemy: base: intelligence, aux: willpower
- Evocation: base: magic, aux: charm
- Divination: base: charm, aux: luck

## Rolls

- Basic rolls are written as `3d6` which means "roll 3 die, each has 6 sides"
- Bonuses can be added/subtracted on top of the roll, e.g. `3d6+5` means "roll 3 die, each has 6 sides, add 5
- Discarding maximum values. Consider `3d6(drop 1 high)` means discard one highest roll. E.g. if rolls were 3,4,5, then 5 will be discarded and 3+4 will be chose. Alternatively `4d5(drop 2 low)` means discarding 2 minimal values. E.g. if 1,2,2,4 were rolled, use 2+4 as result
- Using attribute state. Usual attribute roll is written as `1d{strength}` which means to roll a die where number of sides is equal to attribute luck 
- Attribute roll. E.g. `1d{luck}+{lucky}` is something which is called `luck roll` : it uses attribute of luck + its modifier from trait `lucky`.
- `Default` roll is `8d10(drop 4 high)`, goes from 4 to 40 with average around 13.
- `Chaos` roll is a special roll representing chaos and despair. Default value is `Default` roll
- When two rolls compete against each other, a roll `A` wins against roll `B` if the result of roll `A` is strictly greater than the result of roll `B`. Eg if roll `A` rolled 3, roll `B` rolled 3, this is a tie and no roll won or lose. If role `A` rolled 4 and roll `B` rolled 3, roll `A` wins.

## Cat primary stats generation

Each primary attribute is initially assigned a value using the standard `Default` roll of `8d10(drop 4 high)`, with an average value around 13.
After rolling initial values for the attributes some are upgraded and some are downgraded.

### Upgrades

- Two unique primary attributes are randomly selected to be upgraded:
  - Each of these attributes are rolled against `luck roll` (eg `1d{strength} vs 1d{luck}`).
  - If the `luck roll` wins (its value is strictly greater)
    - `Default` roll is used to generate a new potential value for the attribute.
    - If newly generated value is greater than existed value,
      - the attribute value is assigned to it
    - otherwise(new value ≤ existing value) nothing changes and the attribute value stays the same
  - if `luck roll` doesn't win (tie or loses), nothing changes and the attribute value stays the same
  - **NOTE**: during reroll the `luck` attribute may be changed on the first upgrade roll. If it happens, upgrade of the second attribute will be using the original luck value.

### Downgrades

- Other two unique primary attributes are randomly selected to be downgraded:
  - Each of these attributes are rolled against `chaos` roll
  - If chaos wins (its value is strictly greater)
    - `Default` roll is used to generate a new potential value for the attribute.
    - If newly generated value is less than existing value,
      - the attribute value is assigned to it
    - otherwise(new value ≥ existing value) nothing changes and the attribute value stays the same
  - if chaos doesn't win (tie or loses), nothing changes and the attribute value stays the same
- The values of remaining primary attributes are left untouched (i.e. they are `default` roll).

## Cat secondary stats generation

- Constitution (Ceiling: strength, floor: agility) - affects maximum health and its regeneration
- Speed (Ceiling: agility, floor: Strength) - affects movement speed
- Perception (Ceiling: intelligence, floor: Luck) - affects how much cat can see
- Melee combat (Ceiling: strength, floor: agility) - affects melee attacks
- Ranged combat (Ceiling: agility, floor: luck) - affects ranged attacks
- Magic combat (Ceiling: magic, floor: luck)
- Willpower (Ceiling: intelligence, floor: charm) - affects mental resistance

Their generation while begins with `Default` roll, must compete against ceiling and floor attributes.

- Once attribute value is generated, its roll is compared against ceiling attribute roll. 
- If ceiling attribute roll loses to generated attribute roll (i.e. it's strictly less)
    - the generation is considered invalid, and roll happens again.
- Otherwise: new generated attribute roll competes against floor attribute.
- If floor attribute roll wins to generated attribute roll (i.e. it's strictly greater)
    - the generation is considered invalid, and roll happens again.

The game tries to roll this way several(16) times, if rolls doesn't succeed, last generated value is used.

## Traits

Each cat is spawn with random number of traits.

The traits are:

- Attribute based strength. They can provide bonuses(default +3) and penalties(default -3), no cats can be generated with the contradicting traits(later they can earn them, eg `mighty` cat can get sick and become `weakly` which negates the mighty)
    - Mighty (+3 strength) or weakly (-3 strength)
    - Smarty (+3 intelligence) or dumby (-3 intelligence)
    - Lucky (+3 luck) or unlucky(-3 luck)
    - Swifty (+3 agility) or clumsy (-3 agility)
    - Wizardly (+3 magic), dully (-3 magic)
    - Pretty (+3 charm) or scruffy (-3 charm)

## Generic Implementation roadmap

- [design] write this design document
- [world.gen] generate a world
- [world.gen.empty] generate an empty world 10x10
  provide: [world.gen]
- [world.gen.empty.dummy] add 1x1 cube at cell (4,4,0) and (5,5,1)
- [world.control] implement moving camera around
deps: `bevy.input` `world.control.wasd`,`world.control.qe`, `world.control.mouse.edge`, `world.control.mouse.zoom`
- [world.control.wasd] implement control camera using `w`, `a`, `s`, `d` to move around axis,
- [world.control.qe] implement control camera using `q`, `e` to move camera up/down by level
- [world.control.mouse.edge] implement control camera using mouse on edge
- [world.control.mouse.zoom] implement control mouse wheel to "zoom" in/out
    - move camera closer/further to surface
-[bevy.input] implement a system for user input and tests
deps: [bevy.input.fake], [bevy.input.real], [bevy.cmd.quit]
    - [bevy.input.real] route keyboard to inputs
    - [bevy.input.fake] route presses from files
    - [bevy.cmd.quit] quit on pressing 'escape`
- [roll] generate a system of rolls using provided RNG
- [cat.gen] generate a cat
  - deps: [world.gen], [roll] 

### Today step: up to [bevy.input]
It means
- [bevy.input.real] route keyboard to inputs
- [bevy.input.fake] route presses from files
- [bevy.cmd.quit] quit on pressing 'escape`
