# Milestone tracking

How usable is each map overall? Different milestones (roughly in order):

- doesn't crash, but gets permanently stuck in gridlock
- full PSRC day finishes without gridlock
  - with freeform policy is usually easier, since traffic signal defaults are
    quite bad
- full day finishes without any aborted trips
  - enough parking spaces exist
  - parked cars are available when trips need them
  - no disconnected paths
- no geometry problems; map looks reasonble and represents reality

The different map slices are pretty arbitrary, but roughly get bigger down the
list.

## montlake

- full PSRC day finishes with traffic signals
- ~800 aborted trips, mostly from incomplete 520 fixes
  - adding the eastbound onramp breaks the traffic signal

## 23rd

- full PSRC day doesn't gridlock with freeform policy
  - 23rd and madison traffic signal is the biggest problem
- lots of unfinished trips, pedestrians waiting for missing buses
- sometimes crashes with queue spillover?!

## caphill

- crashes, l4720 isn't near a sidewalk

## downtown

- not enough room to even seed parked cars!

## lightrail

- something crashed when instantiating

## ballard

- crashes, l11123 isn't near a sidewalk
- super weird lane geometry near 26th and gilman
	- redundant points in a straight line

## huge_seattle

- crashes, l77611 isn't near a sidewalk