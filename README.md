## About

A cli tool which can quickly show current year F1 schedule, upcoming Grand Prix, team & driver standings

## Usage

Run `f1gp pull` before trying any of the available options

**Options**

`list`: Shows all Grand Prix races for current calendar year

```
...
[x] Australian Grand Prix / Melbourne
[>] Japanese Grand Prix / Suzuka
[ ] Chinese Grand Prix / Shanghai
...

x -> GP weekend completed
> -> GP weekend in progress
  -> GP weekend in future
```

`next`: Shows session schedule of next Grand Prix

```
+----------------------------------------+
|      Japanese Grand Prix / Suzuka      |
+----------------------------------------+
| [x] FP 1 : Fri 05/04/2024 08:00        |
| [x] FP 2 : Fri 05/04/2024 11:30        |
| [ ] FP 2 : Sat 06/04/2024 08:00        |
| [ ] Quali: Sat 06/04/2024 11:30        |
| [ ] Race : Sun 07/04/2024 10:30        |
+----------------------------------------+

[x] -> Session completed
[ ] -> Session pending

```

`next <#>`: Shows session schedule for next #num of Grand Prix Races

`drivers`: Shows current driver standings

`teams`: Shows current team/constructor standings

`pull`: Pull latest data from internet sources. Data from all these sources is fetched once and cached for subsequent commands. Do a fresh pull if data needs to be updated. There are 3 sources currently used

- https://www.formula1.com/en/results.html/2024/drivers.html
- https://www.formula1.com/en/results.html/2024/team.html
- https://raw.githubusercontent.com/sportstimes/f1/main/_db/f1/2024.json

`help`: Shows all possible commands

## Build

- requirements: rustc, cargo(you can have both by installing rustup), neovim
- clone the repo and `cd` into it
- run `cargo build --release --target_dir="somewhere/in/path"` to build and use binary
