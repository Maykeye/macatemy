from dataclasses import dataclass, asdict, field
import random


@dataclass
class Roll:
    dice_num: int
    side_num: int
    drop_best: int = 0
    add: int = 0

    def __post_init__(self):
        assert self.drop_best < self.dice_num

    def roll(self, apply_trait_bonus=False) -> int:
        values = [random.randint(1, self.side_num) for _ in range(self.dice_num)]
        if self.drop_best:
            values.sort()
            values = values[: -self.drop_best]
        bonus = self.add
        return sum(values) + bonus

    def __call__(self):
        return self.roll()


ATTR_GEN = Roll(dice_num=8, side_num=10, drop_best=4)
CHAOS = Roll(dice_num=8, side_num=10, drop_best=4)

GEN_MIN = 4
GEN_MAX = 36
SECONDARY_GEN_DIV = 3


def clamp(x, vmin, vmax):
    x = max(x, vmin)
    x = min(x, vmax)
    return x


@dataclass
class Attribute:
    name: str
    value: int = 0

    def __post_init__(self):
        if self.value == 0:
            self.value = ATTR_GEN.roll()

    def init_secondary(self, ceiling: "Attribute", floor: "Attribute"):
        base = ceiling.value
        N = SECONDARY_GEN_DIV
        bonus_roll = Roll(1, N + 1, add=-(N + 2))
        bonus = bonus_roll()
        self.value = base + bonus
        self.clamp(GEN_MIN, GEN_MAX)

    def clamp(self, vmin, vmax):
        self.value = clamp(self.value, vmin, vmax)

    def roll(self, level=1):
        return sum(random.randint(1, self.value) for _ in range(level))


@dataclass
class PrimaryAttributes:
    strength: Attribute = field(default_factory=lambda: Attribute("Strength"))
    intellect: Attribute = field(default_factory=lambda: Attribute("Intellect"))
    luck: Attribute = field(default_factory=lambda: Attribute("Luck"))
    agility: Attribute = field(default_factory=lambda: Attribute("Agility"))
    magic: Attribute = field(default_factory=lambda: Attribute("Magic"))
    charm: Attribute = field(default_factory=lambda: Attribute("Charm"))

    def __getitem__(self, idx: int):
        return [
            self.strength,
            self.intellect,
            self.luck,
            self.agility,
            self.magic,
            self.charm,
        ][idx]

    def __len__(self):
        return len(asdict(self))

    def __str__(self):
        s = "PrimaryAttributes("
        s += f"strength={self.strength.value}, "
        s += f"agility={self.agility.value}, "
        s += f"intellect={self.intellect.value}, "
        s += f"luck={self.luck.value}, "
        s += f"magic={self.magic.value}, "
        s += f"charm={self.charm.value})"
        return s


@dataclass
class SecondaryAttributes:
    constitution: Attribute = field(default_factory=lambda: Attribute("Constitution"))
    agility: Attribute = field(default_factory=lambda: Attribute("Agility"))


class Trait:
    LUCKY = "Lucky"
    """Rolls of `luck` that didn't end in nat-1 gain `+ROLL_BONUS`"""

    ROLL_BONUS = 3


class Cat:
    def __init__(self) -> None:
        self.prim_attr = PrimaryAttributes()
        self.sec_attr = SecondaryAttributes()
        self.traits: list[str] = []
        self.init_primary()
        self.init_secondary()

    def init_primary(self):
        UPTICK = 2
        DOWNTICK = 2
        attributes_to_change = list(range(len(self.prim_attr)))
        random.shuffle(attributes_to_change)
        attributes_to_change = attributes_to_change

        luck_rolls = [
            self.prim_attr.luck.roll(Trait.LUCKY in self.traits) for _ in range(UPTICK)
        ]

        # Reroll bad attribute
        for _ in range(UPTICK):
            attr_idx = attributes_to_change.pop()
            attr = self.prim_attr[attr_idx]
            if luck_rolls.pop() > attr.roll():
                attr.value = max(attr.value, ATTR_GEN.roll())

        # Reroll good attribute
        for _ in range(DOWNTICK):
            attr_idx = attributes_to_change.pop()
            attr = self.prim_attr[attr_idx]
            if CHAOS.roll() > attr.roll(Trait.LUCKY in self.traits):
                attr.value = min(attr.value, ATTR_GEN.roll())

        # Clamp
        for attr in self.prim_attr:
            attr.clamp(GEN_MIN, GEN_MAX)

    def init_secondary(self):
        self.sec_attr.constitution.init_secondary(
            self.prim_attr.strength, self.prim_attr.agility
        )
        self.sec_attr.agility.init_secondary(
            self.prim_attr.agility, self.prim_attr.strength
        )

    def __str__(self) -> str:
        return f"Cat({self.prim_attr},\n  {self.sec_attr})"


cat = Cat()
print(cat)
