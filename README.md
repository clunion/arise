# A RaInmeter Skin Evolver

![arise](https://raw.githubusercontent.com/clunion/arise/main/resources/arise-title.png)

![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)

## What is this?

***A*** ***R***a***I***nmeter ***S***kin ***E***volver  
Arise is a simple tool to evolve (generate) [Rainmeter](https://www.rainmeter.net/) skins (.ini-files) from template files (.arise-files).

## What for?

Some not so short time ago i wrote a skin for Rainmeter, which could monitor the physical hard drives of a Microsoft Windows machine.
It shows the existence of a drive, its free and used space, and the current reads and writes in a histogram meter-display.  
A click on the graphic opens the windows filesystem explorer on that drive.

This static, hand-written Rainmeter-Skin works reasonably well, but not perfect:  
Since a Rainmeter skin requires correct coordinates for all possible drive-meters, an error in the manual computations leads to wrong offsets and thus
non-matching visuals. And every change affecting the layout requires the computation of the offsets for all 24 drives. Again and again.  

To solve that, a generator able to compute all the coordinates would be needed. That is what 'arise' is for.

And since this is a rather small and simple application, it may serve as a showcase for setting up a nearly minimal rust project.

## Current state

Most things of the development environment like  
Git and a repository on Github are set up.  
Added clap and flexi_logger recently.

## Maintainer of arise

* [@clunion](https://github.com/clunion)

## Communicate

(currently, the discord server of Shardoverse may be used.)

* [Discord-Server Shardoverse](https://discord.gg/PWAtRU)
