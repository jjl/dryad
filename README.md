# dryad

The flexibility of paper diagramming without the paper.

## Status: Pre-alpha

This is a project for fun, that I hack on in my spare time. Do not
expect to get any good use out of it soon.

You can draw things. They disappear as soon as you let go of the mouse
button.

Right now we're focusing on getting basic vector drawing working and
figuring out an architecture that works.

## Goals

* Invent new kinds of diagrams without the bullshit.

Long term:

* 3D modelling
* Fluid flow simulation

## Code introduction

Dryad is using [druid](https://github.com/linebender/druid) as the GUI
framework. Since it's pre-1.0, there are things messing and a bunch of
stuff is awkward to do.

Everything that is capable of interaction is a widget.

## Copyright and License

Copyright (c) 2020 James Laver, Dryad contributors.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public
License along with this program.  If not, see <https://www.gnu.org/licenses/>.
