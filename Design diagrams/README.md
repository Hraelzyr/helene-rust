### Design Diagrams
This folder has the details of the intended 
structure of the software.

#### Sequence Diagrams
They illustrate the fashion in which the software
will be expected to operate.  
They are located in 'Sequence Diagrams'.

#### Class Diagrams
They provide the members of each class/interface.  
None have been made yet.

#### Object Diagrams
They explain how the classes interact with each other.  
None have been made yet.

### Arguments for extensibility

Renderer is plugged in: 
Any renderer that takes the locations of people and walls
and blits it out is fine.

The Action Queue handles all the processing of actions.
New actions can simply be implemented by changing the code
in that file and adding some subscribers in its constructor.

Each context has its own handler, and so:
1. If a handler wants to expand the range of its commands,
that is up to that handler.
2. If a new handler needs to be added, we simply need to
add the keys that transfer control to and from it.
3. The reverse removes a handler (never giving control)

The Map stores both monsters on a level and the map of that
level, which is sufficient to store and load a level.

Flags in the MainLoop suffice to give it its control signals.
Adding modifiers to those flags is sufficient to allow contexts
to direct control flow.