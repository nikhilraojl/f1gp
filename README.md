## About

Cli tool which quickly shows current year F1 schedule, upcoming Grand Prix, team & driver standings

Get all Grand Prix races by using `f1gp list`:\
**NOTE: Run `f1gp pull` before trying any of the available commands**

```
...
[x]   3. Australian Grand Prix / Melbourne
[->   4. Japanese Grand Prix / Suzuka
[ ]   5. Chinese Grand Prix / Shanghai
[ ]   6. Miami Grand Prix / Miami
[ ]   7. Emilia Romagna Grand Prix Grand Prix / Imola
[ ]   8. Monaco Grand Prix / Monte Carlo
...
```

Or get the next Grand Prix schedule using `f1gp next`:

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
Next session in: 10 days, 10 hours, 18 minutes
```

There are a few more commands & options(see below), try them out

## Usage

**Options**

`help`: Shows all possible commands

`list`: Shows all Grand Prix races for current calendar year. The list will be in the layout\
`[status] #round-number GrandPrix-name / location`. Status symbols are explained below

```
[x] -> GP weekend completed
[-> -> GP weekend in current week
[ ] -> GP weekend in future
```

`next`: Shows session schedule of next Grand Prix. Also shows time until next session
Status symbols in output are explained below

```
[x] -> Session completed
[ ] -> Session pending
```

`next <#>`: Shows session schedule for next #num of Grand Prix Races

`drivers`: Shows current driver standings

`teams`: Shows current team/constructor standings

`result`: Shows last Grand Prix race result

`result <#>`: Shows results of the requested Grand Prix race (#round)

`quali`: Shows last Grand Prix qualifying

`quali <#>`: Shows qualifying results of the requested Grand Prix race (#round)

_NOTE: `0` race or quali position for driver indicates either DNF or DNS or DQ_

`pull`: Pull latest data from sources. Data from all these sources is fetched once and cached for subsequent commands. Do a fresh `f1gp pull` if any data needs to be updated. Below are the sources currently used

- https://www.formula1.com/en/results.html/2024/drivers.html
- https://www.formula1.com/en/results.html/2024/team.html
- https://www.formula1.com/en/results.html/2024/races.html
- https://raw.githubusercontent.com/sportstimes/f1/main/_db/f1/2024.json

`pull`: Pull latest data from sources. Data from all these sources is fetched once and cached for subsequent commands. Do a fresh `f1gp pull` if any data needs to be updated. Below are the sources currently used

`clean`: Removes all cached files. Helpful to clean any invalid cache
 - `--dry-run`: shows files which will be deleted
## Build

- requirements: rustc, cargo(you can have both by installing rustup), neovim
- clone the repo and `cd` into it
- run `cargo build --release --target_dir="somewhere/in/path"` to build and use binary
