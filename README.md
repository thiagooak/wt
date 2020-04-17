# wt
Writing Tools. A command-line style checker for technical writers.

## Usage

```
wt example.txt

More than 10% of your sentences have > 18 words
    Vivamus lacinia, libero ac lobortis iaculis, dui dui malesuada diam, e...
```

example.txt
```
Vivamus lacinia, libero ac lobortis iaculis, dui dui malesuada diam, eu interdum tellus risus nec leo class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos.

Pellentesque vel elit maximus, suscipit turpis a, elementum turpis.

Vivamus sollicitudin arcu sit amet elementum fermentum.
Vestibulum sed velit in dolor molestie congue.

Vestibulum dui quam, pharetra non egestas id, ullamcorper et mauris.

Vestibulum blandit felis quis ligula finibus commodo.
```

## Installation

Download the latest [MacOS binary](https://github.com/thiagooak/wt/releases).

e.g.

```
curl -fsSLO https://github.com/thiagooak/wt/releases/download/v0.1.1/wt

mv wt /usr/local/bin && chmod +x /usr/local/bin/wt

wt --help
```

or build it
```
git clone git@github.com:thiagooak/wt.git
cd wt

cargo build --release

./target/release/wt --help
```
